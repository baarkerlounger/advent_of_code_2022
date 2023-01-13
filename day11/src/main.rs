use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result_1(&file_contents)),
            // 2 => println!("Result for part 2 is {}", result_2(&file_contents)),
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result_1(&file_contents));
        // println!("Result for part 2 is {}", result_2(&file_contents));
    }
}

fn result_1(input: &str) -> u32 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: HashMap<usize, VecDeque<u32>> = HashMap::new();
    let mut inspections: HashMap<usize, u32> = HashMap::new();
    let num_round = 20;

    parse_input(input, &mut monkeys, &mut items, &mut inspections);

    for _round in 0..num_round {
        complete_round(&mut monkeys, &mut items, &mut inspections);
    }

    let mut hash_vec: Vec<(&usize, &u32)> = inspections.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    hash_vec[0].1 * hash_vec[1].1
}

// fn result_2(input: &str) -> String {
//     let lines: Vec<&str> = input.trim().split("\n").collect();
//
//     unimplemented!()
// }

fn parse_input(
    input: &str,
    monkeys: &mut Vec<Monkey>,
    items: &mut HashMap<usize, VecDeque<u32>>,
    inspections: &mut HashMap<usize, u32>,
) {
    let input_groups: Vec<&str> = input.trim().split("\n\n").collect();

    for input in input_groups {
        let lines: Vec<&str> = input.split("\n").collect();
        let starting_items = lines[1].split("Starting items: ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let num_regex = Regex::new(r"\d+").expect("valid regex");
        let op_regex = Regex::new(r"[*+-/]+").expect("valid regex");
        let id = num_regex.find(lines[0]).unwrap().as_str().parse().unwrap();
        let operator = String::from(op_regex.find(lines[2]).unwrap().as_str());
        let operand = match num_regex.find(lines[2]) {
            Some(mat) => Some(mat.as_str().parse::<u32>().unwrap()),
            _ => None,
        };
        let test = num_regex.find(lines[3]).unwrap().as_str().parse().unwrap();
        let test_true_monkey = num_regex.find(lines[4]).unwrap().as_str().parse().unwrap();
        let test_false_monkey = num_regex.find(lines[5]).unwrap().as_str().parse().unwrap();

        items.insert(id, starting_items);
        inspections.insert(id, 0);

        monkeys.push(Monkey::new(
            id,
            operator,
            operand,
            test,
            test_true_monkey,
            test_false_monkey,
        ));
    }
}

fn complete_round(
    monkeys: &mut Vec<Monkey>,
    items: &mut HashMap<usize, VecDeque<u32>>,
    inspections: &mut HashMap<usize, u32>,
) {
    for monkey in monkeys {
        monkey.inspect_items(items, inspections);
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    operator: String,
    operand: Option<u32>,
    test: u32,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl Monkey {
    fn new(
        id: usize,
        operator: String,
        operand: Option<u32>,
        test: u32,
        test_true_monkey: usize,
        test_false_monkey: usize,
    ) -> Self {
        Monkey {
            id,
            operator,
            operand,
            test,
            test_true_monkey,
            test_false_monkey,
        }
    }

    fn inspect_items(
        &self,
        items: &mut HashMap<usize, VecDeque<u32>>,
        inspections: &mut HashMap<usize, u32>,
    ) {
        let starting_items = &items[&self.id].clone();
        for item in starting_items {
            let val = match self.operand {
                Some(op) => op,
                _ => *item,
            };
            let mut new_item = *item;
            match self.operator.as_str() {
                "+" => {
                    new_item = new_item + val;
                }
                "-" => {
                    new_item = new_item - val;
                }
                "*" => {
                    new_item = new_item * val;
                }
                "/" => {
                    new_item = new_item / val;
                }
                _ => panic!(),
            }
            new_item = new_item / 3;
            let mut key = self.id.clone();
            *inspections.get_mut(&key).unwrap() += 1;
            items.get_mut(&key).unwrap().pop_front();
            if new_item % self.test == 0 {
                key = self.test_true_monkey;
            } else {
                key = self.test_false_monkey;
            }
            items.get_mut(&key).unwrap().push_back(new_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result_1(&file_contents), 10605);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
    //     assert_eq!(result_2(&file_contents), 13140);
    // }
}
