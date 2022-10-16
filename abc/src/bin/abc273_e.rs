use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;
use proconio::input;

struct Node {
    value: isize,
    parent: Option<Rc<Node>>,
}

fn main() {
    input! {
        q: usize,
    }

    let mut result = Vec::new();

    let root = Node {
        value: -1,
        parent: None,
    };

    let root_pointer = Rc::new(root);
    let mut current = root_pointer.clone();

    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            s: String,
        }

        match s.as_str() {
            "ADD" => {
                input! {
                    x: isize,
                }

                let new_node = Node {
                    value: x,
                    parent: Some(current.clone()),
                };

                let new_ref = Rc::new(new_node);

                current = new_ref.clone();
            }
            "DELETE" => {
                if let Some(p) = &current.parent {
                    current = p.clone();
                } else {
                    // parent が None は root
                    current = root_pointer.clone();
                }
            }
            "SAVE" => {
                input! {
                    y: usize,
                }

                map.insert(y, current.clone());
            }
            _ => {
                input! {
                    z: usize,
                }

                if let Some(p) = map.get(&z) {
                    current = p.clone();
                } else {
                    current = root_pointer.clone();
                }
            }
        }

        result.push(current.value);
    }

    let text = result.iter().map(|x| x.to_string()).join(" ");

    println!("{}", text);
}
