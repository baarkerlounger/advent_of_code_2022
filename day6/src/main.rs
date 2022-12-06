use std::collections::HashSet;
use std::fs;

fn main() {
    let file_contents =
        fs::read_to_string("data/input.txt").expect("Should have been able to read the file");
    println!("Result for part 1 is {}", result(&file_contents, Part::One));
    println!("Result for part 2 is {}", result(&file_contents, Part::Two));
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
        let res = result(input, Part::One);
        assert_eq!(res, 7);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let res = result(input, Part::One);
        assert_eq!(res, 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let res = result(input, Part::One);
        assert_eq!(res, 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let res = result(input, Part::One);
        assert_eq!(res, 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let res = result(input, Part::One);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let res = result(input, Part::Two);
        assert_eq!(res, 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let res = result(input, Part::Two);
        assert_eq!(res, 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let res = result(input, Part::Two);
        assert_eq!(res, 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let res = result(input, Part::Two);
        assert_eq!(res, 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let res = result(input, Part::Two);
        assert_eq!(res, 26);
    }
}
