fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word.to_string()) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
