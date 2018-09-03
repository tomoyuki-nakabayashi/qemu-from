extern crate combine;
use combine::Parser;
use combine::char::letter;

fn main() {
    let mut word = letter();
    let result = word.parse("test");
    assert!(result.is_ok());
}
