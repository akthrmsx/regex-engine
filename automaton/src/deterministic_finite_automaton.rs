use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Automaton {
    pub(crate) start: usize,
    pub(crate) accepts: HashSet<usize>,
    pub(crate) transitions: HashMap<(usize, char), usize>,
}
