#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::topic_classifier::TopicClassifier;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_filter_train_and_guess() {
        let mut tc = TopicClassifier::new();
        tc.train("water".to_string(), "ocean river bog lake".to_string());
        tc.train("land".to_string(), "hill mountain road field".to_string());
        tc.finish();
        println!("{:?}", tc.classify("road near mountain through the field".to_string()));
        println!("{:?}", tc.classify("mountain near lake".to_string()));
    }
}
