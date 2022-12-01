use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You must supply the path to the input file");
    }

    let path = &args[1];
    let calories_per_elf = fs::read(path)
        .map(|bytes| String::from_utf8(bytes).unwrap())
        .map(|contents| {
            let mut sorted = contents
                .split("\n\n")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|elf| {
                    elf.split('\n')
                        .map(|calorie| calorie.parse::<i64>().unwrap_or(0))
                        .sum::<i64>()
                })
                .collect::<Vec<i64>>();

            sorted.sort();

            sorted
        });

    let (highest, top_three) = match calories_per_elf {
        Ok(calorie_list) => {
            let calorie_iter = calorie_list.iter();
            let highest = calorie_iter.clone().max().unwrap_or(&0).to_owned();
            let top_three = calorie_iter.rev().take(3).sum::<i64>();

            (highest, top_three)
        }
        Err(err) => {
            panic!("Unable to read the contents of {}! {:?}", path, err);
        }
    };

    println!(
        "The elf with the most calories is carrying {} calories",
        highest
    );
    println!("The top three elves carry {} calories together", top_three);
}
