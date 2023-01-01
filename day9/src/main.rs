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

type Coord = (i32, i32);

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> usize {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let head = (0, 0);
    let knot_count = match part {
        Part::One => 1,
        Part::Two => 9,
    };
    let mut rope: Vec<Coord> = Vec::new();
    for _ in 0..=knot_count {
        rope.push(head);
    }
    let mut tail_coords = HashSet::new();
    tail_coords.insert(head);

    for motion in lines {
        let split: Vec<&str> = motion.split(" ").collect();
        let direction = split[0];
        let size: i32 = split[1].parse().unwrap();
        for _ in 1..=size {
            rope[0] = add_coords(rope[0], head_motion_tuple(direction));
            for idx in 1..rope.len() {
                rope[idx] = add_coords(rope[idx], tail_motion_tuple(rope[idx - 1], rope[idx]));
            }
            tail_coords.insert(rope.last().unwrap().clone());
        }
    }
    tail_coords.len()
}

fn head_motion_tuple(direction: &str) -> Coord {
    match direction {
        "D" => (0, 1),
        "U" => (0, -1),
        "R" => (1, 0),
        "L" => (-1, 0),
        _ => panic!(),
    }
}

fn tail_motion_tuple(head: Coord, tail: Coord) -> Coord {
    let mut motion = (0, 0);

    if tail.0 == head.0 && tail.1 == (head.1 - 2) {
        motion = (0, 1);
    } else if tail.0 == head.0 && tail.1 == (head.1 + 2) {
        motion = (0, -1);
    } else if tail.1 == head.1 && tail.0 == (head.0 - 2) {
        motion = (1, 0);
    } else if tail.1 == head.1 && tail.0 == (head.0 + 2) {
        motion = (-1, 0);
    // Diagonal catch up
    } else if tail.0 == (head.0 - 1) && tail.1 == (head.1 + 2) {
        motion = (1, -1);
    } else if tail.0 == (head.0 - 1) && tail.1 == (head.1 - 2) {
        motion = (1, 1);
    } else if tail.0 == (head.0 + 1) && tail.1 == (head.1 + 2) {
        motion = (-1, -1);
    } else if tail.0 == (head.0 + 1) && tail.1 == (head.1 - 2) {
        motion = (-1, 1);
    } else if tail.0 == (head.0 - 2) && tail.1 == (head.1 + 1) {
        motion = (1, -1);
    } else if tail.0 == (head.0 - 2) && tail.1 == (head.1 - 1) {
        motion = (1, 1);
    } else if tail.0 == (head.0 + 2) && tail.1 == (head.1 + 1) {
        motion = (-1, -1);
    } else if tail.0 == (head.0 + 2) && tail.1 == (head.1 - 1) {
        motion = (-1, 1);
    // Multi knot diagonals
    } else if tail.0 == (head.0 + 2) && tail.1 == (head.1 + 2) {
        motion = (-1, -1);
    } else if tail.0 == (head.0 + 2) && tail.1 == (head.1 - 2) {
        motion = (-1, 1);
    } else if tail.0 == (head.0 - 2) && tail.1 == (head.1 + 2) {
        motion = (1, -1);
    } else if tail.0 == (head.0 - 2) && tail.1 == (head.1 - 2) {
        motion = (1, 1);
    }

    motion
}

fn add_coords(position: Coord, motion: Coord) -> Coord {
    (position.0 + motion.0, position.1 + motion.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 13);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 1);
    }

    #[test]
    fn test_part_2_longer() {
        let file_contents = fs::read_to_string("data/demo_input_2.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 36);
    }
}
