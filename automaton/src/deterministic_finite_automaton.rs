use crate::nondeterministic_finite_automaton::Automaton as NFA;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Next {
    New(usize),
    Contains(usize),
}

impl Next {
    fn unwrap(self) -> usize {
        match self {
            Self::New(n) => n,
            Self::Contains(n) => n,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Context {
    current: usize,
    transitions: HashMap<Vec<usize>, usize>,
}

impl Context {
    fn new() -> Self {
        Context {
            current: 0,
            transitions: HashMap::new(),
        }
    }

    fn next(&mut self, destinations: &HashSet<usize>) -> Next {
        let mut destinations = Vec::from_iter(destinations.clone());
        destinations.sort();

        match self.transitions.get(&destinations) {
            Some(destination) => Next::Contains(*destination),
            None => {
                let current = self.current;
                self.current += 1;
                self.transitions.insert(destinations, current);
                Next::New(current)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Automaton {
    pub(crate) start: usize,
    pub(crate) accepts: HashSet<usize>,
    pub(crate) transitions: HashMap<(usize, char), usize>,
}

impl Automaton {
    pub(crate) fn minimize(&mut self) {
        // TODO
    }
}

impl From<NFA> for Automaton {
    fn from(nfa: NFA) -> Self {
        let mut context = Context::new();
        let mut queue = vec![[nfa.start].into()];

        let start = context.next(&[nfa.start].into()).unwrap();
        let mut accepts = HashSet::new();
        let mut transitions = HashMap::new();

        while let Some(destinations) = queue.pop() {
            if !nfa.accepts.is_disjoint(&destinations) {
                accepts.insert(context.next(&destinations).unwrap());
            }

            let chars = nfa.calc_chars_without_epsilon_transitions(Some(&destinations));
            let mut chars = Vec::from_iter(chars);
            chars.sort();

            for c in chars {
                let from = context.next(&destinations).unwrap();
                let destination = {
                    let destinations = destinations
                        .iter()
                        .flat_map(|destination| nfa.calc_destinations(*destination, Some(c)))
                        .collect::<HashSet<_>>();

                    if destinations.is_empty() {
                        continue;
                    }

                    match context.next(&destinations) {
                        Next::New(destination) => {
                            queue.push(destinations);
                            destination
                        }
                        Next::Contains(destination) => destination,
                    }
                };

                transitions.entry((from, c)).or_insert(destination);
            }
        }

        Self {
            start,
            accepts,
            transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        deterministic_finite_automaton::Automaton,
        nondeterministic_finite_automaton::Automaton as NFA,
    };

    #[test]
    fn test_from_nfa() {
        assert_eq!(
            Automaton::from(NFA {
                start: 0,
                accepts: [2].into(),
                transitions: [
                    (
                        0,
                        [(Some('a'), [0, 2].into()), (Some('b'), [1].into())].into(),
                    ),
                    (
                        1,
                        [(Some('a'), [2].into()), (Some('b'), [1, 2].into())].into(),
                    ),
                ]
                .into(),
            }),
            Automaton {
                start: 0,
                accepts: [1, 3, 4].into(),
                transitions: [
                    ((0, 'a'), 1),
                    ((0, 'b'), 2),
                    ((1, 'a'), 1),
                    ((1, 'b'), 2),
                    ((2, 'a'), 3),
                    ((2, 'b'), 4),
                    ((4, 'a'), 3),
                    ((4, 'b'), 4),
                ]
                .into(),
            },
        );
    }
}
