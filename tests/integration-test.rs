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

    #[test]
    fn test_register_classifier_train_and_finish() {
        let script = r#"
        :TEST textclassifier.new
        :rust './tests/rust.txt' textclassifier.train.from_file
        "{A} tokens for RUST" format println
        :kant './tests/kant.txt' textclassifier.train.from_file
        "{A} tokens for KANT" format println
        textclassifier.finish
        "#;
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        let _ = bund.eval(script);
    }

    #[test]
    fn test_register_classifier_train_and_finish_and_classify() {
        let script = r#"
        :TEST textclassifier.new
            :rust "./tests/rust.txt" textclassifier.train.from_file
            "{A} tokens for RUST" format println
            :kant "./tests/kant.txt" textclassifier.train.from_file
            "{A} tokens for KANT" format println
            :astronomy "./tests/astronomy.txt" textclassifier.train.from_file
            "{A} tokens for ASTRONOMY" format println
            :tolstoy "./tests/tolstoy.txt" textclassifier.train.from_file
            "{A} tokens for LEO TOLSTOY" format println
            textclassifier.train.finish
        "At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code"
            textclassifier.classify
        "#;
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        match bund.eval(script) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", &err);
            }
        }
        let res = bund.vm.stack.pull();
        let rust_res = res.unwrap().get("rust").unwrap().cast_float().unwrap();
        assert_eq!(rust_res, 1.0 as f64)
    }

}
