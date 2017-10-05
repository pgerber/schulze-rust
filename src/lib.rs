#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

pub mod ballot;
pub mod nomination;
pub mod paths;
pub mod rank;

use paths::Paths;
use ballot::Ballot;

use std::clone::Clone;
use std::cmp::{max, min};

pub struct Election {
    candidates: Vec<Candidate>,
    ballots: Vec<Ballot>,
}

impl Election {
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    pub fn ballot(&mut self) -> &mut Ballot {
        self.ballots.push(Ballot::new(self.candidates.len(), None));
        self.ballots.last_mut().unwrap()
    }

    pub fn ballot_for<T>(&mut self, name: T) -> &mut Ballot
    where
        T: ToString,
    {
        self.ballots.push(Ballot::new(
            self.candidates.len(),
            Some(name.to_string()),
        ));
        self.ballots.last_mut().unwrap()
    }

    pub fn ballots(&self) -> &[Ballot] {
        &self.ballots
    }

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

    fn find_strongest_paths(&self) -> Paths {
        let mut result = Paths::new(self.candidates.len());

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    let preferring_i = self.preference(i, j);
                    if preferring_i > self.preference(j, i) {
                        *result.mut_path(i, j) = preferring_i;
                    }
                }
            }
        }

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    for k in 0..self.candidates.len() {
                        if i != k && j != k {
                            let j_k = result.path(j, k);
                            let j_i = result.path(j, i);
                            let i_k = result.path(i, k);
                            *result.mut_path(j, k) = max(j_k, min(j_i, i_k));
                        }
                    }
                }
            }
        }

        result
    }

    fn preference(&self, i: usize, j: usize) -> u32 {
        self.ballots
            .iter()
            .filter(|b| b.get_rank(i) > b.get_rank(j))
            .count() as u32
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    name: String,
}

impl Candidate {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct ElectionResult {
    ranked_candidates: Vec<Candidate>,
    paths: Paths,
}

impl ElectionResult {
    pub fn ranked_candidates(&self) -> &[Candidate] {
        &self.ranked_candidates
    }

    pub fn paths(&self) -> &Paths {
        &self.paths
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    extern crate test;

    use super::*;
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
            let ballot = election.ballot();
            for j in 0..nomination_count {
                ballot.rank(j as usize, SimpleRank::from(((i + j) % 50) as u8));
            }
        }

        b.iter(|| { election.find_strongest_paths(); });
    }
}
