use crate::grammar::Word;

pub fn tokenize_input(input: &str) -> Vec<Word> {
    let tokens: Vec<Word> = Vec::new();

    for (i, tok) in input.split([' ', ';', '\t', '\n']).enumerate() {
        println!("{i}: {tok}");
    }
    tokens
}
