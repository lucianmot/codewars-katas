// Rust - Convert boolean values to strings 'Yes' or 'No'.
// https://www.codewars.com/kata/53369039d7ab3ac506000467/train/rust

fn bool_to_word(value: bool) -> &'static str {
    if value {"Yes"} else {"No"}
}
