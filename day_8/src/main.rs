#![warn(clippy::all, clippy::pedantic)]

mod tree;

use crate::tree::{Node, NodeRef};
use core::panic;
use num::integer::lcm;
use scan_fmt::scan_fmt_some;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let instructions = parse_instructions(INPUT);
    // println!("Instructions: {instructions:?}");

    let trees = parse_trees(INPUT);

    let loop_sizes: Vec<_> = trees
        .iter()
        .map(|node| get_loop_size(node, instructions.clone()))
        .collect();

    println!("Loop sizes: {:?}", loop_sizes);

    let collective_loop_size = loop_sizes.iter().fold(1, |acc, x| lcm(acc, *x));

    println!("Step count where all nodes end in Z: {collective_loop_size}");
}

fn get_loop_size(node: &NodeRef<String>, instructions: Vec<Instruction>) -> usize {
    let mut current_node = Rc::clone(node);
    let mut steps = 0;
    while !RefCell::borrow(&current_node).val.ends_with('Z') {
        let current_instruction = instructions[steps % instructions.len()];
        current_node = step_through_node(&current_node, current_instruction);
        steps += 1;
    }
    steps
}

fn step_through_node(
    current_node: &NodeRef<String>,
    current_instruction: Instruction,
) -> NodeRef<String> {
    match current_instruction {
        Instruction::Left => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
        Instruction::Right => Rc::clone(current_node.borrow().right.as_ref().unwrap()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Left,
    Right,
}

// Get the line of instructions from the input file.
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction: {}", c),
        })
        .collect()
}

fn parse_line(line: &str) -> (String, String, String) {
    match scan_fmt_some!(line, "{} = ({}, {})", String, String, String) {
        (Some(node), Some(left), Some(right)) => (node, left, right),
        _ => panic!("Invalid line: {}", line),
    }
}

fn parse_trees(input: &str) -> Vec<NodeRef<String>> {
    let mut hash_map: HashMap<String, NodeRef<String>> = HashMap::new();
    let mut start_nodes = Vec::new();
    for line in input.lines().skip(2) {
        let (node_name, left_name, right_name) = parse_line(line);

        hash_map
            .entry(node_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Node::new(node_name.clone()))));

        hash_map
            .entry(left_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Node::new(left_name.clone()))));

        hash_map
            .entry(right_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Node::new(right_name.clone()))));

        let node = hash_map.get(&node_name).unwrap();
        let left = hash_map.get(&left_name).unwrap();
        let right = hash_map.get(&right_name).unwrap();

        if node_name != left_name {
            RefCell::borrow_mut(node).set_left(Rc::clone(left));
        }
        if node_name != right_name {
            RefCell::borrow_mut(node).set_right(Rc::clone(right));
        }

        if node_name.ends_with('A') {
            start_nodes.push(node_name);
        }
    }

    start_nodes
        .iter()
        .map(|node_name| Rc::clone(hash_map.get(node_name).unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_instructions("RLRLLRR\n\ngibberish");
        assert_eq!(
            instructions,
            vec![
                Instruction::Right,
                Instruction::Left,
                Instruction::Right,
                Instruction::Left,
                Instruction::Left,
                Instruction::Right,
                Instruction::Right,
            ]
        );
    }

    #[test]
    fn test_parse_line() {
        let line = "AAA = (BBB, CCC)";
        let (node, left, right) = parse_line(line);
        assert_eq!(node, "AAA");
        assert_eq!(left, "BBB");
        assert_eq!(right, "CCC");
    }

    #[test]
    fn test_parse_tree() {
        let input = "instructions\n\nAAA = (B, C)\nB = (D, E)\n";
        let trees = parse_trees(input);
        let root = RefCell::borrow(&trees[0]);
        assert_eq!(root.val, "AAA");
        let left = RefCell::borrow(&root.left.as_ref().unwrap());
        assert_eq!(left.val, "B");
        let right = RefCell::borrow(&root.right.as_ref().unwrap());
        assert_eq!(right.val, "C");

        let left_left = RefCell::borrow(&left.left.as_ref().unwrap());
        assert_eq!(left_left.val, "D");
        let left_right = RefCell::borrow(&left.right.as_ref().unwrap());
        assert_eq!(left_right.val, "E");

        assert!(right.left.is_none());

        let input = "instructions\n\nAAA = (BBB, CCC)\nBBB = (BBB, BBB)";
        let trees = parse_trees(input);
        let root = RefCell::borrow(&trees[0]);
        assert_eq!(root.val, "AAA");
        let left = RefCell::borrow(&root.left.as_ref().unwrap());
        assert_eq!(left.val, "BBB");
        let right = RefCell::borrow(&root.right.as_ref().unwrap());
        assert_eq!(right.val, "CCC");

        assert!(left.left.is_none());
        assert!(left.right.is_none());
    }
}
