use renghlish::*;

fn main() {
    words("Renghlish is very cool!")
        .unwrap()
        .get_rows()
        .iter()
        .for_each(|row| {
            eprintln!("{row}");
        })
}
