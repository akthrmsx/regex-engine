use anyhow::Result;
use compiler::{Compiler, Instruction};
use parser::parse;
use virtual_machine::VirtualMachine;

pub(crate) mod compiler;
pub(crate) mod virtual_machine;

#[derive(Debug, Clone, PartialEq)]
pub struct Regex {
    instructions: Vec<Instruction>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        let node = parse(pattern)?;
        let instructions = node.compile();
        Ok(Self { instructions })
    }

    pub fn matches(&self, text: &str) -> bool {
        let mut virtual_machine = VirtualMachine::new(self.instructions.clone());
        match virtual_machine.run(text.chars().collect()) {
            Some(sp) => sp == text.len(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn test_matches() {
        let regex = Regex::new("(a|b)*a(a|b)(a|b)").unwrap();

        assert!(regex.clone().matches("aaa"));
        assert!(regex.clone().matches("ababa"));
        assert!(regex.clone().matches("abababa"));

        assert!(!regex.clone().matches("aa"));
        assert!(!regex.clone().matches("babab"));
        assert!(!regex.clone().matches("abbabba"));
    }
}
