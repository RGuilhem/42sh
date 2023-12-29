use crate::grammar::Name;
use crate::grammar::Word;

pub fn tokenize_input(input: &str) -> Vec<Word> {
    let mut tokens: Vec<Word> = Vec::new();
    let mut sq = false;
    let mut dq = false;
    let mut start = 0;

    for (i, c) in input.chars().enumerate() {
        if "; \n\t|&()<>".contains(c) {
            let word: &str = &input[start..i];
            println!("split at {i}: '{c}' -> '{}'", word);
            //add word
            //add separator
            start = i + 1;
        }
    }
    let word: &str = &input[start..];
    println!("last: '{}'", word);
    tokens
}
