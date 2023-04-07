use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Node {
    value: String,
    next: Option<Arc<Mutex<Node>>>
}

impl Node {
    fn new(val: String) -> Node {
       Node {
        value: val,
        next: None,
       }
    }
}

pub struct MyList {
    head: Option<Arc<Mutex<Node>>>,
    size: usize,
}

impl MyList {

    fn new() -> MyList {
        MyList {
            head: None,
            size: 0
        }
    }

    fn add(&mut self, i_key: usize, string1: &str) -> Result<(), String> {
        let new_node = Arc::new(Mutex::new(Node::new(String::from(string1))));


    }

    fn delete(&mut self, i_key: usize) -> Result<(), String> {

    }

    fn commit() {

    }

    fn rollback() {

    }

}

fn main() {

    // let mut head: &Option<Box<Node>> = &list;

    // Traversal
    // while *head != None {
    //     match *head {
    //         None => (),
    //         Some(ref i) => {
    //             println!{"{}", i.value};
    //             head = &i.next;
    //         }
    //     }
    // }



}
