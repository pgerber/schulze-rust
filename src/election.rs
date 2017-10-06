//! Election
//!
//! Elections are created By [`Nomination`]s:
//!
//! ```
//! use schulze::election::Election;
//! use schulze::Nomination;
//!
//! let mut nomination = Nomination::new();
//! nomination
//!     .nominate("John")
//!     .nominate("Abby");
//! let _: Election = nomination.build();
//! ```
//!
//! [`Nomination`]: ../nomination/struct.Nomination.html

use ballot::Ballot;
use Candidate;
use paths::Paths;

use std::cmp::{max, min};

/// Election
pub struct Election {
    candidates: Vec<Candidate>,
    ballots: Vec<Ballot>,
}

impl Election {
    /// Create new election
    pub(crate) fn new(candidates: Vec<Candidate>) -> Self {
        Election {
            candidates,
            ballots: Vec::new(),
        }
    }

    /// Get all candidates
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// nomination
    ///     .nominate("May")
    ///     .nominate("June")
    ///     .nominate("Ivy");
    /// let election = nomination.build();
    ///
    /// assert_eq!(
    ///     &election.candidates().iter().map(|c| c.name()).collect::<Vec<_>>(),
    ///     &["May", "June", "Ivy"]
    /// );
    ///
    /// ```
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// Create a new `Ballot`
    pub fn new_ballot(&mut self) -> &mut Ballot {
        self.ballots.push(Ballot::new(self.candidates.len()));
        self.ballots.last_mut().unwrap()
    }

    /// Get all `Ballot`s
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination
    /// #     .nominate("Ivy");
    /// # let mut election = nomination.build();
    /// #
    /// election.new_ballot()
    ///     .set_name("Juliet")
    ///     .rank(0, 5);
    ///
    /// let ballots = election.ballots();
    /// assert_eq!(ballots[0].name(), Some("Juliet"));
    /// assert!(ballots[0].get_rank(0) == &5.into());
    /// ```
    pub fn ballots(&self) -> &[Ballot] {
        &self.ballots
    }

    /// Get result of election
    ///
    /// See [`ElectionResult`] for details.
    ///
    /// [`ElectionResult`]: struct.ElectionResult.html
    pub fn result(&self) -> ElectionResult {
        let paths = self.find_strongest_paths();
        let mut ranking: Vec<_> = (0_usize..self.candidates.len()).collect();
        ranking.sort_unstable_by(|s, o| paths.path(*o, *s).cmp(&paths.path(*s, *o)));
        let ranked_candidates: Vec<_> = ranking
            .iter()
            .map(|i| self.candidates[*i].clone())
            .collect();

        ElectionResult {
            ranked_candidates,
            paths,
        }
    }

    /// Find strongest paths for all candidates
    ///
    /// Search for the strongest paths using the Floyd-Warshall algorithm.
    fn find_strongest_paths(&self) -> Paths {
        let mut paths = Paths::new(self.candidates.len());

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    let preferring_i = self.prefered_by(i, j);
                    if preferring_i > self.prefered_by(j, i) {
                        *paths.mut_path(i, j) = preferring_i;
                    }
                }
            }
        }

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    for k in 0..self.candidates.len() {
                        if i != k && j != k {
                            let j_k = paths.path(j, k);
                            let j_i = paths.path(j, i);
                            let i_k = paths.path(i, k);
                            *paths.mut_path(j, k) = max(j_k, min(j_i, i_k));
                        }
                    }
                }
            }
        }

        paths
    }

    /// Number of voters that prefer candidate `i` over `j`.
    fn prefered_by(&self, i: usize, j: usize) -> u32 {
        self.ballots
            .iter()
            .filter(|b| b.get_rank(i) > b.get_rank(j))
            .count() as u32
    }
}

/// Result of an `Election`
pub struct ElectionResult {
    ranked_candidates: Vec<Candidate>,
    paths: Paths,
}

impl ElectionResult {
    /// Candidates ranked according to the Schulze method.
    ///
    /// Cadidates are sorted by rank. Starting with the winner.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// # let mut nomination = Nomination::new();
    /// # nomination
    /// #     .nominate("Jenny")
    /// #     .nominate("Wilma")
    /// #     .nominate("Donald");
    /// # let mut election = nomination.build();
    /// # election.new_ballot()
    /// #     .rank_all(&[Some(1), Some(0), None]);
    /// let result = election.result();
    /// assert_eq!(
    ///     &result.ranked_candidates().iter().map(|c| c.name()).collect::<Vec<_>>(),
    ///     &[
    ///         "Wilma", // ranked 1st
    ///         "Jenny", // ranked 2nd
    ///         "Donald" // ranked 3rd
    ///     ]
    /// );
    /// ```
    pub fn ranked_candidates(&self) -> &[Candidate] {
        &self.ranked_candidates
    }

    /// Get strongest paths between all candidates.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// # let mut nomination = Nomination::new();
    /// # nomination
    /// #     .nominate("Jenny")
    /// #     .nominate("Wilma");
    /// # let mut election = nomination.build();
    /// election.new_ballot().rank_all(&[1, 0]);
    /// election.new_ballot().rank_all(&[0, 1]);
    /// election.new_ballot().rank_all(&[1, 0]);
    ///
    /// let result = election.result();
    /// assert_eq!(
    ///     result.paths().iter().collect::<Vec<_>>(),
    ///     &[
    ///         (0, 1, 0), // candidate 0 loses against 1
    ///         (1, 0, 2), // candidate 0 wins against 0 (two people prefer candidate 1)
    ///     ]
    /// );
    /// ```
    pub fn paths(&self) -> &Paths {
        &self.paths
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    extern crate test;

    use rank::SimpleRank;
    use nomination::Nomination;

    #[bench]
    #[cfg(feature = "unstable")]
    fn bench_find_strongest_paths(b: &mut test::Bencher) {
        let nomination_count = 50;
        let ballots_count = 1_000;
        let mut nomination = Nomination::new();
        for i in 0..nomination_count {
            nomination.nominate(format!("{}", i));
        }
        let mut election = nomination.build();

        for i in 0..ballots_count {
            let ballot = election.new_ballot();
            for j in 0..nomination_count {
                ballot.rank(j as usize, SimpleRank::from(((i + j) % 50) as u8));
            }
        }

        b.iter(|| { election.find_strongest_paths(); });
    }
}
