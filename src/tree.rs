use std::{rc::{Rc, Weak}, cell::RefCell};

#[derive(Debug)]
struct Node {
    _value: i32,
    parent: RefCell<Weak<Node>>,
    _children: RefCell<Vec<Rc<Node>>>
}

pub fn run() {
    let leaf = Rc::new(Node {
        _value: 3,
        parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![])
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        _value: 5,
        parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
