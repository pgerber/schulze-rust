//! Nomination of candidates
//!
//! # Example
//!
//! ```
//! use schulze::nomination::Nomination;
//!
//! let mut nomination = Nomination::new();
//!
//! // nominate three candidates
//! nomination
//!     .nominate("Dianne")
//!     .nominate("John")
//!     .nominate("Ivy");
//!
//! // create election
//! let election = nomination.build();
//! ```

use {Candidate, Election};

/// Nomination of candidates
pub struct Nomination {
    candidates: Vec<Candidate>,
}

impl Nomination {
    /// Create new nomination.
    #[cfg_attr(feature = "clippy", allow(new_without_default_derive))]
    pub fn new() -> Self {
        Nomination { candidates: Vec::new() }
    }

    /// Nominate candidate with name.
    ///
    /// # Panics
    ///
    /// Panics if `name` has been nominated already.
    pub fn nominate<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let candidate = Candidate { name: name.to_string() };
        assert!(
            !self.candidates.contains(&candidate),
            "can't add second candidate with name {:?}",
            candidate.name
        );
        self.candidates.push(candidate);
        self
    }

    /// Create election
    pub fn build(self) -> Election {
        Election {
            candidates: self.candidates,
            ballots: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominate() {
        let mut nomination = Nomination::new();
        nomination
            .nominate("Dianne Summer")
            .nominate("John Winter")
            .nominate("Ivy Spring");
        let election = nomination.build();

        assert_eq!(
            election
                .candidates
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            &["Dianne Summer", "John Winter", "Ivy Spring"]
        );
    }

    #[test]
    #[should_panic(expected = "can't add second candidate with name \"Jane Doe\"")]
    fn duplicate_name() {
        let mut nomination = Nomination::new();
        nomination
            .nominate("Jane Doe")
            .nominate("John Doe")
            .nominate("Jane Doe");
    }
}
