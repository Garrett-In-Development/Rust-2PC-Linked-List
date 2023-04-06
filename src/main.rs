struct Node {
    value: String,
    next: Option<Box<Node>>,
}

struct MyList {
    head: Option<Box<Node>>,
    size: i32
}

impl MyList {
    fn add(&mut self, i_key:i32, string1:String) -> bool {

        if self.size == 0 {
            self.head = Some(Box::new(Node{
                value: string1,
                next: None
            }));
            self.size += 1;
            true
        }
        else {
            if i_key > self.size {
                false
            }
            else {
                let mut index: i32 = 0;
                let ref mut head: Option<Box<Node>> = self.head;
                while index < i_key {
                    match head {
                        None => None,
                        Some(i) => {
                            println!("{}", i.value);
                            match i.next {
                                None => None,
                                Some(j) => {
                                    println!("{}", j.value);
                                    head = Some(j);
                                    Some(j)
                                }
                            }
                        }
                    };
                    index += 1;
                }

                // match head {
                //     None => println!("Null"),
                //     Some(ref node) => println!("{}", node.value)
                // };

                true
            }

        }

    }

    // fn delete(&mut self, i_key:i32) {

    // }
}


// fn commit() {

// }

// fn rollback() {

// }

fn main() {
    let mut list = MyList {
        head: Some(Box::new(Node{
            value: String::from("Hello World"),
            next: Some(Box::new(Node{
                value: String::from("Second Node"),
                next: None,
            })),
        })),
        size: 2
    };

    list.add(2, String::from("Third Node"));
    list.add(3, String::from("Fourth Node"));
}
