use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Result for part 1 is {}", result_1("data/input.txt"));
    println!("Result for part 2 is {}", result_2("data/input.txt"));
}

fn result_1(filename: &str) -> u32 {
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");
    let rucksacks = file_contents.trim().split("\n");
    let mut sum = 0;

    for r in rucksacks {
        let compartment_item_count = r.chars().count() / 2;
        let (compartment_1, compartment_2) = r.split_at(compartment_item_count);
        let compartment_1_uniq: HashSet<char> = compartment_1.chars().collect();
        let compartment_2_uniq: HashSet<char> = compartment_2.chars().collect();
        let common = compartment_1_uniq.intersection(&compartment_2_uniq);

        for item in common {
            let priority = ALPHABET.iter().position(|v| v == item).unwrap() + 1;
            sum = sum + priority as u32;
        }
    }
    sum
}

fn result_2(filename: &str) -> u32 {
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");
    let mut rucksacks: Vec<&str> = file_contents.trim().split("\n").collect();
    let mut sum = 0;

    while rucksacks.len() > 0 {
        let rucksack_1: HashSet<char> = rucksacks[0].chars().collect();
        let rucksack_2: HashSet<char> = rucksacks[1].chars().collect();
        let rucksack_3: HashSet<char> = rucksacks[2].chars().collect();
        let common = &(&rucksack_1 & &rucksack_2) & &rucksack_3;

        for item in common {
            let priority = ALPHABET.iter().position(|&v| v == item).unwrap() + 1;
            sum = sum + priority as u32;
        }
        rucksacks.drain(0..=2);
    }
    sum
}

static ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = result_1("data/demo_input.txt");
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_2() {
        let result = result_2("data/demo_input.txt");
        assert_eq!(result, 70);
    }
}
