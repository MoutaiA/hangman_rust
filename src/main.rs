use std::{io};

fn main() {
    let mystery_word = String::from("toto");
    display_mystery_word(mystery_word);
    println!("{}", get_user_input());
}

fn display_mystery_word(mystery_word: String) {
    println!("The mystery word is {}", mystery_word)
}

fn get_user_input() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
		Ok(_) => {}
		Err(error) => println!("{}", error)
	}

    return input;
}
