#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_library1() {
        let bund = Bund::new();
        assert!(init_lib(&bund));
    }
}
