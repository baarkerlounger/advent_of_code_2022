use enum_iterator::{all, Sequence};
use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result(&file_contents, Part::One)),
            // 2 => println!("Result for part 2 is {}", result(&file_contents, Part::Two)),
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result(&file_contents, Part::One));
        // println!("Result for part 2 is {}", result(&file_contents, Part::Two));
    }
}

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> usize {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    // let mut grid: Vec<Vec<Node>> = Vec::new();
    let grid = Grid {
        height: lines.len(),
        width: lines[0].len(),
        content: input.chars().filter(|c| *c != '\n').collect(),
    };

    let unvisited = grid.coordinates();
    let distances = grid.initial_distances();
    let current = grid.starting_position();

    for direction in all::<Direction>() {
        let next_pos = grid.next_position(current, direction);
        if next_pos == None {
            continue;
        }
    }

    println!("{:#?}", distances);
    println!("{:#?}", unvisited);
    println!("{:#?}", grid);

    unimplemented!()
}

type Position = (isize, isize);

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    content: Vec<char>,
}

impl Grid {
    fn coordinates(&self) -> Vec<Position> {
        self.content
            .iter()
            .enumerate()
            .map(|(idx, _)| ((idx / self.width) as isize, (idx % self.width) as isize))
            .collect()
    }

    fn initial_distances(&self) -> Vec<usize> {
        self.content
            .iter()
            .map(|c| match c {
                'S' => 0,
                _ => usize::MAX,
            })
            .collect()
    }

    fn starting_position(&self) -> Position {
        for (idx, c) in self.content.iter().enumerate() {
            if *c == 'S' {
                return ((idx / self.width) as isize, (idx % self.width) as isize);
            }
        }
        panic!()
    }

    fn next_position(&self, current: Position, direction: Direction) -> Option<Position> {
        let next = match direction {
            Direction::Up => (current.0, current.1 - 1),
            Direction::Down => (current.0, current.1 + 1),
            Direction::Left => (current.0 - 1, current.1),
            Direction::Right => (current.0 + 1, current.1),
        };

        if next.0 < 0 || next.1 < 0 {
            None
        } else {
            Some(next)
        }
    }
}

#[derive(Debug, Sequence)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 31);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
    //     assert_eq!(result(&file_contents, Part::Two), 1);
    // }
}
