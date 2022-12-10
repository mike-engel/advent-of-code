use std::{env, fs};

struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn new() -> State {
        State { stacks: Vec::new() }
    }

    fn new_stack(&mut self) {
        self.stacks.push(Vec::new());
    }

    fn from_str(raw: &str) -> State {
        let mut state = State::new();
        let rows = raw.lines().enumerate();

        for (row, line) in rows {
            let crates = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(String::from_iter)
                .collect::<Vec<String>>();

            for (col, package) in crates.into_iter().enumerate() {
                if state.stacks.len() <= col {
                    state.new_stack();
                }

                if package.trim() == ""
                    || package
                        .trim()
                        .chars()
                        .next()
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .is_ok()
                {
                    continue;
                }

                let crate_name = package.chars().nth(1).unwrap();

                state.stacks[col].insert(0, crate_name);
            }
        }

        state
    }

    fn move_crates_by_one(&mut self, instruction: &str) {
        if instruction.trim() == "" {
            return;
        }

        let mut instruction_parts = instruction.split(' ');
        let (count, from, to) = (
            instruction_parts.nth(1).unwrap().parse::<i32>().unwrap(),
            instruction_parts.nth(1).unwrap().parse::<usize>().unwrap(),
            instruction_parts.nth(1).unwrap().parse::<usize>().unwrap(),
        );

        for _ in 0..count {
            let to_move = self.stacks[from - 1].pop().unwrap();

            self.stacks[to - 1].push(to_move);
        }
    }

    fn move_crates_together(&mut self, instruction: &str) {
        if instruction.trim() == "" {
            return;
        }

        let mut instruction_parts = instruction.split(' ');
        let (count, from, to) = (
            instruction_parts.nth(1).unwrap().parse::<usize>().unwrap(),
            instruction_parts.nth(1).unwrap().parse::<usize>().unwrap(),
            instruction_parts.nth(1).unwrap().parse::<usize>().unwrap(),
        );

        let stack_length = self.stacks[from - 1].len();
        let mut to_move = self.stacks[from - 1].split_off(stack_length - count);

        self.stacks[to - 1].append(&mut to_move);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You must supply the path to the input file");
    }

    let path = &args[1];
    let file_content = fs::read(path).map(|bytes| String::from_utf8(bytes).unwrap());
    let mut parts = file_content.as_ref().unwrap().split("\n\n");
    let (raw_initial_state, moves) = (parts.next().unwrap(), parts.next().unwrap());
    let mut one_by_one_state = State::from_str(raw_initial_state);
    let mut together_state = State::from_str(raw_initial_state);

    moves.lines().for_each(|line| {
        one_by_one_state.move_crates_by_one(line);
        together_state.move_crates_together(line);
    });

    let top_crates_one_by_one = one_by_one_state
        .stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned());
    let top_crates_together = together_state
        .stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned());

    println!(
        "The top crate in each stack when moved one by one is: {}",
        String::from_iter(top_crates_one_by_one)
    );
    println!(
        "The top crate in each stack when moved together is: {}",
        String::from_iter(top_crates_together)
    );
}
