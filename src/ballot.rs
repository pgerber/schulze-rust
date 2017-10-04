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
    pub fn new(candidates: usize, name: Option<String>) -> Self
    where
        R: Rank,
    {
        Ballot {
            name: name,
            ranks: vec![Default::default(); candidates],
        }
    }

    /// Retrieve the name of the ballot.
    ///
    /// ```
    /// use schulze::ballot::Ballot;
    ///
    /// let ballot: Ballot = Ballot::new(5, Some("Ivy O'Neill".to_string()));
    /// assert_eq!(ballot.name(), Some("Ivy O'Neill"));
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    /// Rank candidate with `id`.
    ///
    /// ```
    /// use schulze::ballot::Ballot;
    ///
    /// let mut ballot: Ballot = Ballot::new(5, None);
    /// ballot.rank(2, 15.into());
    /// assert!(ballot.get_rank(2) == &15.into());
    /// ```
    pub fn rank(&mut self, id: usize, rank: R) -> &mut Self
    where
        R: Rank,
    {
        self.ranks[id] = rank;
        self
    }

    /// Get rank for candidate with `id`.
    ///
    /// ```
    /// use schulze::ballot::Ballot;
    ///
    /// let mut ballot: Ballot = Ballot::new(5, None);
    /// ballot.rank(2, 15.into());
    /// assert!(ballot.get_rank(2) == &15.into());
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
    /// # use schulze::ballot::Ballot;
    /// #
    /// # let mut ballot: Ballot = Ballot::new(2, None);
    /// # ballot.rank(0, 5.into());
    /// # ballot.rank(1, None.into());
    /// assert!(&ballot.ranks()[0] == ballot.get_rank(0));
    /// assert!(&ballot.ranks()[1] == ballot.get_rank(1));
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
    use Nomination;

    #[test]
    fn ballots() {
        let mut nomination = Nomination::new();
        nomination
            .nominate("Peter Gerber")
            .nominate("Jane Doe")
            .nominate("Andrew Smith");
        let mut election = nomination.build();

        election
            .ballot()
            .rank(0, 15.into())
            .rank(1, 25.into())
            .rank(2, 0.into());

        election.ballot().rank(0, Some(5).into()).rank(
            1,
            None.into(),
        );

        election.ballot().rank(0, 0.into()).rank(1, 1.into()).rank(
            2,
            0.into(),
        );

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
}
