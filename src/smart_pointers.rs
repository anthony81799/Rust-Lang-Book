use std::{ops::Deref, rc::Rc, cell::RefCell, fmt::Debug};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox (x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPtr {
    data: String
}

impl Drop for CustomSmartPtr {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPtr with data{}!", self.data);
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use List::*;

pub fn run() {
    let x = 5;
    let y = MyBox::new(x);

    // you can dereference MyBox because it implements the deref trait
    assert_ne!(6, x);
    assert_ne!(6, *y);

    let _c = CustomSmartPtr{
        data: String::from("my stuff")
    };
    let _d = CustomSmartPtr{
        data: String::from("other stuff")
    };
    println!("CustomSmartPtrs created.");
    // clean up early use the drop function in std::mem::drop e.g. drop(c)
    // does not need to be brought into scope

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // Rc::clone incriments the reference count
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let e = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("e after = {:?}", e);
}
