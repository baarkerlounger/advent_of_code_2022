use ordinal::Ordinal;
use std::fs;

fn main() {
    let elf_calories = elf_calories_list("data/input.txt");
    let total_calories = top_n_total_calories(&elf_calories, 3);
    for idx in 0..3 {
        println!(
            "The elf carrying the {} most calories is elf {} with {} calories",
            Ordinal(idx + 1),
            elf_calories[idx].0,
            elf_calories[idx].1
        );
    }

    println!(
        "Together they are carrying a total of {} calories",
        total_calories
    );
}

fn elf_calories_list(filename: &str) -> Vec<(usize, u32)> {
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");

    let split_elves = file_contents.split("\n\n");
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
        let elf_calories = elf_calories_list("data/demo_input.txt");
        assert_eq!(elf_calories[0], (4, 24000));
    }

    #[test]
    fn test_part_2() {
        let elf_calories = elf_calories_list("data/demo_input.txt");
        let total_calories = top_n_total_calories(&elf_calories, 3);
        assert_eq!(total_calories, 45000);
    }
}
