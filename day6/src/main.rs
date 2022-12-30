use std::collections::HashSet;
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
    let chars: Vec<char> = input.chars().collect();
    let size = match part {
        Part::One => 4,
        Part::Two => 14,
    };

    for idx in 0..chars.len() {
        let set: HashSet<&char> = HashSet::from_iter(&chars[idx..(idx + size)]);
        if set.len() == size {
            return idx + size;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(result(input, Part::One), 7);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(result(input, Part::One), 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(result(input, Part::One), 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(result(input, Part::One), 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(result(input, Part::One), 11);
    }

    #[test]
    fn test_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(result(input, Part::Two), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(result(input, Part::Two), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(result(input, Part::Two), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(result(input, Part::Two), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(result(input, Part::Two), 26);
    }
}
