use id_tree::*;
use std::env;
use std::fs;
use std::path::PathBuf;

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
    let mut total = 0;
    let file_tree = file_tree(input);
    let root_node_id = file_tree.root_node_id().unwrap();
    let node_ids = file_tree.traverse_pre_order_ids(&root_node_id).unwrap();

    match part {
        Part::One => {
            for node_id in node_ids {
                let node = file_tree.get(&node_id).unwrap();
                let total_size = total_size(&file_tree, &node);
                if total_size <= 100000 && node.data().node_type == NodeType::Dir {
                    total = total + total_size;
                }
            }
        }
        Part::Two => {
            let root_node = file_tree.get(&root_node_id).unwrap();
            let disk_space = 70000000;
            let required_space = 30000000;
            let space_delta = required_space - (disk_space - total_size(&file_tree, &root_node));

            for node_id in node_ids {
                let node = file_tree.get(&node_id).unwrap();
                let total_size = total_size(&file_tree, &node);
                if total_size >= space_delta && node.data().node_type == NodeType::Dir {
                    if total_size < total || total == 0 {
                        total = total_size;
                    }
                }
            }
        }
    }

    total
}

fn file_tree(input: &str) -> Tree<FileTreeNode> {
    let lines = input.trim().split("\n");

    let mut file_tree = Tree::<FileTreeNode>::new();
    let root = file_tree
        .insert(
            Node::new(FileTreeNode {
                path: "/".into(),
                size: 0,
                node_type: NodeType::Dir,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();

    let mut cwd = root;

    for line in lines {
        if line.starts_with("$") {
            let split: Vec<&str> = line.split_whitespace().collect();
            let cmd = split[1];
            if cmd == "cd" {
                let target = split[2];
                match target {
                    ".." => {
                        // Potentially unsafe since we're cloning the ID it may not actually point
                        // to the same thing anymore by the time we're done but since we don't delete
                        // any nodes this seems fine. Same below.
                        // https://docs.rs/id_tree/latest/id_tree/struct.NodeId.html#potential-nodeid-issues
                        cwd = file_tree.get(&cwd).unwrap().parent().unwrap().clone();
                    }
                    "/" => {
                        cwd = file_tree.root_node_id().unwrap().clone();
                    }
                    _ => {
                        cwd = find_or_create_node(
                            &cwd,
                            &mut file_tree,
                            NodeType::Dir,
                            target.into(),
                            0,
                        )
                    }
                }
            }
        } else if line.chars().next().unwrap().is_numeric() {
            let split: Vec<&str> = line.split_whitespace().collect();
            let size = split[0].parse::<usize>().unwrap();
            let path = split[1];

            find_or_create_node(&cwd, &mut file_tree, NodeType::File, path.into(), size);
        }
    }
    file_tree
}

fn find_or_create_node(
    cwd: &NodeId,
    file_tree: &mut Tree<FileTreeNode>,
    node_type: NodeType,
    path: PathBuf,
    size: usize,
) -> NodeId {
    let current_node = file_tree.get(&cwd).unwrap();
    let node = Node::new(FileTreeNode {
        path,
        size,
        node_type,
    });

    let child = current_node
        .children()
        .iter()
        .find(|&c| file_tree.get(c).unwrap().data().path == node.data().path);
    match child {
        Some(c) => c.clone(),
        None => {
            let node = file_tree
                .insert(node, InsertBehavior::UnderNode(&cwd))
                .unwrap();
            node
        }
    }
}

fn total_size(tree: &Tree<FileTreeNode>, node: &Node<FileTreeNode>) -> usize {
    let mut total = node.data().size;
    for child in node.children() {
        total = total + total_size(tree, tree.get(child).unwrap());
    }
    total
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Dir,
    File,
}

#[derive(Debug)]
struct FileTreeNode {
    path: PathBuf,
    size: usize,
    node_type: NodeType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 95437);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 24933642);
    }
}
