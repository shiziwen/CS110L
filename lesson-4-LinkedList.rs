struct Node {
    value: u32,
    next: Option<Box<Node>>
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
    
    pub fn display(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        println!("{}, size: {}", result, self.size);
    }
    
    // impl fmt::Display for LinkedList {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let mut current: &Option<Box<Node>> = &self.head;
    //     let mut result = String::new();
    //     loop {
    //         match current {
    //             Some(node) => {
    //                 result = format!("{} {}", result, node.value);
    //                 current = &node.next;
    //             },
    //             None => break,
    //         }
    //     }
    //     write!(f, "{}", result)
    //     }
    // }
    

}

// impl Drop for LinkedList {
//     fn drop(&mut self) {
//         let current = self.head.take();
//         while let Some(mut node) = current {
//             current = node.next.take();
//         }
//     }
// }

fn main() {
    println!("Hello, world!");
    
    let mut list: LinkedList = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    
    // let mut x: Option<u32> = Some(5);
    // let x_ref: &mut Option<u32> = &mut x;
    
    for i in 1..10 {
        list.push(i);
    }
    list.display();
    
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop().unwrap());
    list.display();
    println!("size: {}", list.get_size());
    
}