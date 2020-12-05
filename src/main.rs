mod tree_lib;
pub use crate::tree_lib::tree::{ BTree };
pub use crate::tree_lib::node::{ Node, NodeOption };
use std::io::{ self };
use std::collections::{ LinkedList };
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Minimax Algorithm:");
    let mut tree = BTree::new_empty();
    match create_tree(&mut tree) {
        false => println!("Failed to create tree!"),
        true => ()
    };
    print_tree(&tree);
    println!("Minimax result: {}", minimax(&tree.root, true));
    println!("Pruning result: {}",
             alpha_beta_pruning(&mut tree.root,
                                std::i32::MIN,
                                std::i32::MAX,
                                true));
    print_tree(&tree);
}


fn create_tree(tree : &mut BTree) -> bool {
    // Get input
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error reading input");
    let split = buffer.split_whitespace();

    let mut nodes : LinkedList<NodeOption> = LinkedList::new();

    // Decode input into Nodes
    for inpt in split {
        let inptint : i32 = inpt.parse::<i32>().expect("Failed to parse i32");
        let newnode : NodeOption = Some(Node::new(inptint, true, None, None));
        nodes.push_back(newnode);
    }

    // Check length of the input
    let flen = nodes.len() as f64;
    if flen.log2().ceil() != flen.log2().floor() {
        println!("This is not a fully binary tree");
        return false;
    }

    // Generate binary tree from leaf values
    while nodes.len() != 1 {
        let mut par_nodes : LinkedList<NodeOption> = LinkedList::new();
        while !nodes.is_empty() {
            let left_node : NodeOption = match nodes.pop_front() {
                None => None,
                Some(node) => node
            };

            let right_node : NodeOption = match nodes.pop_front() {
                None => None,
                Some(node) => node
            };

            let parent_node : NodeOption = Some(Node::new(0, true, left_node, right_node));
            par_nodes.push_back(parent_node);
        }
        nodes = par_nodes;
    }

    tree.set_root(match nodes.pop_front() {
        None => None,
        Some(node) => node
    });

    true
}


fn print_tree(tree : &BTree) {
    if !tree.is_left_none() && !tree.is_right_none() {
        print_node(&tree.root);
    } else {
        match tree.get_root_val() {
            None => println!("No value in root"),
            Some(val) => println!("[{}]", val)
        }
    }
    print!("\n");
}

fn print_node(node : &NodeOption) {
    match node {
        None => println!("No node found!"),
        Some(node) =>
        if node.borrow().is_left_none() && node.borrow().is_right_none() {
            print!("{}", node.borrow().get_val());
        } else {
            print!("[");
            if !node.borrow().is_left_none() {
                print_node(&node.borrow().left);
            }
            if !node.borrow().is_left_none() && !node.borrow().is_right_none() {
                print!(",");
            }
            if !node.borrow().is_right_none() {
                print_node(&node.borrow().right);
            }
            print!("]");
        }
    }
}

fn minimax(node : &NodeOption, color : bool) -> i32 {
    match node {
        None => 0,
        Some(node) =>
            if node.borrow().is_leaf() {
                // Leaf case
                return node.borrow().get_val();
            } else {
                let mut score_l = std::i32::MIN;
                let mut score_r = std::i32::MIN;

                // Update scores
                if !node.borrow().is_left_none() {
                    score_l = minimax(&node.borrow().left, !color);
                }
                if !node.borrow().is_right_none() {
                    score_r = minimax(&node.borrow().right, !color);
                }

                // Decide based on color
                if color {
                    return std::cmp::max(score_l, score_r);
                } else {
                    return std::cmp::min(score_l, score_r);
                }
            }
    }
}

fn alpha_beta_pruning(node : &mut NodeOption, alpha : i32, beta : i32, color : bool) -> i32 {
    match node {
        None => 0,
        Some(node) =>
            if node.borrow().is_leaf() {
                return node.borrow().get_val();
            } else {
                let mut nalpha = alpha;
                let mut nbeta = beta;
                let mut score_r = std::i32::MIN;
                let score_l = alpha_beta_pruning(
                    &mut RefCell::get_mut(Rc::get_mut(node).unwrap()).left,
                    alpha,
                    beta,
                    !color);

                if color && !node.borrow().is_left_none() {
                    nalpha = std::cmp::max(alpha, score_l);
                } else if !node.borrow().is_left_none() {
                    nbeta = std::cmp::min(beta, score_l);
                }

                if nbeta <= nalpha {
                    RefCell::get_mut(Rc::get_mut(node).unwrap()).remove_right();
                } else {
                    score_r = alpha_beta_pruning(
                        &mut RefCell::get_mut(Rc::get_mut(node).unwrap()).right,
                        nalpha,
                        nbeta,
                        !color);
                }

                if color {
                    return std::cmp::max(score_l, score_r);
                } else {
                    return std::cmp::min(score_l, score_r);
                }
            }
    }
}
