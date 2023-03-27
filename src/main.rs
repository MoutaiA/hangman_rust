use std::collections::HashSet;
use std::io;

const END_GAME: u8 = 0;
const STOP_GAME: &str = "STOP";

fn main() {
    let mystery_word = String::from("toto");
    let mut counter = mystery_word.len();
    let mut characters: HashSet<String> = HashSet::new();

    while counter > END_GAME.into() {
        let user_input: String = get_user_input();

        if characters.contains(&user_input) {
            let values: Vec<String> = get_all_characters(&characters);
            print!("Please, enter a character which hasn't been used yeat, here the characters already set: {:?}", values);
            println!();
            continue;
        }

        if STOP_GAME == user_input.trim() {
            println!("Bye!");
            break;
        }

        characters.insert(user_input.clone());

        if is_same_word(&mystery_word, &user_input) {
            println!("GG");
            break;
        } else if is_part(&mystery_word, &user_input) {
            //TODO:
        } else {
            counter -= 1;
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    println!("Please, enter a character, or 'STOP', if you want to stop the game");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("{}", error),
    }

    return input;
}

fn get_all_characters(characters: &HashSet<String>) -> Vec<String> {
    let mut arr: Vec<String> = Vec::with_capacity(characters.len());

    for character in characters.iter() {
        arr.push(character.clone());
    }

    return arr;
}

fn is_same_word(mystery_word: &str, user_input: &str) -> bool {
    return mystery_word.trim() == user_input.trim();
}

fn is_part(mystery_word: &str, user_input: &str) -> bool {
    return true;
}
