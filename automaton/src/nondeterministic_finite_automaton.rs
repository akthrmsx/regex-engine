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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Automaton {
    pub(crate) start: usize,
    pub(crate) accepts: HashSet<usize>,
    pub(crate) transitions: HashMap<usize, HashMap<Option<char>, HashSet<usize>>>,
}

impl Automaton {
    fn new(start: usize, accepts: HashSet<usize>) -> Self {
        Self {
            start,
            accepts,
            transitions: HashMap::new(),
        }
    }

    fn add_transition(&mut self, from: usize, to: usize, c: char) {
        self.transitions
            .entry(from)
            .or_default()
            .entry(Some(c))
            .or_default()
            .insert(to);
    }

    fn add_epsilon_transition(&mut self, from: usize, to: usize) {
        self.transitions
            .entry(from)
            .or_default()
            .entry(None)
            .or_default()
            .insert(to);
    }

    fn merge_transitions(&mut self, other: &HashMap<usize, HashMap<Option<char>, HashSet<usize>>>) {
        for (from, transitions) in other {
            for (c, tos) in transitions {
                self.transitions
                    .entry(*from)
                    .or_default()
                    .entry(*c)
                    .or_default()
                    .extend(tos);
            }
        }
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
}
