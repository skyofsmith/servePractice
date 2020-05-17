// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会造成数据竞争和不一致。
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
fn main() {
    {
        // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // let b = Cons(3, Box::new(a));
        // let c = Cons(3, Box::new(a));    // error

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(3, a.clone());
        println!("b = {:#?}, c = {:#?}", b, c);
    }
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        
        let b = Cons(3, Rc::clone(&a));
        println!("count after bind to b = {}", Rc::strong_count(&a));

        {
            let c = Cons(3, a.clone());
            println!("count after bind to c = {}", Rc::strong_count(&a));
        }

        println!("count at end, a = {}", Rc::strong_count(&a));
    }
}
