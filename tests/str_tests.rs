use support::Strings;

#[cfg(test)]
mod str_tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!("FooBar", "raBooF".reverse());
        assert_eq!("Teniszütő", "őtüzsineT".reverse());
        assert_eq!("❤MultiByte☆", "☆etyBitluM❤".reverse());
    }
}

