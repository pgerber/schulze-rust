//! Strengths of the strongest paths
//!
//! # Example
//!
//! ```ignore("FIXME: move to crate documentation and run test (Paths::new is private)")
//! // get number of voters that prefer candidate 2 over 1
//! assert_eq!(paths.path(2, 1), 3);
//!
//! // iterate over all paths
//! assert_eq!(
//!     &paths.iter().collect::<Vec<_>>(),
//!     &[
//!         (0, 1, 8), // 8 voters prefer candidate 0 over 1
//!         (0, 2, 5),
//!         (1, 0, 7),
//!         (1, 2, 9),
//!         (2, 0, 2),
//!         (2, 1, 3)
//!     ]
//! );
//! ```

#[cfg(feature = "fused")]
use std::iter::FusedIterator;
use std::slice;

/// Strengths of the strongest paths
pub struct Paths {
    candidates: usize,
    paths: Vec<u32>,
}

impl Paths {
    /// Create storage for holding the strengths of strongest paths for N candidates
    pub(crate) fn new(candidates: usize) -> Self {
        Paths {
            candidates,
            paths: vec![0; candidates * candidates],
        }
    }

    /// Return the strength of the strongest path between candidate `to` and candidate `from`
    ///
    /// Returns the total number of voters that prefer candidate `to` over candidate `from`.
    ///
    /// # Panics
    ///
    /// Panics if `to == from` and if `to` or `from` is out of range.
    pub fn path(&self, to: usize, from: usize) -> u32 {
        assert_ne!(to, from, "candidates have no preference to themselves");
        self.paths[to * self.candidates + from]
    }

    /// Return the strength of the strongest path between candidate `to` and candidate `from`
    ///
    /// Returns a mutable reference to the total number of voters that prefer candidate `to`
    /// over candidate `from`.
    ///
    /// # Panics
    ///
    /// Panics if `to == from` and if `to` or `from` is out of range.
    pub(crate) fn mut_path(&mut self, to: usize, from: usize) -> &mut u32 {
        assert_ne!(to, from, "candidates have no preference to themselves");
        &mut self.paths[to * self.candidates + from]
    }

    /// Iterator over the strengths of all paths.
    ///
    /// The iterators yields tuples in the form `(to, from, strength)` where
    /// `strength` is the number of voters that prefer candidate `to` over
    /// candidate `from`.
    ///
    /// The items are sorted ascending, first by `to` then by `from`. For
    /// instance, when there are three candidates, the sorting looks like this:
    /// `(0, 1, _)`, `(0, 2, _)`, `(1, 0, _)`, `(1, 2, _)`, `(2, 0, _)` and
    /// then `(2, 1, _)`.
    pub fn iter(&self) -> PathIter {
        PathIter::new(self)
    }
}

/// Iterator over `Paths`
pub struct PathIter<'a> {
    max_candidate_no: usize,
    paths: slice::Iter<'a, u32>,
    to: usize,
    from: usize,
}

impl<'a> PathIter<'a> {
    fn new(paths: &'a Paths) -> PathIter<'a> {
        PathIter {
            max_candidate_no: paths.candidates - 1,
            paths: paths.paths.iter(),
            to: 0,
            from: 0,
        }
    }

    fn increase_count(&mut self) {
        if self.from == self.max_candidate_no {
            self.to += 1;
            self.from = 0;
        } else {
            self.from += 1;
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<(usize, usize, u32)> {
        if self.to == self.from {
            self.paths.next();
            self.increase_count();
        }

        self.paths.next().map(|value| {
            let path = (self.to, self.from, *value);
            self.increase_count();
            path
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.max_candidate_no + 1) * self.max_candidate_no;
        (size, Some(size))
    }
}

impl<'a> ExactSizeIterator for PathIter<'a> {}

#[cfg(feature = "fused")]
impl<'a> FusedIterator for PathIter<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        let mut paths = Paths::new(3);

        assert_eq!(paths.path(1, 0), 0);
        assert_eq!(paths.path(2, 1), 0);

        *paths.mut_path(0, 2) = 1;
        *paths.mut_path(1, 0) = 2;
        *paths.mut_path(1, 2) = 3;
        *paths.mut_path(2, 0) = 4;
        *paths.mut_path(2, 1) = 5;

        assert_eq!(paths.path(1, 0), 2);
        assert_eq!(paths.path(2, 1), 5);

        assert_eq!(
            &paths.iter().collect::<Vec<_>>(),
            &[
                (0, 1, 0),
                (0, 2, 1),
                (1, 0, 2),
                (1, 2, 3),
                (2, 0, 4),
                (2, 1, 5),
            ]
        );
    }

    #[test]
    fn exhausted_iterator() {
        let paths = Paths::new(3);
        let mut iter = paths.iter().skip(5);
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    #[should_panic(expected = "candidates have no preference to themselves")]
    fn path_to_self() {
        let paths = Paths::new(100);
        paths.path(50, 50);
    }

    #[test]
    #[should_panic(expected = "candidates have no preference to themselves")]
    fn path_to_self_mut() {
        let mut paths = Paths::new(100);
        paths.mut_path(0, 0);
    }

    #[test]
    fn iter_size_hint() {
        let paths = Paths::new(20);
        let count = paths.iter().count();
        assert_eq!(count, paths.iter().size_hint().0);
        assert_eq!(Some(count), paths.iter().size_hint().1);
        assert_eq!(count, paths.iter().len());
    }
}
