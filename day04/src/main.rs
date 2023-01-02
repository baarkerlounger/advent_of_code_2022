use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result(&file_contents, Part::One)),
            2 => println!("Result for part 2 is {}", result(&file_contents, Part::Two)),
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result(&file_contents, Part::One));
        println!("Result for part 2 is {}", result(&file_contents, Part::Two));
    }
}

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> usize {
    let collection = input.trim().split("\n");
    let mut overlapping_elves = Vec::new();

    for (idx, elf_pair) in collection.enumerate() {
        let elf_pair_split: Vec<&str> = elf_pair.split(",").collect();
        let elf_1_split: Vec<&str> = elf_pair_split[0].split("-").collect();
        let elf_2_split: Vec<&str> = elf_pair_split[1].split("-").collect();
        if overlap(
            elf_1_split[0].parse::<u32>().unwrap(),
            elf_1_split[1].parse::<u32>().unwrap(),
            elf_2_split[0].parse::<u32>().unwrap(),
            elf_2_split[1].parse::<u32>().unwrap(),
            &part,
        ) {
            overlapping_elves.push(idx);
        }
    }
    overlapping_elves.len()
}

fn overlap(r_1_1: u32, r_1_2: u32, r_2_1: u32, r_2_2: u32, part: &Part) -> bool {
    match part {
        Part::One => (r_1_1 >= r_2_1 && r_1_2 <= r_2_2) || (r_2_1 >= r_1_1 && r_2_2 <= r_1_2),
        Part::Two => {
            (r_1_1 >= r_2_1 && r_1_1 <= r_2_2)
                || (r_1_2 <= r_2_2 && r_1_2 >= r_2_1)
                || (r_2_1 >= r_1_1 && r_2_1 <= r_1_2)
                || (r_2_2 <= r_1_2 && r_2_2 >= r_1_1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 2);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 4);
    }
}
