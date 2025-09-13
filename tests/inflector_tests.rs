use support::Inflector;

#[cfg(test)]
mod inflector_tests {
    use super::*;

    #[test]
    fn pluralize() {
        assert_eq!("cats", Inflector::pluralize("cat"));
        assert_eq!("Dogs", Inflector::pluralize("Dog"));
        assert_eq!("Tomatoes", Inflector::pluralize("Tomato"));
        assert_eq!("UserGroups", Inflector::pluralize("UserGroup"));
        assert_eq!("ProductCategories", Inflector::pluralize("ProductCategory"));
    }

    #[test]
    fn singularize() {
        assert_eq!("cat", Inflector::singularize("cats"));
        assert_eq!("Dog", Inflector::singularize("Dogs"));
        assert_eq!("Tomato", Inflector::singularize("Tomatoes"));
        assert_eq!("UserGroup", Inflector::singularize("UserGroups"));
        assert_eq!("ProductCategory", Inflector::singularize("ProductCategories"));
    }
}

