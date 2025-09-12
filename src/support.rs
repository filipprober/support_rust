pub trait Strings {
    ///
    /// Reverses the given string.
    ///
    /// # Usage
    ///
    /// ```
    /// use support::Strings;
    ///
    /// "Rust Support".reverse();
    /// // Output: "troppuS tsuR"
    /// ```
    fn reverse(&self) -> String;
}

impl Strings for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}

impl Strings for String {
    fn reverse(&self) -> String {
        self.as_str().reverse()
    }
}

