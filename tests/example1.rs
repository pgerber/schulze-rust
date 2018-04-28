extern crate schulze;

mod common;

use common::*;
use schulze::nomination::Nomination;

#[test]
/// [Example] from Wikipedia
///
/// [Example]: https://en.wikipedia.org/w/index.php?title=Schulze_method&oldid=800023801#Example
fn example1() {
    let mut nomination = Nomination::new();
    nomination
        .nominate("A")
        .nominate("B")
        .nominate("C")
        .nominate("D")
        .nominate("E");
    let mut election = nomination.election();

    rank(&mut election, 5, "ACBED");
    rank(&mut election, 5, "ADECB");
    rank(&mut election, 8, "BEDAC");
    rank(&mut election, 3, "CABED");
    rank(&mut election, 7, "CAEBD");
    rank(&mut election, 2, "CBADE");
    rank(&mut election, 7, "DCEBA");
    rank(&mut election, 8, "EBADC");

    assert_eq!(election.candidates().len(), 5);
    assert_eq!(election.ballots().len(), 45);

    let result = election.result();
    assert_paths_eq(
        &result,
        &[
            ('A', 'B', 28), // 28 people prefer A over B
            ('A', 'C', 28),
            ('A', 'D', 30),
            ('A', 'E', 24),
            ('B', 'A', 25),
            ('B', 'C', 28),
            ('B', 'D', 33),
            ('B', 'E', 24),
            ('C', 'A', 25),
            ('C', 'B', 29),
            ('C', 'D', 29),
            ('C', 'E', 24),
            ('D', 'A', 25),
            ('D', 'B', 28),
            ('D', 'C', 28),
            ('D', 'E', 24),
            ('E', 'A', 25),
            ('E', 'B', 28),
            ('E', 'C', 28),
            ('E', 'D', 31),
        ],
    );
    assert_ranked_candidates_eq(&result, &["E", "A", "C", "B", "D"]);
}
