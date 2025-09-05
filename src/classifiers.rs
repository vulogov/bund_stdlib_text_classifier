use std::collections::HashMap;

use crate::topic_classifier::TopicClassifier;

pub struct Classifiers {
    pub c:          HashMap<String, TopicClassifier>,
}

impl Classifiers {
    pub fn new() -> Self {
        Self {
            c:         HashMap::<String, TopicClassifier>::new(),
        }
    }

    pub fn classifier(&mut self, name: String) -> &mut TopicClassifier {
        let c = self.c.entry(name).or_insert(TopicClassifier::new());
        c
    }

    pub fn classifier_exists(&mut self, name: String) -> bool {
        self.c.contains_key(&name)
    }
}
