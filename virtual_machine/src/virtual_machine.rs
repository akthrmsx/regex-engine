use crate::compiler::Instruction;

#[derive(Debug, Clone, PartialEq)]
struct Thread {
    sp: usize,
    pc: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VirtualMachine {
    instructions: Vec<Instruction>,
    threads: Vec<Thread>,
}

impl VirtualMachine {
    pub(crate) fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            threads: vec![Thread { sp: 0, pc: 0 }],
        }
    }

    pub(crate) fn run(&mut self, chars: Vec<char>) -> Option<usize> {
        loop {
            match self.threads.last_mut() {
                Some(current) => match self.instructions[current.pc] {
                    Instruction::C(c) => {
                        if current.sp < chars.len() && c == chars[current.sp] {
                            current.sp += 1;
                            current.pc += 1;
                        } else {
                            self.threads.pop();
                        }
                    }
                    Instruction::J(n) => {
                        current.pc = n;
                    }
                    Instruction::S(n, m) => {
                        let mut cloned = current.clone();

                        current.pc = n;
                        cloned.pc = m;

                        self.threads.insert(self.threads.len() - 1, cloned);
                    }
                    Instruction::M => {
                        return Some(current.sp);
                    }
                },
                None => {
                    return None;
                }
            }
        }
    }
}
