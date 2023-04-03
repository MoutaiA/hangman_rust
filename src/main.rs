use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io;

const MAX_LIFE: i32 = 6;
const END_GAME: u8 = 0;
const STOP_GAME: &str = "STOP";

/*
                        |___
   - Display the hangman|  O
                        | /|\
                        | / \
                        |_____
*/

#[tokio::main]
async fn main() {
    display_welcome();
    let choice = get_user_choice();

    let mystery_word = get_word(choice).await;
    let mut counter = MAX_LIFE;
    let mut characters: HashSet<String> = HashSet::new();
    let mut response = set_response(&mystery_word, &characters);

    display_response(&response, &counter);

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
        display_response(&response, &counter);
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

fn get_user_choice() -> bool {
    println!("Which kind of word do you want to play with, a real (easy) or a fake one (hard) ?");
    loop {
        let mut input = String::new();
        println!("Enter 'y' for a easy or 'n' for hard: ");
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}

async fn get_word(choice: bool) -> String {
    let word: String;

    if choice {
        let response = make_request("/word").await;
        word = response.unwrap();
    } else {
        let response = make_request("/fake").await;
        word = response.unwrap();
    }

    return word;
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    word: String,
}

async fn make_request(path: &str) -> Result<String, reqwest::Error> {
    let base_url = "http://127.0.0.1:5000";
    let url = format!("{}{}", base_url, path);
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    return Ok(response.word);
}

fn display_response(response: &str, counter: &i32) {
    match counter {
        0 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|      /|\\");
            println!("|      / \\");
            println!("|       ");
            println!("\n\n");
        }
        1 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|      /|\\");
            println!("|      / ");
            println!("|       ");
            println!("\n\n");
        }
        2 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|      /|\\");
            println!("|       ");
            println!("|       ");
            println!("\n\n");
        }
        3 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|      /|");
            println!("|       ");
            println!("|       ");
            println!("\n\n");
        }
        4 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|       |");
            println!("|       ");
            println!("|       ");
            println!("\n\n");
        }
        5 => {
            println!("_________");
            println!("|       |");
            println!("|       0");
            println!("|       ");
            println!("|       ");
            println!("|       ");
            println!("\n\n");
        }
        _ => {
            println!("_________");
            println!("|       |");
            println!("|       ");
            println!("|       ");
            println!("|       ");
            println!("|       ");
            println!("\n\n");
        }
    }

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
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = String::from(input.trim());
            if trimmed_input != STOP_GAME {
                if let Some(first_char) = trimmed_input.chars().next() {
                    input = first_char.to_string();
                } else {
                    input = String::new();
                    println!("Please, provide a character");
                }
            }
        }
        Err(error) => {
            println!("{}", error);
            input = String::from("");
        }
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

fn is_part(mystery_word: &str, user_input: &str) -> bool {
    return mystery_word.contains(user_input);
}

fn is_response_mystery_word(mystery_word: &str, response: &str) -> bool {
    return mystery_word == response;
}
