extern crate schulze;

use schulze::Nomination;
use schulze::rank::Rank;

#[test]
fn custom_ranks() {
    let mut nomination = Nomination::new();
    nomination
        .nominate("Lea")
        .nominate("Nora")
        .nominate("Zahra");
    let mut election = nomination.election_with_ranking::<CustomRank>();

    {
        let ballot = election.new_ballot();
        ballot.rank(0, 'A');
        ballot.rank(1, 'C');
        ballot.rank(2, 'B');
    }

    let result = election.result();
    let canditates: Vec<_> = result.ranked_candidates().iter().map(|c| c.name()).collect();
    assert_eq!(&canditates, &["Nora", "Zahra", "Lea"]);
}

#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
struct CustomRank(char);

impl Rank for CustomRank {
    type Ranking = char;

    fn set_rank(&mut self, rank: Self::Ranking) {
        self.0 = rank
    }

    fn get_rank(&self) -> Self::Ranking {
        self.0
    }
}

impl From<char> for CustomRank {
    fn from(c: char) -> Self {
        CustomRank(c)
    }
}