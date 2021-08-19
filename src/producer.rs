use crate::chomsky::*;

type Terminals = Vec<Terminal>;

/// An iterator over words of a grammar
///
/// It returns words ordered by length, and then the usual order of Strings
pub struct Producer {
    /// the underlying grammar that produces words
    grammar: Grammar,
    /// words already found, by length
    words: Vec<Vec<Terminals>>,
    /// what word to send next out of the ones with max length
    next: usize,
}

impl Producer {
    /// builds a new producer from a given grammar
    pub fn new(grammar: Grammar) -> Self {
        let mut words = vec![vec![vec![]; grammar.rules.len()]];
        if grammar.null {
            words[0][grammar.start].push(Terminal::new())
        }
        Self {
            words,
            grammar,
            next: 0,
        }
    }
    /// finds all words with next bigger size
    fn find_next_words(&mut self) {
        // length the new found words should have
        let target_len = self.words.len();
        self.words.push(vec![vec![]; self.grammar.rules.len()]);

        for (r, rule) in self.grammar.rules.iter().enumerate() {
            let mut new_words = Vec::new();

            // add terminals to found words, if they have the correct length
            for def in rule.iter() {
                if let Definition::Term(t) = def {
                    if t.len() == target_len {
                        new_words.push(t.clone());
                    }
                }
            }

            // add combinations of two nonterminals to the list of words, if the two parts together
            // are the correct length
            for l1 in 1..target_len {
                let l2 = target_len - l1;
                for def in rule.iter() {
                    if let Definition::Product([nta, ntb]) = *def {
                        for a in self.words[l1][nta].iter() {
                            for b in self.words[l2][ntb].iter() {
                                let mut a = a.clone();
                                let mut b = b.clone();
                                a.append(&mut b);
                                new_words.push(a)
                            }
                        }
                    }
                }
            }

            // remove duplicates & sort
            new_words.sort();
            new_words.dedup();
            self.words[target_len][r] = new_words;
        }

        self.next = 0;
    }

    /// returns all currently produced words, even if they haven't been read by the iterator yet
    pub fn all_buffered_words(&self) -> Vec<Terminal> {
        self.words
            .iter()
            .map(|w| &w[self.grammar.start])
            .flatten()
            .cloned()
            .collect()
    }

    /// length of the current longest found word
    fn current_longest(&self) -> usize {
        for (len, words) in self.words.iter().enumerate().rev() {
            if !words[self.grammar.start].is_empty() {
                return len;
            }
        }
        1_000_000
    }
    /// whether it's possible that there are more words still
    fn finished(&self) -> bool {
        self.words.len() > 2 * self.current_longest() + 1
    }
    /// reference to the longest currently found words
    fn long_words(&self) -> &[Terminal] {
        &self.words.last().unwrap()[self.grammar.start]
    }
}

impl Iterator for Producer {
    type Item = Terminal;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished() && self.next >= self.long_words().len() {
            self.find_next_words();
        }
        if self.next < self.long_words().len() {
            self.next += 1;
            Some(self.long_words()[self.next - 1].clone())
        } else {
            None
        }
    }
}
