extern crate schulze;

mod common;

use common::*;
use schulze::*;

#[test]
/// [Example 4] from Electorama wiki
///
/// [Example 4]: http://wiki.electorama.com/w/index.php?title=Schulze_method&oldid=70012#Example_4
fn example4() {
    let mut nomination = Nomination::new();
    nomination
        .nominate("A")
        .nominate("B")
        .nominate("C")
        .nominate("D");
    let mut election = nomination.build();

    vote(&mut election, 3, "ABCD");
    vote(&mut election, 2, "DABC");
    vote(&mut election, 2, "DBCA");
    vote(&mut election, 2, "CBDA");

    assert_eq!(election.candidates().len(), 4);
    assert_eq!(election.ballots().len(), 9);

    let result = election.result();
    assert_paths_eq(
        &result,
        &[
            ('A', 'B', 5), // 5 people prefer A over B
            ('A', 'C', 5),
            ('A', 'D', 5),
            ('B', 'A', 5),
            ('B', 'C', 7),
            ('B', 'D', 5),
            ('C', 'A', 5),
            ('C', 'B', 5),
            ('C', 'D', 5),
            ('D', 'A', 6),
            ('D', 'B', 5),
            ('D', 'C', 5),
        ],
    );

    // FIXME: Sorting isn't yet correct when there are multiple possibilities
    //
    // assert_ranked_candidates_in(
    //     &result,
    //     &[
    //         &["B", "C", "D", "A"],
    //         &["B", "D", "A", "C"],
    //         &["B", "D", "C", "A"],
    //         &["D", "A", "B", "C"],
    //         &["D", "B", "A", "C"],
    //         &["D", "B", "C", "A"],
    //     ],
    // );
}
