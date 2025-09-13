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
    fn between() {
        assert_eq!("abc", "abc".between("", "c"));
        assert_eq!("abc", "abc".between("a", ""));
        assert_eq!("abc", "abc".between("", ""));
        assert_eq!("b", "abc".between("a", "c"));
        assert_eq!("b", "dddabc".between("a", "c"));
        assert_eq!("b", "abcddd".between("a", "c"));
        assert_eq!("b", "dddabcddd".between("a", "c"));
        assert_eq!("nn", "hannah".between("ha", "ah"));
        assert_eq!("a]ab[b", "[a]ab[b]".between("[", "]"));
        assert_eq!("foo", "foofoobar".between("foo", "bar"));
        assert_eq!("bar", "foobarbar".between("foo", "bar"));
        assert_eq!("234", "12345".between(1, 5));
        assert_eq!("45", "123456789".between("123", "6789"));
        assert_eq!("nothing", "nothing".between("foo", "bar"));
    }

    #[test]
    fn between_first() {
        assert_eq!("abc", "abc".between_first("", "c"));
        assert_eq!("abc", "abc".between_first("a", ""));
        assert_eq!("abc", "abc".between_first("", ""));
        assert_eq!("b", "abc".between_first("a", "c"));
        assert_eq!("b", "dddabc".between_first("a", "c"));
        assert_eq!("b", "abcddd".between_first("a", "c"));
        assert_eq!("b", "dddabcddd".between_first("a", "c"));
        assert_eq!("nn", "hannah".between_first("ha", "ah"));
        assert_eq!("a", "[a]ab[b]".between_first("[", "]"));
        assert_eq!("foo", "foofoobar".between_first("foo", "bar"));
        assert_eq!("", "foobarbar".between_first("foo", "bar"));
    }

    #[test]
    fn kebab() {
        assert_eq!("support-package", "SupportPackage".kebab());
        assert_eq!("support-package", "Support Package".kebab());
        assert_eq!("support❤-package", "Support ❤ Package".kebab());
        assert_eq!("", "".kebab());
    }

    #[test]
    fn lcfirst() {
        assert_eq!("", "".lcfirst());
        assert_eq!("support", "Support".lcfirst());
        assert_eq!("rust support", "Rust support".lcfirst());
        assert_eq!("мама", "Мама".lcfirst());
        assert_eq!("мама мыла раму", "Мама мыла раму".lcfirst());
    }

    #[test]
    fn lower() {
        assert_eq!("foo bar baz", "FOO BAR BAZ".lower());
        assert_eq!("foo bar baz", "foO bAr BaZ".lower());
    }

    #[test]
    fn plural() {
        assert_eq!("cats", "cat".plural());
        assert_eq!("Dogs", "Dog".plural());
        assert_eq!("Tomatoes", "Tomato".plural());
        assert_eq!("UserGroups", "UserGroup".plural());
        assert_eq!("ProductCategories", "ProductCategory".plural());
    }


    #[test]
    fn reverse() {
        assert_eq!("FooBar", "raBooF".reverse());
        assert_eq!("Teniszütő", "őtüzsineT".reverse());
        assert_eq!("❤MultiByte☆", "☆etyBitluM❤".reverse());
    }

    #[test]
    fn singular() {
        assert_eq!("cat", "cats".singular());
        assert_eq!("Dog", "Dogs".singular());
        assert_eq!("Tomato", "Tomatoes".singular());
        assert_eq!("UserGroup", "UserGroups".singular());
        assert_eq!("ProductCategory", "ProductCategories".singular());
    }

    #[test]
    fn snake() {
        assert_eq!("r_u_s_t_support", "RUSTSupport".snake());
        assert_eq!("rust_support", "RustSupport".snake());
        assert_eq!("rust_support", "Rust   Support".snake());

        assert_eq!("foo-bar", "foo-bar".snake());
        assert_eq!("foo-_bar", "Foo-Bar".snake());
        assert_eq!("foo__bar", "Foo_Bar".snake());
        assert_eq!("żółtałódka", "ŻółtaŁódka".snake());
    }

    #[test]
    fn snake_with_delimeter() {
        assert_eq!("r-u-s-t-support", "RUSTSupport".snake_with_delimeter("-"));
        assert_eq!("rust/support", "RustSupport".snake_with_delimeter("/"));
        assert_eq!("rust&support", "Rust   Support".snake_with_delimeter("&"));

        assert_eq!("foo-bar", "foo-bar".snake_with_delimeter("."));
        assert_eq!("foo-.bar", "Foo-Bar".snake_with_delimeter("."));
        assert_eq!("foo_.bar", "Foo_Bar".snake_with_delimeter("."));
        assert_eq!("żółtałódka", "ŻółtaŁódka".snake_with_delimeter("."));
    }

    #[test]
    fn take() {
        assert_eq!("ab", "abcdef".take(2));
        assert_eq!("ef", "abcdef".take(-2));
        assert_eq!("", "abcdef".take(0));
        assert_eq!("", "".take(2));
        assert_eq!("abcdef", "abcdef".take(10));
        assert_eq!("ü", "üöä".take(1));
    }

    #[test]
    fn ucfirst() {
        assert_eq!("", "".ucfirst());
        assert_eq!("Support", "support".ucfirst());
        assert_eq!("Rust support", "rust support".ucfirst());
        assert_eq!("Мама", "мама".ucfirst());
        assert_eq!("Мама мыла раму", "мама мыла раму".ucfirst());
    }

    #[test]
    fn upper() {
        assert_eq!("FOO BAR BAZ", "foo bar baz".upper());
        assert_eq!("FOO BAR BAZ", "fOo BaR bAz".upper());
    }
}

