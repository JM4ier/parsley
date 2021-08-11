use super::*;

#[test]
fn lex_a() {
    use Token::*;
    assert_eq!(
        lex("<peter\\ >   : mueller[asdf]\\a\\}"),
        vec![
            RuleOpen,
            String("peter ".into()),
            RuleClose,
            Assign,
            String("mueller".into()),
            OptOpen,
            String("asdf".into()),
            OptClose,
            String("a}".into())
        ]
    )
}
