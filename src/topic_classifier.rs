use std::collections::HashMap;
use bayesic::Bayesic;
use std::fs;

use crate::classifier_filter::ClassifierFilter;

pub fn get_file_from_relative_file(full_path: String) -> Option<String> {
    match fs::read_to_string(full_path) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub struct TopicClassifier {
    pub classifier:      Bayesic,
    pub filter:          HashMap<String, ClassifierFilter>,
}

impl TopicClassifier {
    pub fn new() -> Self {
        Self {
            classifier:     Bayesic::new(),
            filter:         HashMap::<String, ClassifierFilter>::new(),
        }
    }

    pub fn train(&mut self, topic: String, data: String) -> usize {
        let cf = self.filter.entry(topic).or_insert(ClassifierFilter::new());
        cf.tokenize(data.as_str())
    }

    pub fn train_from_file(&mut self, topic: String, fname: String) -> usize {
        match get_file_from_relative_file(fname) {
            Some(data) => {
                self.train(topic, data)
            }
            None => { 0 }
        }
    }

    pub fn finish(&mut self) {
        for (topic, filter) in &mut self.filter {
            self.classifier.train(topic.to_string(), filter.to_vec());
        }
    }

    pub fn classify(&mut self, data: String) -> HashMap<String, f64> {
        let mut cf = ClassifierFilter::new();
        cf.tokenize(data.as_str());
        self.classifier.classify(cf.to_vec())
    }
}
