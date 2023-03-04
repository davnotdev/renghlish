use renghlish::*;

fn main() {
    let mut words_str = String::new();
    std::env::args().skip(1).for_each(|word| {
        words_str.push_str(&word);
        words_str.push(' ');
    });
    words_str.pop();

    if let Some(words) = words(&words_str) {
        words.get_rows().iter().for_each(|row| println!("{row}"));
    } else {
        eprintln!(
            "`eng2reng` expects arguments to be alphabetical and single space separated ie 'Hello World'."
        );
    }
}
