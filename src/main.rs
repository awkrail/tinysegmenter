use std::env;

fn construct_words(text: &String) -> Vec<char> {
    let mut chars : Vec<char> = Vec::new();
    for c in text.chars() {
        chars.push(c);
    }
    return chars
}

fn tokenize(text: &String) -> String {
    if text.len() == 0 { return String::from(""); }
    let chars = construct_words(text);
    let segmented_str = String::from_iter(chars);
    segmented_str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    let segmented_str = tokenize(text);
    println!("{}", segmented_str);
}
