use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    byte: Option<u8>,
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(byte: Option<u8>, freq: u32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            byte,
            freq,
            left,
            right,
        }
    }
    fn box_and_wrap(self) -> Option<Box<Node>> {
        let boxed_value = Box::new(self);
        Some(boxed_value)
    }
}

fn build_frequencies(input_data: Vec<u8>) -> HashMap<u8, u32> {
    let mut freq_table: HashMap<u8, u32> = HashMap::new();
    for byte in input_data {
        *freq_table.entry(byte).or_insert(0) += 1;
    }
    freq_table
}

fn build_priority_queue(freq_table: HashMap<u8, u32>) -> Vec<(u8, u32)> {
    let mut heap: Vec<(u8, u32)> = freq_table.into_iter().collect();
    heap.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    heap
}

fn reorganize_nodes_by_freq(mut nodes: Vec<Node>) -> Vec<Node> {
    nodes.sort_by(|a, b| b.freq.cmp(&a.freq));
    nodes
}

fn build_nodes(freq_table: Vec<(u8, u32)>) -> Vec<Node> {
    freq_table
        .into_iter()
        .map(|(byte, freq)| Node::new(Some(byte), freq, None, None))
        .collect()
}

fn build_tree(mut nodes: Vec<Node>) -> Vec<Node> {
    if nodes.len() == 1 {
        return nodes;
    }
    let left_node = nodes.pop().unwrap();
    let right_node = nodes.pop().unwrap();
    let parent_node = create_parent(left_node, right_node);
    nodes.push(parent_node.clone());
    let reorganized_nodes = reorganize_nodes_by_freq(nodes);
    build_tree(reorganized_nodes)
}

fn create_parent(left_node: Node, right_node: Node) -> Node {
    let parent_node_freq = left_node.freq + right_node.freq;
    let boxed_left_node = left_node.box_and_wrap();
    let boxed_right_node = right_node.box_and_wrap();
    Node::new(None, parent_node_freq, boxed_left_node, boxed_right_node)
}

pub fn create_tree(data: &[u8]) -> Node {
    let data_vec = data.to_vec();
    let huffman_frequencies = build_frequencies(data_vec);
    let priority_queue = build_priority_queue(huffman_frequencies);
    let nodes = build_nodes(priority_queue.clone());
    build_tree(nodes).pop().unwrap()
}
