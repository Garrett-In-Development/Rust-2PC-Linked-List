

#[derive(Clone, PartialEq)]
pub struct Node {
    value: String,
    next: Option<Box<Node>>
}

impl Node {
    fn new(val: String) -> Node {
       Node {
        value: val,
        next: None,
       }
    }
}

fn main() {
    let list = Some(Box::new(Node{value: String::from("Hello World"), next: Some(Box::new(Node::new(String::from("Second Node"))))}));

    let mut head: Box<&Option<Box<Node>>> = Box::new(&list);

    while *head != &None {
        match *head {
            None => (),
            Some(ref i) => {
                println!{"{}", i.value};
                *head = &i.next;
            }
        }
    }
}
