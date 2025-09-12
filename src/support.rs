use std::fmt::Display;

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

    fn reverse(&self) -> String {
        self.chars().rev().collect()
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

    fn reverse(&self) -> String {
        self.as_str().reverse()
    }
}

