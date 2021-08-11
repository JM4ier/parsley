#[cfg(test)]
use super::*;
#[cfg(test)]
use rand::prelude::*;

#[test]
fn alphanumeric_literals() {
    assert_eq!(lit("abc"), Ok(("", BnfPart::Literal("abc".to_string()))));
    assert_eq!(lit("123"), Ok(("", BnfPart::Literal("123".to_string()))));
    assert_eq!(
        lit("abc123"),
        Ok(("", BnfPart::Literal("abc123".to_string())))
    );
}

#[test]
fn literal_choice2() {
    assert_eq!(
        bnf_rule_def("a|b"),
        Ok((
            "",
            BnfPart::Choice(vec!(
                BnfPart::Literal("a".to_string()),
                BnfPart::Literal("b".to_string()),
            ))
        ))
    )
}

#[test]
fn literal_choice3() {
    assert_eq!(
        bnf_rule_def("a|b|c"),
        Ok((
            "",
            BnfPart::Choice(vec!(
                BnfPart::Literal("a".to_string()),
                BnfPart::Literal("b".to_string()),
                BnfPart::Literal("c".to_string()),
            ))
        ))
    )
}

#[test]
fn basic_grouping() {
    assert_eq!(
        bnf_rule_def("(abc)"),
        Ok(("", BnfPart::Literal("abc".to_string())))
    );
}

#[test]
fn big_grouping() {
    assert_eq!(
        bnf_rule_def("((((((abc))))))"),
        Ok(("", BnfPart::Literal("abc".to_string())))
    );
}

#[test]
fn concat_grouping() {
    assert_eq!(
        bnf_rule_def("(a)(b)"),
        Ok((
            "",
            BnfPart::Concat(vec![
                BnfPart::Literal("a".to_string()),
                BnfPart::Literal("b".to_string())
            ])
        ))
    );
}

#[test]
fn choice_grouping() {
    assert_eq!(
        bnf_rule_def("(a|b)|c"),
        Ok((
            "",
            BnfPart::Choice(vec!(
                BnfPart::Choice(vec!(
                    BnfPart::Literal("a".to_string()),
                    BnfPart::Literal("b".to_string())
                )),
                BnfPart::Literal("c".to_string()),
            ))
        ))
    )
}

#[test]
fn instantiation() {
    assert_eq!(
        bnf_rule_def("<rule>"),
        Ok(("", BnfPart::Rule("rule".to_string())))
    )
}

#[test]
fn repetition() {
    assert_eq!(
        bnf_rule_def("{<rule>}"),
        Ok((
            "",
            BnfPart::Repeat(Box::new(BnfPart::Rule("rule".to_string())))
        ))
    )
}

#[test]
fn known_timeouts() {
    let test_this = |bnf: &str| {
        let bnf2 = bnf.to_string();
        timeout_test(0.1, bnf, move || {
            bnf_rule_def(&bnf2);
        })
    };
    test_this("[({([(<c>)])})]");
    test_this("((((((((((a))))))))))");
    test_this("[([([([(<a>)])])])]");
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

    for _ in 0..100 {
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
                assert_eq!(bnf_rule_def(&product.0), Ok(("", product.1.clone())));
            });
        }

        if product.0.len() < 15 {
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

    sleep(Duration::from_secs_f64(timeout));

    match rx.try_recv() {
        Ok(_) => handle.join().unwrap(),
        Err(_) => panic!("Experienced timeout: {}", timeout_msg),
    };
}
