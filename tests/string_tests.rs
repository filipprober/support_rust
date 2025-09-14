use support::Strings;

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn after() {
        assert_eq!("nah", String::from("hannah").after("han"));
        assert_eq!("nah", String::from("hannah").after("n"));
        assert_eq!("nah", String::from("ééé hannah").after("han"));
        assert_eq!("hannah", String::from("hannah").after("xxxx"));
        assert_eq!("hannah", String::from("hannah").after(""));
        assert_eq!("nah", String::from("han0nah").after("0"));
        assert_eq!("nah", String::from("han0nah").after(0));
        assert_eq!("nah", String::from("han2nah").after(2));
    }

    #[test]
    fn after_last() {
        assert_eq!("tte", String::from("yvette").after_last("yve"));
        assert_eq!("e", String::from("yvette").after_last("t"));
        assert_eq!("e", String::from("ééé yvette").after_last("t"));
        assert_eq!("", String::from("yvette").after_last("tte"));
        assert_eq!("yvette", String::from("yvette").after_last("xxxx"));
        assert_eq!("yvette", String::from("yvette").after_last(""));
        assert_eq!("te", String::from("yv0et0te").after_last("0"));
        assert_eq!("te", String::from("yv0et0te").after_last(0));
        assert_eq!("te", String::from("yv2et2te").after_last(2));
        assert_eq!("foo", String::from("----foo").after_last("---"));
    }

    #[test]
    fn before() {
        assert_eq!("han", String::from("hannah").before("nah"));
        assert_eq!("ha", String::from("hannah").before("n"));
        assert_eq!("ééé", String::from("ééé").before("han"));
        assert_eq!("hannah", String::from("hannah").before("xxxx"));
        assert_eq!("hannah", String::from("hannah").before(""));
        assert_eq!("han", String::from("han0nah").before("0"));
        assert_eq!("han", String::from("han0nah").before(0));
        assert_eq!("han", String::from("han2nah").before(2));
        assert_eq!("", "".before(""));
        assert_eq!("", String::from("a").before("a"));
        assert_eq!("", String::from("a").before("a"));
        assert_eq!("foo", String::from("foo@bar.com").before("@"));
        assert_eq!("foo", String::from("foo@@bar.com").before("@"));
        assert_eq!("", String::from("@foo@bar.com").before("@"));
    }

    #[test]
    fn before_last() {
        assert_eq!("yve", String::from("yvette").before_last("tte"));
        assert_eq!("yvet", String::from("yvette").before_last("t"));
        assert_eq!("ééé ", String::from("ééé yvette").before_last("yve"));
        assert_eq!("", String::from("yvette").before_last("yve"));
        assert_eq!("yvette", String::from("yvette").before_last("xxxx"));
        assert_eq!("yvette", String::from("yvette").before_last(""));
        assert_eq!("yv0et", String::from("yv0et0te").before_last("0"));
        assert_eq!("yv0et", String::from("yv0et0te").before_last(0));
        assert_eq!("yv2et", String::from("yv2et2te").before_last(2));
        assert_eq!("", String::from("").before_last("test"));
        assert_eq!("", String::from("yvette").before_last("yvette"));
        assert_eq!("support", String::from("support package").before_last(" "));
        assert_eq!("yvette", String::from("yvette\tyv0et0te").before_last("\t"));
    }

    #[test]
    fn between() {
        assert_eq!("abc", String::from("abc").between("", "c"));
        assert_eq!("abc", String::from("abc").between("a", ""));
        assert_eq!("abc", String::from("abc").between("", ""));
        assert_eq!("b", String::from("abc").between("a", "c"));
        assert_eq!("b", String::from("dddabc").between("a", "c"));
        assert_eq!("b", String::from("abcddd").between("a", "c"));
        assert_eq!("b", String::from("dddabcddd").between("a", "c"));
        assert_eq!("nn", String::from("hannah").between("ha", "ah"));
        assert_eq!("a]ab[b", String::from("[a]ab[b]").between("[", "]"));
        assert_eq!("foo", String::from("foofoobar").between("foo", "bar"));
        assert_eq!("bar", String::from("foobarbar").between("foo", "bar"));
        assert_eq!("234", String::from("12345").between(1, 5));
        assert_eq!("45", String::from("123456789").between("123", "6789"));
        assert_eq!("nothing", String::from("nothing").between("foo", "bar"));
    }

    #[test]
    fn between_first() {
        assert_eq!("abc", String::from("abc").between_first("", "c"));
        assert_eq!("abc", String::from("abc").between_first("a", ""));
        assert_eq!("abc", String::from("abc").between_first("", ""));
        assert_eq!("b", String::from("abc").between_first("a", "c"));
        assert_eq!("b", String::from("dddabc").between_first("a", "c"));
        assert_eq!("b", String::from("abcddd").between_first("a", "c"));
        assert_eq!("b", String::from("dddabcddd").between_first("a", "c"));
        assert_eq!("nn", String::from("hannah").between_first("ha", "ah"));
        assert_eq!("a", String::from("[a]ab[b]").between_first("[", "]"));
        assert_eq!("foo", String::from("foofoobar").between_first("foo", "bar"));
        assert_eq!("", String::from("foobarbar").between_first("foo", "bar"));
    }

    #[test]
    fn kebab() {
        assert_eq!("support-package", String::from("SupportPackage").kebab());
        assert_eq!("support-package", String::from("Support Package").kebab());
        assert_eq!("support❤-package", String::from("Support ❤ Package").kebab());
        assert_eq!("", String::from("").kebab());
    }

    #[test]
    fn lcfirst() {
        assert_eq!("", String::from("").lcfirst());
        assert_eq!("support", String::from("Support").lcfirst());
        assert_eq!("rust support", String::from("Rust support").lcfirst());
        assert_eq!("мама", String::from("Мама").lcfirst());
        assert_eq!("мама мыла раму", String::from("Мама мыла раму").lcfirst());
    }

    #[test]
    fn lower() {
        assert_eq!("foo bar baz", String::from("FOO BAR BAZ").lower());
        assert_eq!("foo bar baz", String::from("foO bAr BaZ").lower());
    }

    #[test]
    fn plural() {
        assert_eq!("cats", String::from("cat").plural());
        assert_eq!("Dogs", String::from("Dog").plural());
        assert_eq!("Tomatoes", String::from("Tomato").plural());
        assert_eq!("UserGroups", String::from("UserGroup").plural());
        assert_eq!("ProductCategories", String::from("ProductCategory").plural());
    }

    #[test]
    fn reverse() {
        assert_eq!("FooBar", String::from("raBooF").reverse());
        assert_eq!("Teniszütő", String::from("őtüzsineT").reverse());
        assert_eq!("❤MultiByte☆", String::from("☆etyBitluM❤").reverse());
    }

    #[test]
    fn singular() {
        assert_eq!("cat", String::from("cats").singular());
        assert_eq!("Dog", String::from("Dogs").singular());
        assert_eq!("Tomato", String::from("Tomatoes").singular());
        assert_eq!("UserGroup", String::from("UserGroups").singular());
        assert_eq!("ProductCategory", String::from("ProductCategories").singular());
    }

    #[test]
    fn snake() {
        assert_eq!("r_u_s_t_support", String::from("RUSTSupport").snake());
        assert_eq!("rust_support", String::from("RustSupport").snake());
        assert_eq!("rust_support", String::from("Rust   Support").snake());

        assert_eq!("foo-bar", String::from("foo-bar").snake());
        assert_eq!("foo-_bar", String::from("Foo-Bar").snake());
        assert_eq!("foo__bar", String::from("Foo_Bar").snake());
        assert_eq!("żółtałódka", String::from("ŻółtaŁódka").snake());
    }

    #[test]
    fn snake_with_delimeter() {
        assert_eq!("r-u-s-t-support", String::from("RUSTSupport").snake_with_delimeter("-"));
        assert_eq!("rust/support", String::from("RustSupport").snake_with_delimeter("/"));
        assert_eq!("rust&support", String::from("Rust   Support").snake_with_delimeter("&"));

        assert_eq!("foo-bar", String::from("foo-bar").snake_with_delimeter("."));
        assert_eq!("foo-.bar", String::from("Foo-Bar").snake_with_delimeter("."));
        assert_eq!("foo_.bar", String::from("Foo_Bar").snake_with_delimeter("."));
        assert_eq!("żółtałódka", String::from("ŻółtaŁódka").snake_with_delimeter("."));
    }

    #[test]
    fn studly() {
        assert_eq!("RustSUPPORTLibrary", String::from("rust_s_u_p_p_o_r_t_library").studly());
        assert_eq!("RustSupportLibrary", String::from("rust_support_library").studly());
        assert_eq!("RustSupporTLibrary", String::from("rust-supporT-library").studly());
        assert_eq!("RustSupportLibrary", String::from("rust  -_- support  -_- library  ").studly());

        assert_eq!("FooBar", String::from("fooBar").studly());
        assert_eq!("FooBar", String::from("foo_bar").studly());
        assert_eq!("FooBarBaz", String::from("foo-barBaz").studly());
        assert_eq!("FooBarBaz", String::from("foo-bar_baz").studly());

        assert_eq!("ÖffentlicheÜberraschungen", String::from("öffentliche-überraschungen").studly());
    }

    #[test]
    fn take() {
        assert_eq!("ab", String::from("abcdef").take(2));
        assert_eq!("ef", String::from("abcdef").take(-2));
        assert_eq!("", String::from("abcdef").take(0));
        assert_eq!("", String::from("").take(2));
        assert_eq!("abcdef", String::from("abcdef").take(10));
        assert_eq!("ü", String::from("üöä").take(1));
    }

    #[test]
    fn ucfirst() {
        assert_eq!("", String::from("").ucfirst());
        assert_eq!("Support", String::from("support").ucfirst());
        assert_eq!("Rust support", String::from("rust support").ucfirst());
        assert_eq!("Мама", String::from("мама").ucfirst());
        assert_eq!("Мама мыла раму", String::from("мама мыла раму").ucfirst());
    }

    #[test]
    fn upper() {
        assert_eq!("FOO BAR BAZ", String::from("foo bar baz").upper());
        assert_eq!("FOO BAR BAZ", String::from("fOo BaR bAz").upper());
    }
}


