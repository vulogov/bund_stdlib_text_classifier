#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::classifier_filter::ClassifierFilter;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_filter_add_true() {
        let mut filter = ClassifierFilter::new();
        assert_eq!(filter.add("test"), true);
    }

    #[test]
    fn test_filter_add_false() {
        let mut filter = ClassifierFilter::new();
        filter.add("test");
        assert_eq!(filter.add("test"), false);
    }
    #[test]
    fn test_filter_to_string() {
        let mut filter = ClassifierFilter::new();
        filter.add("test");
        filter.add("test2");
        assert_eq!(filter.to_string(), "test2 test");
    }
}
