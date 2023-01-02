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

fn result(input: &str, _part: Part) -> i32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut cycle = 1;
    let mut register = 1;
    let mut signal_strength = 0;

    for instruction in lines {
        let split: Vec<&str> = instruction.split(" ").collect();
        let op = split[0];

        match op {
            "noop" => {
                cycle += 1;
                signal_strength = calc_signal_strength(cycle, register, signal_strength);
            }
            "addx" => {
                cycle += 1;
                signal_strength = calc_signal_strength(cycle, register, signal_strength);
                cycle += 1;
                let val: i32 = split[1].parse().unwrap();
                register += val;
                signal_strength = calc_signal_strength(cycle, register, signal_strength);
            }
            _ => panic!(),
        }
    }
    signal_strength
}

fn calc_signal_strength(cycle: u32, register: i32, signal_strength: i32) -> i32 {
    let mut result = signal_strength;
    let keys = [20, 60, 100, 140, 180, 220];

    if keys.contains(&cycle) {
        result += cycle as i32 * register;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 13140);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
    //     assert_eq!(result(&file_contents, Part::Two), 1);
    // }
}
