#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

use std::clone::Clone;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;


pub struct Nomination {
    candidates: Vec<Candidate>,
}

impl Nomination {
    pub fn new() -> Self {
        Nomination { candidates: Vec::new() }
    }

    pub fn nominate<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let candidate = Candidate { name: name.to_string() };
        assert!(
            !self.candidates.contains(&candidate),
            "can't add candidate {:?} that name is already used",
            candidate.name
        );
        self.candidates.push(candidate);
        self
    }

    pub fn build(self) -> Election {
        Election {
            candidates: self.candidates,
            ballots: Vec::new(),
        }
    }
}

pub struct Election {
    candidates: Vec<Candidate>,
    ballots: Vec<Ballot>,
}

impl Election {
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    pub fn ballot(&mut self) -> &mut Ballot {
        let ballot = Ballot {
            name: None,
            votes: vec![Vote::unranked(); self.candidates.len()],
        };
        self.ballots.push(ballot);
        self.ballots.last_mut().unwrap()
    }

    pub fn ballot_for<T>(&mut self, name: T) -> &mut Ballot
    where
        T: ToString,
    {
        let ballot = Ballot {
            name: Some(name.to_string()),
            votes: vec![Vote::unranked(); self.candidates.len()],
        };
        self.ballots.push(ballot);
        self.ballots.last_mut().unwrap()
    }

    pub fn ballots(&self) -> &[Ballot] {
        &self.ballots
    }

    pub fn result(&self) -> ElectionResult {
        let paths = self.find_strongest_paths();
        let mut ranking: Vec<_> = (0_usize..self.candidates.len()).collect();
        ranking.sort_unstable_by(|s, o| paths[&(*o, *s)].cmp(&paths[&(*s, *o)]));
        let ranked_candidates: Vec<_> = ranking
            .iter()
            .map(|i| self.candidates[*i].clone())
            .collect();

        ElectionResult {
            ranked_candidates,
            paths,
        }
    }

    fn find_strongest_paths(&self) -> HashMap<(usize, usize), u32> {
        let mut result = HashMap::new();

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    let preferring_i = self.preference(i, j);
                    let val = if preferring_i > self.preference(j, i) {
                        preferring_i
                    } else {
                        0
                    };
                    result.insert((i, j), val);
                }
            }
        }

        for i in 0..self.candidates.len() {
            for j in 0..self.candidates.len() {
                if i != j {
                    for k in 0..self.candidates.len() {
                        if i != k && j != k {
                            let j_k = result[&(j, k)];
                            let j_i = result[&(j, i)];
                            let i_k = result[&(i, k)];
                            result.insert((j, k), max(j_k, min(j_i, i_k)));
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
            .filter(|b| b.votes()[i] > b.votes()[j])
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

#[derive(Debug)]
pub struct Ballot {
    name: Option<String>,
    votes: Vec<Vote>,
}

impl Ballot {
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn vote(&mut self, id: usize, rank: Vote) -> &mut Self {
        self.votes[id] = rank;
        self
    }

    pub fn votes(&self) -> &[Vote] {
        &self.votes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vote {
    rank: Option<u8>,
}

impl Vote {
    pub fn new(rank: Option<u8>) -> Self {
        Vote { rank }
    }

    pub fn ranked(rank: u8) -> Self {
        Vote { rank: Some(rank) }
    }

    pub fn unranked() -> Self {
        Vote { rank: None }
    }

    fn rank(&self) -> Option<u8> {
        self.rank
    }
}

impl PartialOrd for Vote {
    fn partial_cmp(&self, other: &Vote) -> Option<Ordering> {
        match (self.rank(), other.rank()) {
            (Some(s), Some(o)) => o.partial_cmp(&s),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl Ord for Vote {
    fn cmp(&self, other: &Vote) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<Option<u8>> for Vote {
    fn from(v: Option<u8>) -> Self {
        Vote::new(v)
    }
}

impl From<u8> for Vote {
    fn from(v: u8) -> Self {
        Vote::new(Some(v))
    }
}

pub struct ElectionResult {
    ranked_candidates: Vec<Candidate>,
    paths: HashMap<(usize, usize), u32>,
}

impl ElectionResult {
    pub fn ranked_candidates(&self) -> &[Candidate] {
        &self.ranked_candidates
    }

    pub fn paths(&self) -> &HashMap<(usize, usize), u32> {
        &self.paths
    }
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    extern crate test;

    use super::*;

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
            .vote(0, 15.into())
            .vote(1, 25.into())
            .vote(2, 0.into());

        election.ballot().vote(0, Some(5).into()).vote(
            1,
            None.into(),
        );

        election.ballot().vote(0, 0.into()).vote(1, 1.into()).vote(
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
            .map(|b| b.votes().iter().map(|v| v.rank()).collect::<Vec<_>>())
            .collect();
        assert_eq!(&shall, &is.as_slice());
    }

    #[test]
    fn vote_partial_cmp() {
        assert_eq!(Vote::from(5), Vote::from(5));
        assert!(Vote::from(5) > Vote::from(15));
        assert!(Vote::from(5) > Vote::from(None));
        assert!(Vote::from(None) < Vote::from(15));
        assert_eq!(Vote::from(None), Vote::from(None));
    }

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
                ballot.vote(j as usize, Vote::from(((i + j) % 50) as u8));
            }
        }

        b.iter(|| { election.find_strongest_paths(); });
    }
}
