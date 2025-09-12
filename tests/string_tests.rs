use support::Strings;

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!("FooBar", "raBooF".reverse());
        assert_eq!("Teniszütő", "őtüzsineT".reverse());
        assert_eq!("❤MultiByte☆", "☆etyBitluM❤".reverse());
    }
}


