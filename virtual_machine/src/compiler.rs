use parser::Node;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Instruction {
    C(char),
    J(usize),
    S(usize, usize),
    M,
}

trait CompileHelper {
    fn recur(&self, n: usize) -> (Vec<Instruction>, usize);
}

impl CompileHelper for Node {
    fn recur(&self, n: usize) -> (Vec<Instruction>, usize) {
        let mut instructions = Vec::new();
        let mut n = n;

        match self {
            Node::Empty => (),
            Node::Char(c) => {
                instructions.push(Instruction::C(*c));
                n += 1;
            }
            Node::Concat(left, right) => {
                let (instructions1, n1) = left.recur(n);
                let (instructions2, n2) = right.recur(n1);

                instructions.extend(instructions1);
                instructions.extend(instructions2);
                n = n2;
            }
            Node::Union(left, right) => {
                let (instructions1, n1) = left.recur(n + 1);
                let (instructions2, n2) = right.recur(n1 + 1);

                instructions.push(Instruction::S(n + 1, n1 + 1));
                instructions.extend(instructions1);
                instructions.push(Instruction::J(n2));
                instructions.extend(instructions2);
                n = n2;
            }
            Node::Star(node) => {
                if node.is_star() {
                    return node.recur(n);
                } else {
                    let (instructions1, n1) = node.recur(n + 1);

                    instructions.push(Instruction::S(n + 1, n1 + 1));
                    instructions.extend(instructions1);
                    instructions.push(Instruction::J(n));
                    n = n1 + 1;
                }
            }
        }

        (instructions, n)
    }
}

pub(crate) trait Compiler {
    fn compile(&self) -> Vec<Instruction>;
}

impl Compiler for Node {
    fn compile(&self) -> Vec<Instruction> {
        let (mut instructions, _) = self.recur(0);
        instructions.push(Instruction::M);
        instructions
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, Instruction};
    use parser::Node;

    #[test]
    fn test_compile() {
        assert_eq!(Node::Empty.compile(), vec![Instruction::M]);

        assert_eq!(
            Node::Char('a').compile(),
            vec![Instruction::C('a'), Instruction::M],
        );

        assert_eq!(
            Node::Concat(Box::new(Node::Char('a')), Box::new(Node::Char('b'))).compile(),
            vec![Instruction::C('a'), Instruction::C('b'), Instruction::M],
        );

        assert_eq!(
            Node::Union(
                Box::new(Node::Concat(
                    Box::new(Node::Char('a')),
                    Box::new(Node::Char('b')),
                )),
                Box::new(Node::Concat(
                    Box::new(Node::Char('c')),
                    Box::new(Node::Char('d')),
                )),
            )
            .compile(),
            vec![
                Instruction::S(1, 4),
                Instruction::C('a'),
                Instruction::C('b'),
                Instruction::J(6),
                Instruction::C('c'),
                Instruction::C('d'),
                Instruction::M,
            ],
        );

        assert_eq!(
            Node::Star(Box::new(Node::Concat(
                Box::new(Node::Char('a')),
                Box::new(Node::Char('b')),
            )))
            .compile(),
            vec![
                Instruction::S(1, 4),
                Instruction::C('a'),
                Instruction::C('b'),
                Instruction::J(0),
                Instruction::M,
            ],
        );

        assert_eq!(
            Node::Star(Box::new(Node::Star(Box::new(Node::Concat(
                Box::new(Node::Char('a')),
                Box::new(Node::Char('b')),
            )))))
            .compile(),
            vec![
                Instruction::S(1, 4),
                Instruction::C('a'),
                Instruction::C('b'),
                Instruction::J(0),
                Instruction::M,
            ],
        );
    }
}
