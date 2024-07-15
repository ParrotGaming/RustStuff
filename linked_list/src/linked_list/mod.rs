mod node;
use std::ptr::NonNull;

use node::LinkedListNode;

pub struct LinkedList {
    head: Option<NonNull<LinkedListNode>>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
        }
    }

    pub fn append(&mut self, d: String) {
        let mut node = Box::new(LinkedListNode::new(None, None, d.clone()));
        
        if self.head.is_none() {
            let tmp: NonNull<LinkedListNode> = Box::leak(node).into();
            self.head = Some(tmp);
        } else {
            let mut tmp: Option<NonNull<LinkedListNode>> = unsafe {self.head.expect("ERROR").as_mut().next};
            let mut prev: Option<NonNull<LinkedListNode>> = self.head;
            while !tmp.is_none() {
                prev = tmp;
                tmp = unsafe {tmp.expect("ERROR").as_mut().next};
            }
            node = Box::new(LinkedListNode::new(prev, None, d.clone()));
            unsafe { prev.expect("ERROR").as_mut().next = Some(Box::leak(node).into())}
            node = Box::new(LinkedListNode::new(prev, None, d));
            tmp = Some(Box::leak(node).into());
        }
    }

    pub fn print(&mut self) {
        if self.head.is_none() {
            return;
        } else {
            unsafe {print!("{}",self.head.expect("ERROR").as_ref().data);}
            let mut tmp: Option<NonNull<LinkedListNode>> = unsafe {self.head.expect("ERROR").as_mut().next};
            while !tmp.is_none() {
                unsafe {print!("{}",tmp.expect("ERROR").as_ref().data);}
                tmp = unsafe {tmp.expect("ERROR").as_mut().next};
            }
            println!();
        }
    }
}