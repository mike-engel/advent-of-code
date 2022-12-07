use std::collections::HashSet;
use std::{env, fs};

fn find_pairs(backpack: &str) -> Vec<char> {
    let compartment_size = &backpack.len() / 2;
    let (compartment_one_str, compartment_two_str) = backpack.split_at(compartment_size);
    let compartment_one: HashSet<char> = HashSet::from_iter(compartment_one_str.chars());
    let compartment_two: HashSet<char> = HashSet::from_iter(compartment_two_str.chars());

    Vec::from_iter(
        compartment_one
            .intersection(&compartment_two)
            .map(|char| char.to_owned()),
    )
}

fn find_badge(backpacks: Vec<&str>) -> char {
    let hashes = backpacks
        .into_iter()
        .map(|pack| -> HashSet<char> { HashSet::from_iter(pack.chars()) });

    let intersection = hashes.fold(None, |acc, hash| {
        if acc.is_none() {
            return Some(hash);
        }

        Some(HashSet::from_iter(
            acc.unwrap().intersection(&hash).map(|char| char.to_owned()),
        ))
    });

    intersection
        .unwrap()
        .into_iter()
        .collect::<Vec<char>>()
        .first()
        .unwrap()
        .to_owned()
}

fn score_letter(letter: char) -> u32 {
    if letter.is_uppercase() {
        return letter
            .to_digit(36)
            .unwrap_or_else(|| panic!("{} is not a valid entry!", &letter))
            - 9
            + 26;
    }

    letter
        .to_digit(36)
        .unwrap_or_else(|| panic!("{} is not a valid entry!", &letter))
        - 9
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You must supply the path to the input file");
    }

    let path = &args[1];
    let file_content = fs::read(path).map(|bytes| String::from_utf8(bytes).unwrap());
    let score = file_content.as_ref().map(|content| {
        content
            .lines()
            .flat_map(find_pairs)
            .map(score_letter)
            .sum::<u32>()
    });
    let group_score = file_content.as_ref().map(|content| {
        content
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(Vec::from)
            .map(find_badge)
            .map(score_letter)
            .sum::<u32>()
    });

    match score {
        Ok(num) => println!("The sum of priorities is {}", num),
        Err(err) => eprintln!("Uh oh: {}", err),
    }

    match group_score {
        Ok(num) => println!("The sum of priorities for all group badges is {}", num),
        Err(err) => eprintln!("Uh oh: {}", err),
    }
}
