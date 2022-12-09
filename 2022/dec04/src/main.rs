use std::ops::RangeInclusive;
use std::{env, fs};

struct Pair(RangeInclusive<u32>, RangeInclusive<u32>);

impl Pair {
    fn from_str(raw: &str) -> Pair {
        let mut parts = raw.split(',');
        let mut first = parts.next().unwrap().split('-');
        let mut second = parts.next().unwrap().split('-');
        let first_range = RangeInclusive::new(
            str::parse::<u32>(first.next().unwrap()).unwrap(),
            str::parse::<u32>(first.next().unwrap()).unwrap(),
        );
        let second_range = RangeInclusive::new(
            str::parse::<u32>(second.next().unwrap()).unwrap(),
            str::parse::<u32>(second.next().unwrap()).unwrap(),
        );

        Pair(first_range, second_range)
    }

    fn has_contained_range(&self) -> bool {
        let Pair(left, right) = self;
        let (left_size, right_size) = (left.end() - left.start(), right.end() - right.start());

        if left_size <= right_size {
            return left.start() >= right.start() && left.end() <= right.end();
        } else {
            return right.start() >= left.start() && right.end() <= left.end();
        }
    }

    fn has_partially_contained_range(&self) -> bool {
        let Pair(left, right) = self;

        if left.start() <= right.start() {
            return left.end() >= right.start();
        } else {
            return right.end() >= left.start();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You must supply the path to the input file");
    }

    let path = &args[1];
    let file_content = fs::read(path).map(|bytes| String::from_utf8(bytes).unwrap());
    let full_overlap_count = file_content.as_ref().map(|content| {
        content
            .lines()
            .map(Pair::from_str)
            .map(|pair| pair.has_contained_range())
            .filter(|contained| contained == &true)
            .count()
    });
    let partial_overlap_count = file_content.as_ref().map(|content| {
        content
            .lines()
            .map(Pair::from_str)
            .map(|pair| pair.has_partially_contained_range())
            .filter(|contained| contained == &true)
            .count()
    });

    match full_overlap_count {
        Ok(count) => println!("There are {} pairs that have a complete overlap!", count),
        Err(err) => eprintln!("There was an error getting the count: {}", err),
    };

    match partial_overlap_count {
        Ok(count) => println!("There are {} pairs that have a partial overlap!", count),
        Err(err) => eprintln!("There was an error getting the count: {}", err),
    };
}
