use std::collections::HashSet;
use growable_bloom_filter::GrowableBloom;
use charabia::Tokenize;

#[derive(Clone, Debug)]
pub struct ClassifierFilter {
    pub words:      HashSet<String>,
    pub filter:     GrowableBloom,
}

impl ClassifierFilter {
    pub fn new() -> Self {
        Self {
            words:      HashSet::new(),
            filter:     GrowableBloom::new(0.05, 10),
        }
    }
    pub fn add<N: AsRef<str> + std::hash::Hash  + ToString + std::fmt::Display>(&mut self, token: N) -> bool {
        if self.filter.contains(&token.to_string()) {
            return false;
        }
        self.filter.insert(&token.to_string());
        self.words.insert(token.to_string());
        true
    }
    pub fn to_string(&mut self) -> String {
        let mut res = String::new();
        for t in self.words.iter() {
            res = res + &format!("{} ", t).to_string();
        }
        res.trim().to_string()
    }

    pub fn to_vec(&mut self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        for t in self.words.iter() {
            res.push(t.to_string());
        }
        res
    }

    pub fn tokenize<'a, N: AsRef<str> + std::hash::Hash  + ToString + std::fmt::Display + Tokenize<'a>>(&mut self, token: N) -> usize {
        let tokens = token.tokenize();
        for t in tokens {
            if t.is_word() {
                let _ = self.add(&t.lemma());
            }
        }
        self.words.len()
    }
}
