#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_text_classifier::*;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_library1() {
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
    }
}
