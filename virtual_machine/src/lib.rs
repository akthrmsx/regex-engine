use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Regex {}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        let _ = pattern;
        Ok(Self {})
    }

    pub fn matches(&self, text: &str) -> bool {
        let _ = text;
        true
    }
}
