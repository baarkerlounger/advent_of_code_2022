use std::fs;

fn main() {
    let filename = "input.txt";
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");

    let split_elves = file_contents.split("\n\n");
    let mut elf_calories = Vec::new();

    for (index, calories) in split_elves.enumerate() {
        let calorie_sum: i32 = calories
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .sum();
        let elf = index + 1;
        elf_calories.push((elf, calorie_sum));
    }

    elf_calories.sort_by(|a, b| b.1.cmp(&a.1));

    let total_calories: i32 = elf_calories[0..=2].iter().map(|tup| tup.1).sum();

    println!(
        "The elf carrying the most calories is elf {} with {} calories",
        elf_calories[0].0, elf_calories[0].1
    );
    println!(
        "The elf carrying the second most calories is elf {} with {} calories",
        elf_calories[1].0, elf_calories[1].1
    );
    println!(
        "The elf carrying the third most calories is elf {} with {} calories",
        elf_calories[2].0, elf_calories[2].1
    );

    println!(
        "Together they are carrying a total of {} calories",
        total_calories
    );
}
