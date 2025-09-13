use std::fmt::Display;
use regex::Regex;

pub trait Strings {
    ///
    /// Return the remainder of a string after the first occurrence of a given value.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".after("Rust ");
    /// // "Support"
    /// ```
    fn after<T: Display>(&self, search: T) -> String;

    ///
    /// Return the remainder of a string after the last occurrence of a given value.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support Rust".after_last("R");
    /// // "ust"
    /// ```
    fn after_last<T: Display>(&self, search: T) -> String;

    ///
    /// Get the portion of a string before the first occurrence of a given value.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".before("Support");
    /// // "Rust "
    /// ```
    fn before<T: Display>(&self, search: T) -> String;

    ///
    /// Get the portion of a string before the last occurrence of a given value.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".before_last("o");
    /// // "Rust Supp"
    /// ```
    fn before_last<T: Display>(&self, search: T) -> String;

    ///
    /// Get the portion of a string between two given values.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".between("Ru", "rt");
    /// // "st Suppo"
    /// ```
    fn between<T: Display>(&self, from: T, to: T) -> String;

    ///
    /// Get the smallest possible portion of a string between two given values.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".between_first("R", "t");
    /// // "us"
    /// ```
    fn between_first<T: Display>(&self, from: T, to: T) -> String;

    ///
    /// Convert a string to kebab case.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".kebab();
    /// // "rust-support"
    /// ```
    fn kebab(&self) -> String;

    ///
    /// Convert the first character of the given string to lower-case.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".lcfirst();
    /// // "rust Support"
    /// ```
    fn lcfirst(&self) -> String;

    ///
    /// Convert the given string to lower-case.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".lower();
    /// // "rust support"
    /// ```
    fn lower(&self) -> String;

    ///
    /// Reverses the given string.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".reverse();
    /// // "troppuS tsuR"
    /// ```
    fn reverse(&self) -> String;

    ///
    /// Convert a string to snake case.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".snake();
    /// // "rust_support"
    /// ```
    fn snake(&self) -> String;

    ///
    /// Convert a string to snake case with the specified delimeter.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".snake_with_delimeter("-");
    /// // "rust-support"
    /// ```
    fn snake_with_delimeter(&self, delimiter: &str) -> String;

    ///
    /// Convert the first character of the given string to upper-case.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "rust support".ucfirst();
    /// // "Rust support"
    /// ```
    fn ucfirst(&self) -> String;

    ///
    /// Convert the given string to upper-case.
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".upper();
    /// // "RUST SUPPORT"
    /// ```
    fn upper(&self) -> String;
}

impl Strings for str {
    fn after<T: Display>(&self, search: T) -> String {
        let search = search.to_string();
        match self.find(&search) {
            Some(index) => {
                let start = index + search.len();
                self[start..].to_string()
            }
            None => self.to_string(),
        }
    }

    fn after_last<T: Display>(&self, search: T) -> String {
        let search = search.to_string();

        if search.is_empty() {
            return self.to_string();
        }

        match self.rfind(&search) {
            Some(index) => {
                let start = index + search.len();
                self[start..].to_string()
            }
            None => self.to_string(),
        }
    }

    fn before<T: Display>(&self, search: T) -> String {
        let search = search.to_string();

        if search.is_empty() {
            return self.to_string();
        }

        match self.find(&search) {
            Some(index) => self[..index].to_string(),
            None => self.to_string(),
        }
    }

    fn before_last<T: Display>(&self, search: T) -> String {
        let search = search.to_string();

        if search.is_empty() {
            return self.to_string();
        }

        match self.rfind(&search) {
            Some(index) => self[..index].to_string(),
            None => self.to_string(),
        }
    }

    fn between<T: Display>(&self, from: T, to: T) -> String {
        let from = from.to_string();
        let to = to.to_string();

        if from.is_empty() || to.is_empty() {
            return self.to_string();
        }

        self.after(from).before_last(to)
    }

    fn between_first<T: Display>(&self, from: T, to: T) -> String {
        let from = from.to_string();
        let to = to.to_string();

        if from.is_empty() || to.is_empty() {
            return self.to_string();
        }

        self.after(from).before(to)
    }

    fn kebab(&self) -> String {
        self.snake_with_delimeter("-")
    }

    fn lcfirst(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }

    fn lower(&self) -> String {
        self.to_lowercase()
    }

    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }

    fn snake(&self) -> String {
        self.snake_with_delimeter("_")
    }

    fn snake_with_delimeter(&self, delimiter: &str) -> String {
        if self.chars().any(|c| c.is_uppercase()) {
            let whitespace_regex = Regex::new(r"\s+").unwrap();
            let no_whitespace = whitespace_regex.replace_all(self, "");

            let result = no_whitespace.chars().enumerate().fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && c.is_ascii_uppercase() {
                    acc.push_str(delimiter);
                }
                acc.push(c);
                acc
            });

            result.to_lowercase()
        } else {
            self.to_string()
        }
    }

    fn ucfirst(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }
}

impl Strings for String {
    fn after<T: Display>(&self, search: T) -> String {
        self.as_str().after(search)
    }

    fn after_last<T: Display>(&self, search: T) -> String {
        self.as_str().after_last(search)
    }

    fn before<T: Display>(&self, search: T) -> String {
        self.as_str().before(search)
    }

    fn before_last<T: Display>(&self, search: T) -> String {
        self.as_str().before_last(search)
    }

    fn between<T: Display>(&self, from: T, to: T) -> String {
        self.as_str().between(from, to)
    }

    fn between_first<T: Display>(&self, from: T, to: T) -> String {
        self.as_str().between_first(from, to)
    }

    fn kebab(&self) -> String {
        self.as_str().kebab()
    }

    fn lcfirst(&self) -> String {
        self.as_str().lcfirst()
    }

    fn lower(&self) -> String {
        self.as_str().lower()
    }

    fn reverse(&self) -> String {
        self.as_str().reverse()
    }

    fn snake(&self) -> String {
        self.as_str().snake()
    }

    fn snake_with_delimeter(&self, delimiter: &str) -> String {
        self.as_str().snake_with_delimeter(delimiter)
    }

    fn ucfirst(&self) -> String {
        self.as_str().ucfirst()
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }
}

