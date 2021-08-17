use super::*;

fn term(s: &str) -> Token {
    Token::T(String::from(s).chars().collect())
}
fn cterm(s: &str) -> crate::chomsky::Definition {
    crate::chomsky::Definition::Term(String::from(s).chars().collect())
}

#[test]
fn sanity_check() {
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![term("foo")]]],
    };
    g.simplify();
    assert_eq!(g.rules, vec![vec![vec![term("foo")]]]);
}

#[test]
fn remove_duplicate_definitions() {
    let mut g = Grammar {
        start: 0,
        rules: Vec::new(),
    };

    let def1 = vec![vec![term("hello")]];

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
        rules: vec![vec![vec![term("hello"), term(" "), term("world")]]],
    };
    g.simplify();

    assert_eq!(g.rules, vec![vec![vec![term("hello world")]]]);
}

#[test]
fn normalize_sequence() {
    let r1 = vec![vec![Token::NT(1), Token::NT(2), Token::NT(3)]];
    let r2 = vec![vec![term("a")]];
    let r3 = vec![vec![term("b")]];
    let r4 = vec![vec![term("c")]];

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
    use crate::chomsky::Grammar as CGram;
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![], vec![term("hello")]]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: true,
            rules: vec![vec![cterm("hello")]]
        })
    );
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![term("hello")]]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: false,
            rules: vec![vec![cterm("hello")]]
        })
    );
}

#[test]
fn normalize_optional_alternative() {
    use crate::chomsky::Grammar as CGram;
    let mut g = Grammar {
        start: 0,
        rules: vec![vec![vec![], vec![term("hello")], vec![term("world")]]],
    };
    g.normalize();
    assert_eq!(
        CGram::from_normalized(&g),
        Ok(CGram {
            start: 0,
            null: true,
            rules: vec![vec![cterm("hello"), cterm("world"),]]
        })
    )
}
