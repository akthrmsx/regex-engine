use anyhow::Result;
use compiler::{Compiler, Instruction};
use parser::parse;

pub(crate) mod compiler;

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
        let _ = text;
        true
    }
}
