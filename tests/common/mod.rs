#![allow(dead_code)]

use schulze::election::{Election, ElectionResult};

pub fn rank(election: &mut Election, count: u32, ranks: &str) {
    for _ in 0..count {
        let ballot = election.new_ballot();
        for (i, v) in ranks.as_bytes().iter().enumerate() {
            ballot.rank((v - b'A') as usize, (i as u8));
        }
    }
}

pub fn extract_paths(result: &ElectionResult) -> Vec<(char, char, u32)> {
    let mut paths: Vec<_> = result
        .paths()
        .iter()
        .map(|(to, from, pref)| {
            ((to as u8 + b'A') as char, (from as u8 + b'A') as char, pref)
        })
        .collect();
    paths.sort_unstable();
    paths
}

pub fn assert_paths_eq(result: &ElectionResult, other: &[(char, char, u32)]) {
    let paths_is = extract_paths(result);
    assert_eq!(paths_is, other);
}

pub fn extract_ranked_candidates(result: &ElectionResult) -> Vec<&str> {
    result
        .ranked_candidates()
        .iter()
        .map(|c| c.name())
        .collect()
}

pub fn assert_ranked_candidates_eq(result: &ElectionResult, other: &[&str]) {
    let candidates_is = extract_ranked_candidates(result);
    assert_eq!(candidates_is, other);
}

pub fn assert_ranked_candidates_in(result: &ElectionResult, others: &[&[&str]]) {
    let candidates_is = extract_ranked_candidates(result);
    assert!(
        others.contains(&candidates_is.as_slice()),
        "{:?} is not in {:?}",
        candidates_is,
        others
    );
}
