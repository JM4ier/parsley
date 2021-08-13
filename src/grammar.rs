pub type NonTerminal = usize;
pub type Terminal = String;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    NT(NonTerminal),
    T(Terminal),
}

impl Token {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::T(_) => true,
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        if let Self::T(t) = self {
            if t.len() == 0 {
                return true;
            }
        }
        false
    }
}

pub type Definition = Vec<Token>;
pub type Rule = Vec<Definition>;

#[derive(Clone, PartialEq, Eq)]
pub struct Grammar {
    pub start: usize,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn add_rule(&mut self, rule: Rule) -> NonTerminal {
        self.rules.push(rule);
        self.rules.len() - 1
    }
}

// Normalization
impl Grammar {
    /// Brings this grammar into CNF form
    pub fn normalize(&mut self) {
        self.simplify();
        self.n_start();
        self.n_term();
        self.n_bin();
        self.n_del();
        self.n_unit();
        self.simplify();
    }
    fn n_start(&mut self) {
        let new_start = vec![vec![Token::NT(self.start)]];
        self.start = self.add_rule(new_start)
    }
    fn n_term(&mut self) {
        for r in 0..self.rules.len() {
            for d in 0..self.rules[r].len() {
                for t in 0..self.rules[r][d].len() {
                    if let Token::T(term) = &self.rules[r][d][t] {
                        let term = term.clone();
                        let new_rule = vec![vec![Token::T(term)]];
                        let new_rule = self.add_rule(new_rule);
                        self.rules[r][d][t] = Token::NT(new_rule);
                    }
                }
            }
        }
    }
    fn n_bin(&mut self) {
        for r in 0..self.rules.len() {
            for d in 0..self.rules[r].len() {
                let len = self.rules[r][d].len();
                if len > 2 {
                    let mut snek = self.add_rule(vec![vec![
                        self.rules[r][d][len - 2].clone(),
                        self.rules[r][d][len - 1].clone(),
                    ]]);
                    let mut i = len - 2;
                    while i > 1 {
                        i -= 1;
                        let x = self.rules[r][d][i].clone();
                        snek = self.add_rule(vec![vec![x, Token::NT(snek)]]);
                    }
                    self.rules[r][d][1] = Token::NT(snek);
                }
            }
        }
    }

    fn n_del(&mut self) {
        let mut nullable = vec![false; self.rules.len()];
        let mut nullable_known = vec![false; self.rules.len()];
    }
    fn n_unit(&mut self) {}
}

// simplifications
impl Grammar {
    pub fn simplify(&mut self) {
        self.concatenate_strings();
        self.remove_empty_strings();
        self.dedup();
        self.remove_cycles();
        self.flatten_impossible();
        self.remove_unreachable();
    }

    fn dedup(&mut self) {
        for rule in self.rules.iter_mut() {
            rule.sort();
            rule.dedup();
        }
    }

    fn flatten_impossible(&mut self) {
        fn is_possible(rules: &[Rule], visited: &mut [bool], nt: NonTerminal) -> bool {
            if visited[nt] {
                return false;
            }
            visited[nt] = true;
            for def in rules[nt].iter() {
                if def.iter().all(|t| match t {
                    Token::T(_) => true,
                    Token::NT(nt) => is_possible(rules, visited, *nt),
                }) {
                    return true;
                }
            }
            false
        }

        let possible = (0..self.rules.len())
            .map(|i| is_possible(&self.rules, &mut vec![false; self.rules.len()], i))
            .collect::<Vec<_>>();

        for (idx, rule) in self.rules.iter_mut().enumerate() {
            if !possible[idx] {
                *rule = vec![];
            }
        }
    }

    fn remove_cycles(&mut self) {
        for (rule_idx, rule) in self.rules.iter_mut().enumerate() {
            *rule = rule
                .iter()
                .cloned()
                .filter(|def| *def == vec![Token::NT(rule_idx)])
                .collect();
        }
    }

    fn concatenate_strings(&mut self) {
        for rule in self.rules.iter_mut() {
            for def in rule.iter_mut() {
                let mut acc = String::new();
                let mut new_def = Vec::new();
                for tok in def.iter() {
                    match tok {
                        Token::T(term) => {
                            acc += &term;
                        }
                        nt => {
                            new_def.push(Token::T(acc));
                            acc = String::new();
                            new_def.push(nt.clone())
                        }
                    }
                }
                new_def.push(Token::T(acc));
                *def = new_def
            }
        }
    }

    fn remove_empty_strings(&mut self) {
        for rule in self.rules.iter_mut() {
            for def in rule.iter_mut() {
                *def = def.clone().into_iter().filter(|t| t.is_empty()).collect();
            }
        }
    }

    fn remove_unreachable(&mut self) {
        let mut reachable = vec![false; self.rules.len()];
        let mut q = Vec::new();

        reachable[self.start] = true;
        q.push(self.start);

        while let Some(rule) = q.pop() {
            for def in self.rules[rule].iter() {
                for tok in def.iter() {
                    if let Token::NT(nt) = *tok {
                        if !reachable[nt] {
                            reachable[nt] = true;
                            q.push(nt);
                        }
                    }
                }
            }
        }

        let mut offsets = vec![0; self.rules.len()];
        let mut offset = 0;

        for i in 0..self.rules.len() {
            if reachable[i] {
                offsets[i] = offset;
                offset += 1;
            }
        }

        for i in 0..self.rules.len() {
            if reachable[i] {
                self.rules.push(Rule::new());
                let mut moved = self.rules.swap_remove(i);
                for def in moved.iter_mut() {
                    for tok in def.iter_mut() {
                        if let Token::NT(nt) = *tok {
                            *tok = Token::NT(offsets[nt]);
                        }
                    }
                }
                self.rules[offsets[i]] = moved;
            }
        }

        self.rules.resize(offset, Rule::new());
        self.start = offsets[self.start];
    }
}
