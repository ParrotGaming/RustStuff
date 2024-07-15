use std::ptr::NonNull;

pub struct LinkedListNode {
    pub prev: Option<NonNull<LinkedListNode>>,
    pub next: Option<NonNull<LinkedListNode>>,
    pub data: String,
}

impl LinkedListNode {
    pub fn new(p: Option<NonNull<LinkedListNode>>, n: Option<NonNull<LinkedListNode>>, d: String) -> LinkedListNode {
        LinkedListNode {
            prev: p,
            next: n,
            data: d,
        }
    }
}