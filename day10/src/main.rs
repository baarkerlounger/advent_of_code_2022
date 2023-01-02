use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result_1(&file_contents)),
            2 => {
                println!("Result for part 2 is");
                println!("{}", result_2(&file_contents));
            },
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result_1(&file_contents));
        println!("Result for part 2 is");
        println!("{}", result_2(&file_contents));
    }
}

fn result_1(input: &str) -> i32 {
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

fn result_2(input: &str) -> String {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut cycle = 1;
    let mut register = 1;

    let mut crt_row: [&str; 240] = ["."; 240];
    crt_row[0] = "#";

    for instruction in lines {
        let split: Vec<&str> = instruction.split(" ").collect();
        let op = split[0];

        match op {
            "noop" => {
                crt_row = updated_crt_row(cycle, register, crt_row);
                cycle += 1;
            }
            "addx" => {
                crt_row = updated_crt_row(cycle, register, crt_row);
                cycle += 1;
                let val: i32 = split[1].parse().unwrap();
                register += val;
                crt_row = updated_crt_row(cycle, register, crt_row);
                cycle += 1;
            }
            _ => panic!(),
        }
    }

    let mut result_string = String::from("");
    for chunk in crt_row.chunks(40) {
        result_string.push_str(&chunk.join(""));
        result_string.push_str("\n");
    }
    result_string
}

fn updated_crt_row(cycle: u32, register: i32, row: [&str; 240]) -> [&str; 240] {
    let mut crt_row = row;

    let adj: i32 = cycle as i32 / 40 * 40;
    let register = register + adj;
    for j in (register - 1)..=(register + 1) {
        if cycle == j as u32 {
            crt_row[j as usize] = "#";
        }
    }
    crt_row
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result_1(&file_contents), 13140);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        let result = String::from(
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\
            \n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\
            \n######......######......######......####\n#######.......#######.......#######.....\n"
        );
        assert_eq!(result_2(&file_contents), result);
    }
}
