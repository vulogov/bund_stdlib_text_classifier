#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::*;
    use bund_stdlib_text_classifier::classifiers::Classifiers;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_get_classifier_train_from_file_and_guess_rust() {
        let mut c = Classifiers::new();
        let tc = c.classifier("TEST".to_string());
        tc.train_from_file("tolstoy".to_string(), "./tests/tolstoy.txt".to_string());
        tc.train_from_file("kant".to_string(), "./tests/kant.txt".to_string());
        tc.train_from_file("rust".to_string(), "./tests/rust.txt".to_string());
        tc.train_from_file("astronomy".to_string(), "./tests/astronomy.txt".to_string());
        tc.finish();
        let res = tc.classify("At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code".to_string());
        assert_eq!(*res.get("rust").unwrap(), 1.0 as f64);
    }

    #[test]
    fn test_get_classifier_train_from_file_and_guess_rust_global() {
        let mut c = TEXTCLASSIFIERS.lock().unwrap();
        let tc = c.classifier("TEST".to_string());
        tc.train_from_file("tolstoy".to_string(), "./tests/tolstoy.txt".to_string());
        tc.train_from_file("kant".to_string(), "./tests/kant.txt".to_string());
        tc.train_from_file("rust".to_string(), "./tests/rust.txt".to_string());
        tc.train_from_file("astronomy".to_string(), "./tests/astronomy.txt".to_string());
        tc.finish();
        let res = tc.classify("At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code".to_string());
        drop(c);
        assert_eq!(*res.get("rust").unwrap(), 1.0 as f64);
    }

}
