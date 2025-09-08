#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::*;
    use bund_stdlib_text_classifier::classifiers::Classifiers;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_register_classifier() {
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        let _ = bund.eval(":TEST textclassifier.new");
    }

    #[test]
    fn test_register_classifier_exists() {
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        let _ = bund.eval(":TEST textclassifier.new textclassifier.exists");
        let res = bund.vm.stack.pull();
        assert_eq!(res.unwrap().cast_bool().unwrap(), true);
    }

    #[test]
    fn test_register_classifier_train_from_file() {
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        let _ = bund.eval(":TEST :rust './tests/rust.txt' textclassifier.train.from_file");
        let res = bund.vm.stack.pull();
        assert_eq!(res.unwrap().cast_int().unwrap(), 88 as i64);
    }

}
