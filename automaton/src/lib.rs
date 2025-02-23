use anyhow::Result;
use deterministic_finite_automaton::Automaton as DFA;
use nondeterministic_finite_automaton::Automaton as NFA;
use parser::parse;

pub(crate) mod deterministic_finite_automaton;
pub(crate) mod nondeterministic_finite_automaton;

#[derive(Debug, Clone, PartialEq)]
pub struct Regex {
    automaton: DFA,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        let node = parse(pattern)?;

        let mut nfa = NFA::from(node);
        nfa.remove_epsilon_transitions();

        let dfa = DFA::from(nfa);

        // TODO: 最小化

        Ok(Self { automaton: dfa })
    }

    pub fn matches(&self, text: &str) -> bool {
        let mut current = self.automaton.start;

        for c in text.chars() {
            match self.automaton.transitions.get(&(current, c)) {
                Some(destination) => current = *destination,
                None => return false,
            }
        }

        self.automaton.accepts.contains(&current)
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn test_matches() {
        let regex = Regex::new("(a|b)*a(a|b)(a|b)").unwrap();

        assert!(regex.matches("aaa"));
        assert!(regex.matches("ababa"));
        assert!(regex.matches("abababa"));

        assert!(!regex.matches("aa"));
        assert!(!regex.matches("babab"));
        assert!(!regex.matches("abbabba"));
    }
}
