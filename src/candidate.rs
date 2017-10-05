use std::cmp::PartialEq;

#[derive(Clone, Debug)]
pub struct Candidate<'a> {
    id: usize,
    name: &'a str,
}

impl<'a> Candidate<'a> {
    pub(crate) fn new(id: usize, name: &str) -> Self {
        Candidate { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }
}

impl<'a> PartialEq for Candidate<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
