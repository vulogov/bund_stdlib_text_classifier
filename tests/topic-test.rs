#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::topic_classifier::TopicClassifier;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_topic_train_and_guess() {
        let mut tc = TopicClassifier::new();
        tc.train("water".to_string(), "ocean river bog lake".to_string());
        tc.train("land".to_string(), "hill mountain road field".to_string());
        tc.finish();
        println!("{:?}", tc.classify("road near mountain through the field".to_string()));
        println!("{:?}", tc.classify("mountain near lake".to_string()));
    }

    #[test]
    fn test_topic_train_from_file_and_guess_rust() {
        let mut tc = TopicClassifier::new();
        tc.train_from_file("tolstoy".to_string(), "./tests/tolstoy.txt".to_string());
        tc.train_from_file("kant".to_string(), "./tests/kant.txt".to_string());
        tc.train_from_file("rust".to_string(), "./tests/rust.txt".to_string());
        tc.train_from_file("astronomy".to_string(), "./tests/astronomy.txt".to_string());
        tc.finish();
        let res = tc.classify("At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code".to_string());
        assert_eq!(*res.get("rust").unwrap(), 1.0 as f64);
    }

    #[test]
    fn test_topic_train_from_file_and_guess_astronomy() {
        let mut tc = TopicClassifier::new();
        tc.train_from_file("tolstoy".to_string(), "./tests/tolstoy.txt".to_string());
        tc.train_from_file("kant".to_string(), "./tests/kant.txt".to_string());
        tc.train_from_file("rust".to_string(), "./tests/rust.txt".to_string());
        tc.train_from_file("astronomy".to_string(), "./tests/astronomy.txt".to_string());
        tc.finish();
        let res = tc.classify("Not all stars are the same. Some stars are very cool and dim".to_string());
        assert_eq!(*res.get("astronomy").unwrap(), 1.0 as f64);
    }
}
