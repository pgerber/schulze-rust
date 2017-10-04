extern crate schulze;

mod common;

use common::*;
use schulze::*;

#[test]
/// [Example 3] from Electorama wiki
///
/// [Example 3]: http://wiki.electorama.com/w/index.php?title=Schulze_method&oldid=70012#Example_3
fn example3() {
    let mut nomination = Nomination::new();
    nomination
        .nominate("A")
        .nominate("B")
        .nominate("C")
        .nominate("D")
        .nominate("E");
    let mut election = nomination.build();

    rank(&mut election, 3, "ABDEC");
    rank(&mut election, 5, "ADEBC");
    rank(&mut election, 1, "ADECB");
    rank(&mut election, 2, "BADEC");
    rank(&mut election, 2, "BDECA");
    rank(&mut election, 4, "CABDE");
    rank(&mut election, 6, "CBADE");
    rank(&mut election, 2, "DBECA");
    rank(&mut election, 5, "DECAB");

    assert_eq!(election.candidates().len(), 5);
    assert_eq!(election.ballots().len(), 30);

    let result = election.result();
    assert_paths_eq(
        &result,
        &[
            ('A', 'B', 18), // 18 people prefer A over B
            ('A', 'C', 20),
            ('A', 'D', 21),
            ('A', 'E', 21),
            ('B', 'A', 19),
            ('B', 'C', 19),
            ('B', 'D', 19),
            ('B', 'E', 19),
            ('C', 'A', 19),
            ('C', 'B', 18),
            ('C', 'D', 19),
            ('C', 'E', 19),
            ('D', 'A', 19),
            ('D', 'B', 18),
            ('D', 'C', 20),
            ('D', 'E', 30),
            ('E', 'A', 19),
            ('E', 'B', 18),
            ('E', 'C', 20),
            ('E', 'D', 19),
        ],
    );
    assert_ranked_candidates_eq(&result, &["B", "A", "D", "E", "C"]);
}
