use support::Strings;

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!("support".to_string().reverse(), "troppus");
    }
}


