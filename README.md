# Text Converter

A trait with all methods needed to convert text into a specific format

## Example
```rust
use text_converter::TextConverter;

struct ReverseText;

impl TextConverter for ReverseText {
    fn converter(input: impl AsRef<str>) -> String {
        input.as_ref().chars().rev().collect()
    }
}

let clipboard_reverse = ReverseText::new_from_clipboard();
let file_reverse = ReverseText::new_from_file("input.txt");

let reverse_text = ReverseText::new_from_text("Hello World!");
assert_eq!("!dlroW olleH", reverse_text);
```