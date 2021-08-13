use super::*;

#[test]
fn sanity_check() {
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![Token::T("foo".into())]]],
    };
    g.simplify();
    assert_eq!(g.rules, vec![vec![vec![Token::T("foo".into())]]]);
}

#[test]
fn remove_duplicate_definitions() {
    let mut g = Grammar {
        start: 0,
        rules: Vec::new(),
    };

    let def1 = vec![vec![Token::T("hello".into())]];

    let mut def4 = def1.clone();
    def4.extend_from_within(..);
    def4.extend_from_within(..);

    g.rules.push(def4);

    g.simplify();

    assert_eq!(g.rules[0], def1);
    assert_eq!(g.rules.len(), 1);
}

#[test]
fn concat_strings() {
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![
            Token::T("hello".into()),
            Token::T(" ".into()),
            Token::T("world".into()),
        ]]],
    };
    g.simplify();

    assert_eq!(g.rules, vec![vec![vec![Token::T("hello world".into())]]]);
}

#[test]
fn normalize_sequence() {
    let r1 = vec![vec![Token::NT(1), Token::NT(2), Token::NT(3)]];
    let r2 = vec![vec![Token::T("a".into())]];
    let r3 = vec![vec![Token::T("b".into())]];
    let r4 = vec![vec![Token::T("c".into())]];

    let mut g = Grammar {
        start: 0,
        rules: vec![r1, r2, r3, r4],
    };

    g.simplify();
    assert_eq!(g.rules.len(), 4);

    g.normalize();
    assert!(g.rules.len() >= 4, "{:?}", g);
    assert!(
        crate::chomsky::Grammar::from_normalized(&g).is_ok(),
        "{:?}",
        g
    );
}

#[test]
fn normalize_optional() {
    use crate::chomsky::{Definition as CDef, Grammar as CGram};
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![], vec![Token::T(String::from("hello"))]]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: true,
            rules: vec![vec![CDef::Term(String::from("hello"))]]
        })
    );
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![Token::T(String::from("hello"))]]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: false,
            rules: vec![vec![CDef::Term(String::from("hello"))]]
        })
    );
}

#[test]
fn normalize_optional_alternative() {
    use crate::chomsky::{Definition as CDef, Grammar as CGram};
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![
            vec![],
            vec![Token::T(String::from("hello"))],
            vec![Token::T(String::from("world"))],
        ]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: true,
            rules: vec![vec![
                CDef::Term(String::from("hello")),
                CDef::Term(String::from("world"))
            ]]
        })
    )
}
