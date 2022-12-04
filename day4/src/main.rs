use std::fs;

fn main() {
    println!(
        "Result for part 1 is {}",
        result("data/input.txt", Part::One)
    );
    println!(
        "Result for part 2 is {}",
        result("data/input.txt", Part::Two)
    );
}

enum Part {
    One,
    Two,
}

fn result(filename: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");
    let collection = file_contents.trim().split("\n");
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
        let result = result("data/demo_input.txt", Part::One);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let result = result("data/demo_input.txt", Part::Two);
        assert_eq!(result, 4);
    }
}
