//! Ranks on `Ballot`s
//!
//! Custom ranks can be used by implementing [`Rank`]. By default, [`SimpleRank`] is used.
//!
//! [`Rank`]: trait.Rank.html
//! [`SimpleRank`]: struct.SimpleRank.html

use std::cmp::Ordering;

/// Ranking used on `Ballot`s.
pub trait Rank: Clone + Default + Ord {
    type Ranking;

    /// Set the rank
    fn set_rank(&mut self, rank: Self::Ranking);

    /// Get the rank
    fn get_rank(&self) -> Self::Ranking;
}

/// Very simple implementation of ranks.
///
/// # High, Low and Absent Ranks
///
/// Ranks are from `Some(0_u8)` to `Some(255_u8)` where the former is the highest and the
/// latter the second lowest rank; `None` represents the absence of a rank and is considered
/// even lower.
///
/// ```
/// use schulze::rank::SimpleRank;
///
/// assert!(SimpleRank::new(Some(0)) > SimpleRank::new(Some(255)));
/// assert!(SimpleRank::new(Some(255)) > SimpleRank::new(None));
/// ```
///
/// # Using `From` trait
///
/// `Option<u8>` and `u8` types can be converted easily.
///
/// ```
/// use schulze::rank::SimpleRank;
///
/// let rank1: SimpleRank = 5.into();
/// let rank2: SimpleRank = Some(5).into();
/// assert!(rank1 == rank2);
/// ```
#[derive(Clone, Default, Eq, PartialEq)]
pub struct SimpleRank {
    rank: Option<u8>,
}

impl SimpleRank {
    /// Create new rank with value `rank`.
    pub fn new(rank: Option<u8>) -> Self {
        SimpleRank { rank }
    }
}

impl Rank for SimpleRank {
    type Ranking = Option<u8>;

    fn set_rank(&mut self, rank: Self::Ranking) {
        self.rank = rank
    }

    fn get_rank(&self) -> Self::Ranking {
        self.rank
    }
}

impl PartialOrd for SimpleRank {
    fn partial_cmp(&self, other: &SimpleRank) -> Option<Ordering> {
        match (self.get_rank(), other.get_rank()) {
            (Some(s), Some(o)) => o.partial_cmp(&s),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl Ord for SimpleRank {
    fn cmp(&self, other: &SimpleRank) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<Option<u8>> for SimpleRank {
    fn from(v: Option<u8>) -> Self {
        SimpleRank::new(v)
    }
}

impl From<u8> for SimpleRank {
    fn from(v: u8) -> Self {
        SimpleRank::new(Some(v))
    }
}

#[cfg(tests)]
mod test {
    use super::*;

    #[test]
    fn vote_set_get() {
        let vote = Rank::new(Some(3));
        assert_eq!(vote.get_vote(), Some(3));
        vote.set_vote(None);
        assert_eq!(vote.get_vote(), None);
    }

    #[test]
    fn vote_partial_cmp() {
        assert_eq!(SimpleRank::from(5), SimpleRank::from(5));
        assert!(SimpleRank::from(5) > SimpleRank::from(15));
        assert!(SimpleRank::from(5) > SimpleRank::from(None));
        assert!(SimpleRank::from(None) < SimpleRank::from(15));
        assert_eq!(SimpleRank::from(None), SimpleRank::from(None));
    }
}
