use support::Strings;

#[cfg(test)]
mod str_tests {
    use super::*;

    #[test]
    fn after() {
        assert_eq!("nah", "hannah".after("han"));
        assert_eq!("nah", "hannah".after("n"));
        assert_eq!("nah", "ééé hannah".after("han"));
        assert_eq!("hannah", "hannah".after("xxxx"));
        assert_eq!("hannah", "hannah".after(""));
        assert_eq!("nah", "han0nah".after("0"));
        assert_eq!("nah", "han0nah".after(0));
        assert_eq!("nah", "han2nah".after(2));
    }

    #[test]
    fn after_last() {
        assert_eq!("tte", "yvette".after_last("yve"));
        assert_eq!("e", "yvette".after_last("t"));
        assert_eq!("e", "ééé yvette".after_last("t"));
        assert_eq!("", "yvette".after_last("tte"));
        assert_eq!("yvette", "yvette".after_last("xxxx"));
        assert_eq!("yvette", "yvette".after_last(""));
        assert_eq!("te", "yv0et0te".after_last("0"));
        assert_eq!("te", "yv0et0te".after_last(0));
        assert_eq!("te", "yv2et2te".after_last(2));
        assert_eq!("foo", "----foo".after_last("---"));
    }

    #[test]
    fn before() {
        assert_eq!("han", "hannah".before("nah"));
        assert_eq!("ha", "hannah".before("n"));
        assert_eq!("ééé", "ééé".before("han"));
        assert_eq!("hannah", "hannah".before("xxxx"));
        assert_eq!("hannah", "hannah".before(""));
        assert_eq!("han", "han0nah".before("0"));
        assert_eq!("han", "han0nah".before(0));
        assert_eq!("han", "han2nah".before(2));
        assert_eq!("", "".before(""));
        assert_eq!("", "a".before("a"));
        assert_eq!("", "a".before("a"));
        assert_eq!("foo", "foo@bar.com".before("@"));
        assert_eq!("foo", "foo@@bar.com".before("@"));
        assert_eq!("", "@foo@bar.com".before("@"));
    }

    #[test]
    fn before_last() {
        assert_eq!("yve", "yvette".before_last("tte"));
        assert_eq!("yvet", "yvette".before_last("t"));
        assert_eq!("ééé ", "ééé yvette".before_last("yve"));
        assert_eq!("", "yvette".before_last("yve"));
        assert_eq!("yvette", "yvette".before_last("xxxx"));
        assert_eq!("yvette", "yvette".before_last(""));
        assert_eq!("yv0et", "yv0et0te".before_last("0"));
        assert_eq!("yv0et", "yv0et0te".before_last(0));
        assert_eq!("yv2et", "yv2et2te".before_last(2));
        assert_eq!("", "".before_last("test"));
        assert_eq!("", "yvette".before_last("yvette"));
        assert_eq!("support", "support package".before_last(" "));
        assert_eq!("yvette", "yvette\tyv0et0te".before_last("\t"));
    }

    #[test]
    fn reverse() {
        assert_eq!("FooBar", "raBooF".reverse());
        assert_eq!("Teniszütő", "őtüzsineT".reverse());
        assert_eq!("❤MultiByte☆", "☆etyBitluM❤".reverse());
    }
}

