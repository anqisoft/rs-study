// 080.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/cell.html

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, ..Node::default() }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));

    let subtree = Node::new(10);
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));

    root.borrow_mut().children.push(subtree);

    println!("graph: {root:#?}");
    println!("graph sum: {}", root.borrow().sum());
}

/* result:
graph: RefCell {
    value: Node {
        value: 1,
        children: [
            RefCell {
                value: Node {
                    value: 5,
                    children: [],
                },
            },
            RefCell {
                value: Node {
                    value: 10,
                    children: [
                        RefCell {
                            value: Node {
                                value: 11,
                                children: [],
                            },
                        },
                        RefCell {
                            value: Node {
                                value: 12,
                                children: [],
                            },
                        },
                    ],
                },
            },
        ],
    },
}
graph sum: 39
*/
