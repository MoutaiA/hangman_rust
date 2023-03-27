use colored::*;
use std::collections::HashSet;
use std::io;

const END_GAME: u8 = 0;
const STOP_GAME: &str = "STOP";

fn main() {
	//TODO: API
    let mystery_word = String::from("toto");
    let mut counter = mystery_word.len();
    let mut characters: HashSet<String> = HashSet::new();
    let mut response = set_response(&mystery_word, &characters);

    display_welcome();
    display_response(&response);

    while counter > END_GAME.into() {
        let mut user_input: String = get_user_input();
        user_input = String::from(user_input.trim());

        if STOP_GAME == user_input {
            break;
        }

        if characters.contains(&user_input) {
            display_character_already_set(&characters);
            continue;
        }

        characters.insert(user_input.clone());

        if is_part(&mystery_word, &user_input) {
            response = set_response(&mystery_word, &characters);
            if is_response_mystery_word(&mystery_word, &response) {
                println!("You did it!");
                break;
            }
        } else {
            counter -= 1;
        }
        display_response(&response);
    }
    println!("Bye!");
}

fn display_welcome() {
    println!();
    println!(
        "{}",
        "Welcome to the hangman"
            .yellow()
            .bold()
            .reverse()
            .underline()
    );
    println!("{}", "Please enter only one character at the time, if you enter multiple, only the first one will be read.".bold());
    println!(
        "{} {}",
        "Also, if you want to quit, just enter ".bold(),
        STOP_GAME.red().bold()
    );
    println!(
        "{}",
        "Finally, only word between a to z, and on lowercase are allowed".bold()
    );
    println!();
}

fn display_response(response: &str) {
    println!("Your current response is: {}", response);
}

fn set_response(mystery_word: &str, characters: &HashSet<String>) -> String {
    let mut response = String::with_capacity(mystery_word.len() * 2 - 1);

    for c in mystery_word.chars() {
        if characters.contains(&c.to_string()) {
            response += &c.to_string();
        } else {
            response += "_ ";
        }
    }

    return response;
}

fn get_user_input() -> String {
    let mut input = String::new();

    println!("{}", "Next character?".bold());
	//TODO: handle none
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = String::from(input.trim());
            if trimmed_input != STOP_GAME {
                input = trimmed_input.chars().nth(0).unwrap().to_string();
            }
        }
        Err(error) => println!("{}", error),
    }

    return input;
}

fn display_character_already_set(characters: &HashSet<String>) {
    let values: Vec<String> = get_all_characters(&characters);
    print!("Please, enter a character which hasn't been used yeat, here the characters already set: {:?}", values);
    println!();
}

fn get_all_characters(characters: &HashSet<String>) -> Vec<String> {
    let mut arr: Vec<String> = Vec::with_capacity(characters.len());

    for character in characters.iter() {
        arr.push(character.clone());
    }

    return arr;
}

//TODO:
fn is_part(mystery_word: &str, user_input: &str) -> bool {
    return true;
}

fn is_response_mystery_word(mystery_word: &str, response: &str) -> bool {
    return mystery_word == response;
}
