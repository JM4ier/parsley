use crate::bnf::*;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};

mod test;

const SPECIAL_CHARS: &str = "=|<>()[]{}";

type Res<'a, T> = IResult<&'a str, T>;

macro_rules! delim {
    ($left:literal, $right:literal, $parse:expr) => {
        delimited(tag($left), $parse, tag($right))
    };
}

pub fn bnf_rules(i: &str) -> Res<Vec<BnfRule>> {
    separated_list1(tag("\n"), bnf_rule)(i)
}

pub fn bnf_rule(i: &str) -> Res<BnfRule> {
    map(
        tuple((rule_name, tag("<="), bnf_rule_def)),
        |(name, _, def)| BnfRule { name, def },
    )(i)
}

pub fn bnf_rule_def(i: &str) -> Res<BnfPart> {
    terminated(rule_def, alt((eof, tag("\n"))))(i)
}

fn rule_def(i: &str) -> Res<BnfPart> {
    for closing in vec![")", ">", "]"] {
        if i.starts_with(closing) || i == "" {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Fix,
            )));
        }
    }
    map(
        many1(alt((choice, grouped, rule_instant, opt, rep, lit))),
        |parts| {
            if parts.len() == 1 {
                parts.into_iter().next().unwrap()
            } else {
                BnfPart::Concat(parts)
            }
        },
    )(i)
}

fn rule_def1(i: &str) -> Res<BnfPart> {
    for closing in vec![")", ">", "]"] {
        if i.starts_with(closing) || i == "" {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Fix,
            )));
        }
    }
    map(
        many1(alt((grouped, rule_instant, opt, rep, lit))),
        |parts| {
            if parts.len() == 1 {
                parts.into_iter().next().unwrap()
            } else {
                BnfPart::Concat(parts)
            }
        },
    )(i)
}

fn rule_name(i: &str) -> Res<String> {
    let name_parse = map(
        take_while1(|c: char| !SPECIAL_CHARS.contains(c)),
        str::to_string,
    );
    delim!("<", ">", name_parse)(i)
}

fn choice(i: &str) -> Res<BnfPart> {
    let sl2 = tuple((rule_def1, tag("|"), separated_list1(tag("|"), rule_def1)));
    map(sl2, |(head, _, mut tail)| {
        tail.insert(0, head);
        BnfPart::Choice(tail)
    })(i)
}

fn rule_instant(i: &str) -> Res<BnfPart> {
    map(rule_name, |name| BnfPart::Rule(name))(i)
}

fn grouped(i: &str) -> Res<BnfPart> {
    delim!("(", ")", rule_def)(i)
}

fn rep(i: &str) -> Res<BnfPart> {
    delim!("{", "}", map(rule_def, |e| BnfPart::Repeat(Box::new(e))))(i)
}

fn opt(i: &str) -> Res<BnfPart> {
    let map = map(rule_def, |e| BnfPart::Opt(e));
    delim!("[", "]", map)(i)
}

fn lit(i: &str) -> Res<BnfPart> {
    map_res(
        escaped(none_of(SPECIAL_CHARS), '\\', one_of(SPECIAL_CHARS)),
        |lit: &str| {
            if lit.len() == 0 {
                Err("empty literal")
            } else {
                Ok(BnfPart::Literal(lit.to_string()))
            }
        },
    )(i)
}
