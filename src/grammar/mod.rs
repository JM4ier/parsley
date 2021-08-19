use std::fmt::{self, Display, Formatter};

pub type NonTerminal = usize;
pub type Terminal = Vec<char>;
pub type TerminalRef<'a> = &'a [char];

#[cfg(test)]
mod test;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Token {
    NT(NonTerminal),
    T(Terminal),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NT(nt) => write!(f, "{}", nt),
            Self::T(t) => write!(f, "{:?}", t),
        }
    }
}

impl Token {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::T(_))
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Grammar {
    pub start: usize,
    pub rules: Vec<Rule>,
}

impl Display for Grammar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Start: {}", self.start)?;
        for (idx, rule) in self.rules.iter().enumerate() {
            write!(f, "{:>3 } -> ", idx)?;
            let mut first = true;
            for def in rule.iter() {
                if !first {
                    write!(f, " | ")?;
                }
                first = false;
                let mut first_token = true;
                for token in def.iter() {
                    if !first_token {
                        write!(f, " ")?;
                    }
                    first_token = false;
                    match token {
                        Token::NT(nt) => write!(f, "{}", nt)?,
                        Token::T(t) => write!(f, "'{}'", t.iter().cloned().collect::<String>())?,
                    }
                }
                if first_token {
                    write!(f, "\"\"")?;
                }
            }
            if first {
                write!(f, "undefined")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grammar {
    pub fn new() -> Self {
        Default::default()
    }
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
        self.remove_cycles();
        self.n_unit();
        self.simplify();
    }

    /// adds new starting point to avoid any rule producing the starting nonterminal
    fn n_start(&mut self) {
        let new_start = vec![vec![Token::NT(self.start)]];
        self.start = self.add_rule(new_start)
    }

    /// puts every terminal into its own definition
    fn n_term(&mut self) {
        for r in 0..self.rules.len() {
            for d in 0..self.rules[r].len() {
                if self.rules[r][d].len() < 2 {
                    continue;
                }
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

    /// breaks up any definition that contains more than two tokens
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
                    self.rules[r][d].resize(2, Token::T(Default::default()));
                }
            }
        }
    }

    /// eliminates all null productions from any nonterminal except the start
    fn n_del(&mut self) {
        // stores whether a nonterminal can produce the empty string
        let mut nullable = vec![false; self.rules.len()];

        // look up of which nonterminals depend on the i-th nonterminal
        let mut rev = vec![Vec::new(); self.rules.len()];

        for (idx, rule) in self.rules.iter().enumerate() {
            for tok in rule.iter().flatten() {
                if let Token::NT(nt) = tok {
                    rev[*nt].push(idx);
                }
            }
        }

        // remove duplicates
        for r in rev.iter_mut() {
            r.sort_unstable();
            r.dedup();
        }

        // first mark trivially nullable nonterminals as nullable
        let mut q = Vec::new();
        for (idx, rule) in self.rules.iter().enumerate() {
            if rule.contains(&Vec::new()) {
                // there is at least one definition containing no tokens, i.e. that definition
                // produces the empty string
                nullable[idx] = true;

                // mark all depending nonterminals for further inspection
                for dep in rev[idx].iter() {
                    q.push(*dep);
                }
            }
        }

        while let Some(idx) = q.pop() {
            if nullable[idx] {
                continue;
            }
            // if all tokens in one definition are nullable, then this nonterminal is also nullable
            let null = self.rules[idx].iter().any(|def| {
                def.iter().all(|t| match t {
                    Token::NT(nt) => nullable[*nt],
                    _ => false,
                })
            });
            if null {
                nullable[idx] = true;
                for &next in rev[idx].iter() {
                    if !nullable[next] {
                        q.push(next)
                    }
                }
            }
        }

        // after finding all nullable nonterminals, eliminate null productions everywhere except
        // the start
        for (idx, rule) in self.rules.iter_mut().enumerate() {
            let mut new_defs = Vec::new();
            for def in rule.iter() {
                // indices of nullable nonterminals in the definition
                let mut nulls = Vec::new();
                for (i, token) in def.iter().enumerate() {
                    if let Token::NT(nt) = token {
                        if nullable[*nt] {
                            nulls.push(i)
                        }
                    }
                }

                // find all possible combinations of leaving out nullable nonterminals and add as
                // new definitions
                for k in 0..((1 << nulls.len()) - 1) {
                    let mut def = def.clone();
                    for i in (0..nulls.len()).rev() {
                        if (1 << i) & k == 0 {
                            def.remove(nulls[i]);
                        }
                    }
                    new_defs.push(def);
                }
            }

            // add new rule combinations to the actual rule and remove duplicates
            rule.append(&mut new_defs);
            rule.sort();
            rule.dedup();

            // remove null productions unless it is the starting rule
            if idx != self.start {
                rule.retain(|def| !def.is_empty());
            }
        }
    }

    /// eliminates all unit productions of the form `A -> B` by adding the definitions of `B` to `A`
    fn n_unit(&mut self) {
        for r in 0..self.rules.len() {
            let mut i = 0;
            let mut removal = Vec::new();
            while i < self.rules[r].len() {
                if self.rules[r][i].len() == 1 {
                    if let Token::NT(nt) = self.rules[r][i][0] {
                        let mut new_rules = self.rules[nt].clone();
                        self.rules[r].append(&mut new_rules);
                        removal.push(i);
                    }
                }
                i += 1;
            }
            for i in removal.into_iter().rev() {
                self.rules[r].remove(i);
            }
        }
    }
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
                    visited[nt] = false;
                    return true;
                }
            }
            visited[nt] = false;
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
                .filter(|def| *def != vec![Token::NT(rule_idx)])
                .collect();
        }
    }

    fn concatenate_strings(&mut self) {
        for rule in self.rules.iter_mut() {
            for def in rule.iter_mut() {
                let mut acc = Vec::default();
                let mut new_def = Vec::new();
                for tok in def.iter() {
                    match tok {
                        Token::T(term) => {
                            acc.append(&mut term.clone());
                        }
                        nt => {
                            new_def.push(Token::T(acc));
                            acc = Default::default();
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
                *def = def.clone().into_iter().filter(|t| !t.is_empty()).collect();
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
