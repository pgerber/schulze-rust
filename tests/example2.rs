extern crate schulze;

mod common;

use common::*;
use schulze::nomination::Nomination;

#[test]
/// [Example 2] from Electorama wiki
///
/// [Example 2]: http://wiki.electorama.com/w/index.php?title=Schulze_method&oldid=70012#Example_2
fn example2() {
    let mut nomination = Nomination::new();
    nomination
        .nominate("A")
        .nominate("B")
        .nominate("C")
        .nominate("D");
    let mut election = nomination.build();

    rank(&mut election, 5, "ACBD");
    rank(&mut election, 2, "ACDB");
    rank(&mut election, 3, "ADCB");
    rank(&mut election, 4, "BACD");
    rank(&mut election, 3, "CBDA");
    rank(&mut election, 3, "CDBA");
    rank(&mut election, 1, "DACB");
    rank(&mut election, 5, "DBAC");
    rank(&mut election, 4, "DCBA");

    assert_eq!(election.candidates().len(), 4);
    assert_eq!(election.ballots().len(), 30);

    let result = election.result();
    assert_paths_eq(
        &result,
        &[
            ('A', 'B', 20), // 20 people prefer A over B
            ('A', 'C', 20),
            ('A', 'D', 17),
            ('B', 'A', 19),
            ('B', 'C', 19),
            ('B', 'D', 17),
            ('C', 'A', 19),
            ('C', 'B', 21),
            ('C', 'D', 17),
            ('D', 'A', 18),
            ('D', 'B', 18),
            ('D', 'C', 18),
        ],
    );
    assert_ranked_candidates_eq(&result, &["D", "A", "C", "B"]);
}
