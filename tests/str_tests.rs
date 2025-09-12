use support::Strings;

#[cfg(test)]
mod str_tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!("support".reverse(), "troppus");
    }
}

