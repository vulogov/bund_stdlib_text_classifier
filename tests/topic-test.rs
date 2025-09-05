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

    #[test]
    fn test_filter_train_from_file_and_guess() {
        let mut tc = TopicClassifier::new();
        tc.train_from_file("tolstoy".to_string(), "./tests/tolstoy.txt".to_string());
        tc.train_from_file("kant".to_string(), "./tests/kant.txt".to_string());
        tc.train_from_file("rust".to_string(), "./tests/rust.txt".to_string());
        tc.finish();
        println!("{:?}", tc.classify("At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code".to_string()));
    }
}
