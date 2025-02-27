use anyhow::{bail, Result};
use nom::{
    branch::alt,
    character::complete::{anychar, char, none_of},
    combinator::{eof, map, opt},
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Empty,
    Char(char),
    Concat(Box<Node>, Box<Node>),
    Union(Box<Node>, Box<Node>),
    Star(Box<Node>),
}

impl Node {
    pub fn is_star(&self) -> bool {
        matches!(self, Self::Star(_))
    }
}

pub fn parse(input: &str) -> Result<Node> {
    match expression(input) {
        Ok((_, node)) => Ok(node),
        Err(_) => bail!("failed to parse"),
    }
}

fn expression(input: &str) -> IResult<&str, Node> {
    map((sub_expression, eof), |(node, _)| node).parse(input)
}

fn sub_expression(input: &str) -> IResult<&str, Node> {
    alt((
        map((sequence, char('|'), sub_expression), |(left, _, right)| {
            Node::Union(Box::new(left), Box::new(right))
        }),
        sequence,
    ))
    .parse(input)
}

fn sequence(input: &str) -> IResult<&str, Node> {
    map(opt(sub_sequence), |node| node.unwrap_or(Node::Empty)).parse(input)
}

fn sub_sequence(input: &str) -> IResult<&str, Node> {
    alt((
        map((star, sub_sequence), |(left, right)| {
            Node::Concat(Box::new(left), Box::new(right))
        }),
        star,
    ))
    .parse(input)
}

fn star(input: &str) -> IResult<&str, Node> {
    alt((
        map((factor, char('*')), |(node, _)| Node::Star(Box::new(node))),
        factor,
    ))
    .parse(input)
}

fn factor(input: &str) -> IResult<&str, Node> {
    alt((
        map(delimited(char('('), sub_expression, char(')')), |node| node),
        map(none_of("|*()\\"), Node::Char),
        map((char('\\'), anychar), |(_, c)| Node::Char(c)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse, Node};

    #[test]
    fn test_success() {
        assert_eq!(
            parse(r"\a|(bc)*").unwrap(),
            Node::Union(
                Box::new(Node::Char('a')),
                Box::new(Node::Star(Box::new(Node::Concat(
                    Box::new(Node::Char('b')),
                    Box::new(Node::Char('c'))
                ))))
            ),
        );

        assert_eq!(
            parse(r"a|").unwrap(),
            Node::Union(Box::new(Node::Char('a')), Box::new(Node::Empty)),
        );
    }

    #[test]
    fn test_failure() {
        assert!(parse(r"a(").is_err());
        assert!(parse(r"a)").is_err());
    }
}
