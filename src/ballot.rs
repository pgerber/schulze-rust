//! Ballots

use rank::{SimpleRank, Rank};

/// A ballot
pub struct Ballot<R = SimpleRank> {
    name: Option<String>,
    ranks: Vec<R>,
}

impl<R> Ballot<R> {
    /// Create a new ballot for N `candidates`.
    ///
    /// Optionally, a `name` can be used to identify the ballot (i.e. the
    /// voters name).
    pub(crate) fn new(candidates: usize) -> Self
    where
        R: Rank,
    {
        Ballot {
            name: None,
            ranks: vec![Default::default(); candidates],
        }
    }

    /// Set a name for the ballot (i.e. the voters name).
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Paul");
    /// # let mut election = nomination.build();
    /// # let mut ballot = election.new_ballot();
    /// ballot.set_name("Ivy O'Neill");
    /// assert_eq!(ballot.name(), Some("Ivy O'Neill"));
    /// ```
    pub fn set_name<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        self.name = Some(name.to_string());
        self
    }

    /// Unset the name of the ballot.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Paul");
    /// # let mut election = nomination.build();
    /// # let mut ballot = election.new_ballot();
    /// # ballot.set_name("Ivy O'Neill");
    /// assert_eq!(ballot.name(), Some("Ivy O'Neill"));
    /// ballot.unset_name();
    /// assert_eq!(ballot.name(), None);
    /// ```
    pub fn unset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }

    /// Retrieve the name of the ballot.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Paul");
    /// # let mut election = nomination.build();
    /// # let mut ballot = election.new_ballot();
    /// ballot.set_name("Ivy O'Neill");
    /// assert_eq!(ballot.name(), Some("Ivy O'Neill"));
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    /// Rank candidate with `id`.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut n = Nomination::new();
    /// # n.nominate("Paul");
    /// # let mut e = n.build();
    /// # let mut ballot = e.new_ballot();
    /// #
    /// // set rank 5 on candidate 0
    /// ballot.rank(0, 5);
    /// ```
    pub fn rank<T>(&mut self, id: usize, rank: T) -> &mut Self
    where
        R: Rank,
        T: Into<R>,
    {
        self.ranks[id] = rank.into();
        self
    }

    /// Set ranks for all candidates
    ///
    /// # Panics
    ///
    /// Panics if `ranks` doesn't yield exactly one `Rank` per candidate.
    ///
    /// # Example
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Joe");
    /// # nomination.nominate("Zoe");
    /// # nomination.nominate("Ivy");
    /// # let mut election = nomination.build();
    /// # let ballot = election.new_ballot();
    /// // rank all three candidates at once
    /// ballot.rank_all(&[4, 7, 3]);
    ///
    /// assert!(ballot.get_rank(0) == &4.into());
    /// assert!(ballot.get_rank(1) == &7.into());
    /// assert!(ballot.get_rank(2) == &3.into());
    /// ```
    pub fn rank_all<T, I>(&mut self, ranks: T) -> &mut Self
    where
        R: Rank,
        T: IntoIterator<Item = I>,
        I: Into<R>,
    {
        let len = self.ranks.len();
        let mut src_iter = ranks.into_iter();
        let processed = self.ranks
            .iter_mut()
            .zip(src_iter.by_ref().take(len))
            .map(|(src, dest)| { *src = dest.into(); })
            .count();

        assert!(
            self.ranks.len() == processed && src_iter.next().is_none(),
            "number of ranks must match number of candidates exactly"
        );
        self
    }

    /// Get rank for candidate with `id`.
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Paul");
    /// # let mut election = nomination.build();
    /// # let mut ballot = election.new_ballot();
    /// #
    /// // set rank 5 on candidate 0
    /// ballot.rank(0, 5);
    ///
    /// // get rank of candidate 0
    /// assert!(ballot.get_rank(0) == &5.into());
    /// ```
    pub fn get_rank(&self, id: usize) -> &R
    where
        R: Rank,
    {
        &self.ranks[id]
    }

    /// Retrieve ranks for all candidates.
    ///
    /// Candidate with id 0 is `ranks()[0]`, with id 1 `ranks()[1]`, …:
    ///
    /// ```
    /// # use schulze::Nomination;
    /// #
    /// # let mut nomination = Nomination::new();
    /// # nomination.nominate("Joe");
    /// # nomination.nominate("Zoe");
    /// # nomination.nominate("Ivy");
    /// # let mut election = nomination.build();
    /// # let mut ballot = election.new_ballot();
    /// #
    /// ballot
    ///     .rank(0, 5)
    ///     .rank(1, 2)
    ///     .rank(2, None);
    ///
    /// assert!(ballot.ranks() == &[5.into(), 2.into(), None.into()]);
    /// ```
    pub fn ranks(&self) -> &[R]
    where
        R: Rank,
    {
        &self.ranks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use election::Election;
    use nomination::Nomination;

    #[test]
    fn ballots() {
        let mut election = create_election();
        election.new_ballot().rank(0, 15).rank(1, 25).rank(2, 0);

        election.new_ballot().rank(0, Some(5)).rank(1, None);

        election.new_ballot().rank(0, 0).rank(1, 1).rank(2, 0);

        assert!(election.candidates().iter().map(|c| c.name()).eq(
            &[
                "Peter Gerber".to_string(),
                "Jane Doe".to_string(),
                "Andrew Smith".to_string(),
            ],
        ));

        let shall = &[
            vec![Some(15), Some(25), Some(0)],
            vec![Some(5), None, None],
            vec![Some(0), Some(1), Some(0)],
        ];
        let is: Vec<_> = election
            .ballots()
            .iter()
            .map(|b| {
                b.ranks().iter().map(|v| v.get_rank()).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(&shall, &is.as_slice());
    }

    #[test]
    #[should_panic(expected = "number of ranks must match number of candidates exactly")]
    fn rank_all_too_few_ballots() {
        let mut election = create_election();
        election.new_ballot().rank_all(&[1, 2]);
    }

    #[test]
    #[should_panic(expected = "number of ranks must match number of candidates exactly")]
    fn rank_all_too_many_ballots() {
        let mut election = create_election();
        election.new_ballot().rank_all(&[1, 2, 3, 4]);
    }

    fn create_election() -> Election {
        let mut nomination = Nomination::new();
        nomination
            .nominate("Peter Gerber")
            .nominate("Jane Doe")
            .nominate("Andrew Smith");
        nomination.build()
    }
}
