use ordinal::Ordinal;
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

fn result(input: &str, part: Part) -> u32 {
    let elf_calories = elf_calories_list(input);
    match part {
        Part::One => {
            for idx in 0..3 {
                println!(
                    "The elf carrying the {} most calories is elf {} with {} calories",
                    Ordinal(idx + 1),
                    elf_calories[idx].0,
                    elf_calories[idx].1
                );
            }
            elf_calories[0].1
        }
        Part::Two => top_n_total_calories(&elf_calories, 3),
    }
}

fn elf_calories_list(input: &str) -> Vec<(usize, u32)> {
    let split_elves = input.split("\n\n");
    let mut elf_calories = Vec::new();

    for (index, calories) in split_elves.enumerate() {
        let calorie_sum: u32 = calories
            .split("\n")
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .sum();
        let elf = index + 1;
        elf_calories.push((elf, calorie_sum));
    }

    elf_calories.sort_by(|a, b| b.1.cmp(&a.1));
    elf_calories
}

fn top_n_total_calories(elf_calories: &Vec<(usize, u32)>, n: usize) -> u32 {
    elf_calories[0..n].iter().map(|tup| tup.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 24000);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 45000);
    }
}
