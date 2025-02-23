use parser::Node;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct Context {
    current: usize,
}

impl Context {
    fn new() -> Self {
        Context { current: 0 }
    }

    fn next(&mut self) -> usize {
        let current = self.current;
        self.current += 1;
        current
    }
}

type Transitions = HashMap<usize, HashMap<Option<char>, HashSet<usize>>>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Automaton {
    pub(crate) start: usize,
    pub(crate) accepts: HashSet<usize>,
    pub(crate) transitions: Transitions,
}

impl Automaton {
    fn new(start: usize, accepts: HashSet<usize>) -> Self {
        Self {
            start,
            accepts,
            transitions: HashMap::new(),
        }
    }

    fn add_transition(&mut self, from: usize, destination: usize, c: char) {
        self.transitions
            .entry(from)
            .or_default()
            .entry(Some(c))
            .or_default()
            .insert(destination);
    }

    fn add_epsilon_transition(&mut self, from: usize, destination: usize) {
        self.transitions
            .entry(from)
            .or_default()
            .entry(None)
            .or_default()
            .insert(destination);
    }

    fn merge_transitions(&mut self, other: &Transitions) {
        for (from, transitions) in other {
            for (c, destinations) in transitions {
                self.transitions
                    .entry(*from)
                    .or_default()
                    .entry(*c)
                    .or_default()
                    .extend(destinations);
            }
        }
    }

    pub(crate) fn remove_epsilon_transitions(&mut self) {
        if self
            .accepts
            .is_subset(&self.calc_epsilon_closure(self.start))
        {
            self.accepts.insert(self.start);
        }

        self.transitions = self.new_transitions_without_epsilon_transitions();
    }

    fn new_transitions_without_epsilon_transitions(&self) -> Transitions {
        let mut new_transitions: Transitions = HashMap::new();

        for from in self.transitions.keys().cloned() {
            let chars = self.calc_chars_without_epsilon_transitions(None);
            let mut chars = Vec::from_iter(chars);
            chars.sort();

            for c in chars {
                for destination in self.calc_epsilon_closure(from) {
                    let destinations = self.expand_epsilon_closure(destination, Some(c));

                    if destinations.is_empty() {
                        continue;
                    }

                    new_transitions
                        .entry(from)
                        .or_default()
                        .entry(Some(c))
                        .or_default()
                        .extend(destinations);
                }
            }
        }

        new_transitions
    }

    fn expand_epsilon_closure(&self, from: usize, c: Option<char>) -> HashSet<usize> {
        let first_destinations = self.calc_epsilon_closure(from);
        let mut second_destinations = HashSet::new();
        let mut final_destinations = HashSet::new();

        for destination in first_destinations {
            second_destinations.extend(self.calc_destinations(destination, c));
        }

        for destination in second_destinations {
            final_destinations.extend(self.calc_epsilon_closure(destination));
        }

        final_destinations
    }

    fn calc_epsilon_closure(&self, from: usize) -> HashSet<usize> {
        let mut epsilon_closure = HashSet::new();
        epsilon_closure.insert(from);
        let mut modified = true;

        while modified {
            let mut temp_epsilon_closure = HashSet::<usize>::new();
            modified = false;

            for destination in &epsilon_closure {
                let destinations = self.calc_destinations(*destination, None);

                if destinations.is_empty()
                    || epsilon_closure
                        .union(&temp_epsilon_closure)
                        .cloned()
                        .collect::<HashSet<_>>()
                        .is_superset(&destinations)
                {
                    continue;
                }

                temp_epsilon_closure.extend(destinations);
                modified = true;
            }

            epsilon_closure.extend(temp_epsilon_closure);
        }

        epsilon_closure
    }

    pub(crate) fn calc_destinations(&self, from: usize, c: Option<char>) -> HashSet<usize> {
        self.transitions
            .get(&from)
            .cloned()
            .unwrap_or_default()
            .get(&c)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn calc_chars_without_epsilon_transitions(
        &self,
        destinations: Option<&HashSet<usize>>,
    ) -> HashSet<char> {
        self.transitions
            .iter()
            .filter(|(from, _)| {
                destinations
                    .map(|destinations| destinations.contains(from))
                    .unwrap_or(true)
            })
            .flat_map(|(_, transitions)| transitions.keys().flatten())
            .cloned()
            .collect()
    }
}

impl From<Node> for Automaton {
    fn from(node: Node) -> Self {
        node.assemble(&mut Context::new())
    }
}

trait Assembler {
    fn assemble(&self, context: &mut Context) -> Automaton;
}

impl Assembler for Node {
    fn assemble(&self, context: &mut Context) -> Automaton {
        match self {
            Node::Empty => {
                let start = context.next();
                let accept = context.next();
                let accepts = [accept].into();

                let mut automaton = Automaton::new(start, accepts);
                automaton.add_epsilon_transition(start, accept);

                automaton
            }
            Node::Char(c) => {
                let start = context.next();
                let accept = context.next();
                let accepts = [accept].into();

                let mut automaton = Automaton::new(start, accepts);
                automaton.add_transition(start, accept, *c);

                automaton
            }
            Node::Concat(left, right) => {
                let left = left.assemble(context);
                let right = right.assemble(context);
                let start = left.start;
                let accepts = right.accepts.clone();

                let mut automaton = Automaton::new(start, accepts);
                automaton.merge_transitions(&left.transitions);
                automaton.merge_transitions(&right.transitions);

                for accept in left.accepts {
                    let start = accept;
                    let accept = right.start;
                    automaton.add_epsilon_transition(start, accept);
                }

                automaton
            }
            Node::Union(left, right) => {
                let left = left.assemble(context);
                let right = right.assemble(context);
                let start = context.next();
                let accept = context.next();
                let accepts = [accept].into();

                let mut automaton = Automaton::new(start, accepts);
                automaton.add_epsilon_transition(start, left.start);
                automaton.add_epsilon_transition(start, right.start);
                automaton.merge_transitions(&left.transitions);
                automaton.merge_transitions(&right.transitions);

                for left_accept in left.accepts {
                    let start = left_accept;
                    automaton.add_epsilon_transition(start, accept);
                }

                for right_accept in right.accepts {
                    let start = right_accept;
                    automaton.add_epsilon_transition(start, accept);
                }

                automaton
            }
            Node::Star(node) => {
                let inner = node.assemble(context);
                let start = context.next();
                let accept = context.next();
                let accepts = inner.accepts.union(&[accept].into()).cloned().collect();

                let mut automaton = Automaton::new(start, accepts);
                automaton.add_epsilon_transition(start, inner.start);
                automaton.add_epsilon_transition(start, accept);
                automaton.merge_transitions(&inner.transitions);

                for accept in inner.accepts {
                    let start = accept;
                    let accept = inner.start;
                    automaton.add_epsilon_transition(start, accept);
                }

                automaton
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nondeterministic_finite_automaton::Automaton;
    use parser::Node;

    #[test]
    fn test_from_node() {
        assert_eq!(
            Automaton::from(Node::Empty),
            Automaton {
                start: 0,
                accepts: [1].into(),
                transitions: [(0, [(None, [1].into())].into())].into(),
            },
        );

        assert_eq!(
            Automaton::from(Node::Char('a')),
            Automaton {
                start: 0,
                accepts: [1].into(),
                transitions: [(0, [(Some('a'), [1].into())].into())].into(),
            },
        );

        assert_eq!(
            Automaton::from(Node::Concat(
                Box::new(Node::Char('a')),
                Box::new(Node::Char('b')),
            )),
            Automaton {
                start: 0,
                accepts: [3].into(),
                transitions: [
                    (0, [(Some('a'), [1].into())].into()),
                    (1, [(None, [2].into())].into()),
                    (2, [(Some('b'), [3].into())].into()),
                ]
                .into(),
            },
        );

        assert_eq!(
            Automaton::from(Node::Union(
                Box::new(Node::Char('a')),
                Box::new(Node::Char('b')),
            )),
            Automaton {
                start: 4,
                accepts: [5].into(),
                transitions: [
                    (4, [(None, [0, 2].into())].into()),
                    (0, [(Some('a'), [1].into())].into()),
                    (2, [(Some('b'), [3].into())].into()),
                    (1, [(None, [5].into())].into()),
                    (3, [(None, [5].into())].into()),
                ]
                .into(),
            },
        );

        assert_eq!(
            Automaton::from(Node::Star(Box::new(Node::Char('a')))),
            Automaton {
                start: 2,
                accepts: [1, 3].into(),
                transitions: [
                    (2, [(None, [0, 3].into())].into()),
                    (0, [(Some('a'), [1].into())].into()),
                    (1, [(None, [0].into())].into()),
                ]
                .into(),
            },
        );
    }

    #[test]
    fn test_remove_epsilon_transition() {
        let mut automaton = Automaton {
            start: 0,
            accepts: [2].into(),
            transitions: [
                (0, [(Some('a'), [0].into()), (None, [1].into())].into()),
                (1, [(Some('b'), [1].into()), (None, [2].into())].into()),
                (2, [(Some('c'), [2].into())].into()),
            ]
            .into(),
        };
        automaton.remove_epsilon_transitions();

        assert_eq!(
            automaton,
            Automaton {
                start: 0,
                accepts: [0, 2].into(),
                transitions: [
                    (
                        0,
                        [
                            (Some('a'), [0, 1, 2].into()),
                            (Some('b'), [1, 2].into()),
                            (Some('c'), [2].into())
                        ]
                        .into()
                    ),
                    (
                        1,
                        [(Some('b'), [1, 2].into()), (Some('c'), [2].into())].into()
                    ),
                    (2, [(Some('c'), [2].into())].into()),
                ]
                .into(),
            },
        );

        let mut automaton = Automaton {
            start: 0,
            accepts: [3, 4].into(),
            transitions: [
                (0, [(Some('a'), [1].into())].into()),
                (1, [(None, [2].into()), (Some('b'), [1, 3].into())].into()),
                (2, [(Some('a'), [4].into())].into()),
            ]
            .into(),
        };
        automaton.remove_epsilon_transitions();

        assert_eq!(
            automaton,
            Automaton {
                start: 0,
                accepts: [3, 4].into(),
                transitions: [
                    (0, [(Some('a'), [1, 2].into())].into()),
                    (
                        1,
                        [(Some('a'), [4].into()), (Some('b'), [1, 2, 3].into())].into()
                    ),
                    (2, [(Some('a'), [4].into())].into()),
                ]
                .into(),
            },
        );
    }
}
