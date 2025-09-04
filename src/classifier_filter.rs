use std::collections::HashSet;
use growable_bloom_filter::GrowableBloom;

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
}
