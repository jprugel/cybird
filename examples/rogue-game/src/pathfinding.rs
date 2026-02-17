use rogue_lib::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::Logger;

pub struct Map {
    width: u16,
    height: u16,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        Map { width, height }
    }

    pub fn into_graph(&self) -> Graph {
        let mut graph = Graph::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let node = Node {
                    position: Vector2::new(x, y),
                };
                graph.add_node(node);
            }
        }
        graph
    }
}

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn get_node(&self, position: Vector2<u16>) -> Option<&Node> {
        self.nodes.iter().find(|node| node.position == position)
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn neighbors<'a>(&'a self, node: &'a Node) -> Vec<&'a Node> {
        let mut result = vec![];
        if let Some(index) = self.nodes.iter().position(|n| n == node) {
            let x = node.position.x;
            let y = node.position.y;
            if x > 0
                && let Some(node) = self.get_node(Vector2::new(x - 1, y))
            {
                result.push(node);
            }
            if let Some(node) = self.get_node(Vector2::new(x + 1, y)) {
                result.push(node);
            }
            if y > 0
                && let Some(node) = self.get_node(Vector2::new(x, y - 1))
            {
                result.push(node);
            }
            if let Some(node) = self.get_node(Vector2::new(x, y + 1)) {
                result.push(node);
            }
        }
        result
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Node {
    pub position: Vector2<u16>,
}

pub fn pathfind<'a>(
    start: &'a Node,
    goal: &'a Node,
    graph: &'a Graph,
    logger: &'a mut Logger,
) -> Option<Vec<&'a Node>> {
    let mut frontier = VecDeque::new();
    let mut reached = HashMap::<&'a Node, Option<&'a Node>>::new();
    frontier.push_back(start);
    reached.insert(start, None);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        for next in graph.neighbors(current) {
            logger.log(format!("{:?}", start));
            if !reached.contains_key(next) {
                frontier.push_back(next);
                reached.insert(next, Some(current));
            }
        }
    }

    let mut result = vec![];
    result.push(goal);
    let mut current = goal;
    while !result.contains(&start) {
        if let Some(next) = reached.get(current) {
            result.push(next.unwrap());
            current = next.unwrap();
        } else {
            return None;
        }
    }
    //result.reverse();
    Some(result)
}
