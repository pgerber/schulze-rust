use std::slice;

pub struct Paths {
    candidates: usize,
    paths: Vec<u32>,
}

impl Paths {
    pub(crate) fn new(candidates: usize) -> Self {
        Paths {
            candidates,
            paths: vec![0; candidates * candidates],
        }
    }

    pub fn path(&self, to: usize, from: usize) -> u32 {
        assert_ne!(to, from, "candidates have no preference to themselves");
        self.paths[to * self.candidates + from]
    }

    pub(crate) fn mut_path(&mut self, to: usize, from: usize) -> &mut u32 {
        assert_ne!(to, from, "candidates have no preference to themselves");
        &mut self.paths[to * self.candidates + from]
    }

    pub fn iter(&self) -> PathIter {
        PathIter::new(self)
    }
}

pub struct PathIter<'a> {
    max_candidates: usize,
    paths: slice::Iter<'a, u32>,
    to: usize,
    from: usize,
}

impl<'a> PathIter<'a> {
    fn new(paths: &'a Paths) -> PathIter<'a> {
        PathIter {
            max_candidates: paths.candidates - 1,
            paths: paths.paths.iter(),
            to: 0,
            from: 0,
        }
    }

    fn increase_count(&mut self) {
        if self.from == self.max_candidates {
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

        if let Some(value) = self.paths.next() {
            let path = Some((self.to, self.from, *value));
            self.increase_count();
            path
        } else {
            None
        }
    }
}

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

        let paths_is: Vec<_> = paths.iter().collect();
        assert_eq!(
            &paths_is,
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
        let mut iter = paths.iter().skip(9);
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
}
