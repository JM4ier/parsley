use super::*;
use rand::prelude::*;
use BnfPart::*;

fn full_parse(bnf: &str) -> BnfPart {
    use crate::lex::*;
    let tokens = lex(bnf);
    let mut tokens: &[_] = &tokens;
    rparse(&mut tokens, None).unwrap()
}

#[test]
fn literal_choice2() {
    assert_eq!(
        full_parse("a|b"),
        Choice(vec![Literal("a".into()), Literal("b".into())])
    )
}

#[test]
fn literal_choice3() {
    assert_eq!(
        full_parse("a|b | c"),
        Choice(vec![
            Literal("a".into()),
            Literal("b".into()),
            Literal("c".into())
        ])
    )
}

#[test]
fn instantiation() {
    assert_eq!(full_parse("<rule>"), Rule("rule".into()))
}

#[test]
fn repetition() {
    assert_eq!(
        full_parse("{<rule>}"),
        Repeat(Box::new(Rule("rule".into())))
    )
}

#[test]
fn generative() {
    let lit = |s: &str| (s.to_string(), BnfPart::Literal(s.to_string()));
    let rul = |s: &str| (format!("<{}>", s), BnfPart::Rule(s.to_string()));
    let atoms = vec![
        lit("abc"),
        lit("def"),
        lit("123"),
        rul("a"),
        rul("b"),
        rul("c"),
    ];

    let choice = |v: &[(String, BnfPart)]| {
        let text = v
            .iter()
            .map(|e| &e.0)
            .cloned()
            .collect::<Vec<_>>()
            .join("|");
        let rules = v.iter().map(|e| &e.1).cloned().collect();
        (text, BnfPart::Choice(rules))
    };

    let concat = |v: &[(String, BnfPart)]| {
        let text = v.iter().map(|e| &e.0).cloned().collect();
        let rules = v.iter().map(|e| &e.1).cloned().collect();
        (text, BnfPart::Concat(rules))
    };

    let opt = |v: &[(String, BnfPart)]| {
        let text = format!("[{}]", v[0].0);
        let rule = BnfPart::Opt(v[0].1.clone());
        (text, rule)
    };

    let rep = |v: &[(String, BnfPart)]| {
        let text = format!("{{{}}}", v[0].0);
        let rule = BnfPart::Repeat(Box::new(v[0].1.clone()));
        (text, rule)
    };

    let prod: Vec<&dyn Fn(&[(String, BnfPart)]) -> (String, BnfPart)> =
        vec![&choice, &concat, &opt, &rep];

    let mut elems = atoms.clone();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let combs = 2 + rng.gen::<usize>() % 4;
        let mut vec = Vec::new();
        for _ in 0..combs {
            let elem = &elems[rng.gen::<usize>() % elems.len()];
            vec.push((format!("({})", elem.0), elem.1.clone()));
        }

        let producer = prod[rng.gen::<usize>() % prod.len()];
        let product = producer(&vec);

        {
            let product = product.clone();
            timeout_test(0.1, &product.0.clone(), move || {
                assert_eq!(full_parse(&product.0), product.1.clone());
            });
        }

        if product.0.len() < 100 {
            elems.push(product);
        }
    }
}

#[cfg(test)]
fn timeout_test(
    timeout: f64,
    timeout_msg: &str,
    test: impl 'static + Send + Sync + FnOnce() -> (),
) {
    use std::sync::mpsc::*;
    use std::thread::*;
    use std::time::*;

    let (tx, rx) = channel();

    let handle = spawn(move || {
        test();
        tx.send(()).unwrap();
    });

    match rx.recv_timeout(Duration::from_secs_f64(timeout)) {
        Ok(_) => handle.join().unwrap(),
        Err(_) => panic!("Experienced timeout: {}", timeout_msg),
    };
}

/// tests for crashes in the parser by testing random inputs
#[test]
fn fuzzing() {
    use crate::lex::Token::*;
    use rand::prelude::*;

    let tokens = vec![
        String("x".into()),
        String("y".into()),
        String("z".into()),
        RuleOpen,
        RuleClose,
        GroupOpen,
        GroupClose,
        OptOpen,
        OptClose,
        RepOpen,
        RepClose,
        Alternative,
        Assign,
        Newline,
    ];

    let begin = vec![RuleOpen, String("rule".into()), RuleClose, Assign];
    let empty = vec![];

    let mut rng = rand::thread_rng();
    let max_len = 100;
    let mut _correct = 0;
    let reps = 100_000;

    for i in 0..reps {
        let start = if i % 2 == 0 { &begin } else { &empty };
        let mut input = start.clone();
        for _ in 0..(rng.gen::<usize>() % max_len) {
            input.push(tokens[rng.gen::<usize>() % tokens.len()].clone());
        }
        let input = input
            .into_iter()
            .enumerate()
            .map(|(i, t)| (i..=i, t))
            .collect::<Vec<_>>();

        let dummy_source = Some('*')
            .iter()
            .cycle()
            .take(input.len())
            .collect::<std::string::String>();

        let mut input: &[_] = &input;
        let parsed = parse(&mut input);
        match parsed {
            Ok(_) => {
                _correct += 1;
            }
            Err(errs) => {
                format_errors("fuzz.txt", &dummy_source, errs);
            }
        }
    }
    println!("{}/{} successful parses", _correct, reps);
}
