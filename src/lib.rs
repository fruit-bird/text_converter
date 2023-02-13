use arboard::Clipboard;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Trait with all methods needed to convert text into a specific format
pub trait TextConverter {
    /// Transforms the input into the desired form
    ///
    /// Preferably never called directly, but through the other trait methods
    ///
    /// # Examples
    ///
    /// ```
    /// struct ReverseText;
    ///
    /// impl TextConverter for ReverseText {
    ///     fn converter(input: impl AsRef<str>) -> String {
    ///         input.as_ref().chars().rev().collect()
    ///     }
    /// }
    ///
    /// let text = "Hello World!";
    /// let reverse_text = ReverseText::new_from_text(text);
    /// assert_eq!("!dlroW olleH", reverse_text);
    /// ```
    fn converter(input: impl AsRef<str>) -> String;

    /// Converts given input with the [converter](Self::converter()) method
    fn new_from_text(input: impl AsRef<str>) -> String {
        Self::converter(input)
    }

    /// Fetches clipboard contents and converts them with the [converter](Self::converter()) method
    /// 
    /// # Returns
    /// Returns the converted text from the clipboard
    /// 
    /// Will return an empty string if it fails to fetch the clipboard contents or if it contains something other than text
    /// 
    /// # Panics
    /// Will panic if it fails to fetch the clipboard
    fn new_from_clipboard() -> String {
        let mut clipboard = Clipboard::new().expect("Could not fetch the clipboard contents");
        let input = clipboard.get_text().unwrap_or_default();

        Self::converter(input)
    }

    /// Fetches file contents and converts them with the [converter](Self::converter()) method
    ///
    /// # Panics
    /// - If file is inaccessible or if it is not in text format (.txt, .md...)
    /// - If it fails to create the output file
    ///
    /// # Returns
    /// - The conversion string from the file contents
    /// - Outputs the conversion into a file called originalname_converted.md
    fn new_from_file(path: impl AsRef<Path>) -> String {
        let input = fs::read_to_string(path.as_ref()).expect("Failed to read file contents");
        let output = Self::converter(input);
        let new_path = path
            .as_ref()
            .to_str()
            .unwrap()
            .split('.')
            .take(1)
            .collect::<String>()
            + "_converted.md";

        File::create(new_path)
            .expect("Failed to create the output file")
            .write_all(output.as_bytes())
            .expect("Failed to write to the output file");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ReverseText;

    impl TextConverter for ReverseText {
        fn converter(input: impl AsRef<str>) -> String {
            input.as_ref().chars().rev().collect()
        }
    }

    #[test]
    fn reverse_conversion() {
        let text = "Hello World!";
        let reverse_text = ReverseText::new_from_text(text);
        assert_eq!("!dlroW olleH", reverse_text);
    }
}
