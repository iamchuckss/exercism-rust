use std::ops::Add;

pub fn reverse(input: &str) -> String {
    input.to_string()
         .chars()
         .rev()
         .collect()
}
