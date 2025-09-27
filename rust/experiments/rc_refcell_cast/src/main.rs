/// This example demonstrates how to coerce an Rc<RefCell<T>> into one
/// of its implemented traits. The machinery is quite simpler than
/// one would have guessed.


use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;


trait A {
    fn a(&self) -> bool;
}

trait B {
    fn b(&self) -> char;
}

#[derive(Clone)]
struct D {a: bool, b: char,}

impl D {
    pub fn new() -> Self { D {a:true, b:'b',} }
}

impl A for D {
    fn a(&self) -> bool { self.a }
}

impl B for D {
    fn b(&self) -> char { self.b }
}

fn main() {
    let d = Rc::new(RefCell::new(D::new()));
    let a: Rc<RefCell<dyn A>> = d.clone() as Rc<RefCell<dyn A>>;
    println!("a.a(): {}", a.deref().borrow().a());
    let b: Rc<RefCell<dyn B>> = d.clone() as Rc<RefCell<dyn B>>;
    println!("b.b(): {}", b.deref().borrow().b());
    println!("And that's it!");
}
