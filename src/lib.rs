//! # Implementation of the Schulze Method
//!
//! # Example
//!
//! ```
//! use schulze::Nomination;
//!
//  // nominate candidates and create election
//! let mut nomination = Nomination::new();
//! nomination
//!     .nominate("Ivy Winter")    // candidate 0
//!     .nominate("Lena Summer")   // candidate 1
//!     .nominate("Lea Tanner");   // candidate 2
//! let mut election = nomination.build();
//!
//! // fill in first ballot
//! election.new_ballot()
//!     .rank(0, 3)     // rank candidate 0
//!     .rank(1, 1)     // rank candidate 1
//!     .rank(2, 2);    // rank candidate 2
//!
//! // fill in second ballot
//! election.new_ballot()
//!     .rank_all(&[None, Some(1), Some(1)]); // rank all three candidates at once
//!
//! // fill in third ballot
//! election.new_ballot()
//!     // .rank(0, None).into()) // None is default
//!     .rank(1, 1)
//!     .rank(2, 2);
//!
//! // get election results
//! let result = election.result();
//! assert_eq!(
//!     &result.ranked_candidates().iter().map(|c| c.name()).collect::<Vec<_>>(),
//!     &["Lena Summer", "Lea Tanner", "Ivy Winter"]); // Lena 1st, Lea 2nd and Ivy 3rd
//! ```

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

pub mod ballot;
pub mod election;
pub mod nomination;
pub mod paths;
pub mod rank;

pub use nomination::Nomination;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    name: String,
}

impl Candidate {
    pub fn name(&self) -> &str {
        &self.name
    }
}
