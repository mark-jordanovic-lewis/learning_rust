// TODO: find a way to implement a binary tree sort. Nearly there, just need to get the value out of Box

use std;
// #![feature(box_leak)] // cannot use in stable: only nightly
type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}


impl <T: PartialOrd> Node<T>
where T: std::fmt::Debug + std::clone::Clone {
    pub fn new(t: T) -> Node<T> {
        Node { payload: t, left: None, right: None }
    }
    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }
    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node)
    }
    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node)
    }
    pub fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut node) => node.insert(data),
                None => self.set_left(Self::new(data))
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(data),
                None => self.set_right(Self::new(data))
            }
        }
    }
    // pub fn sort(&self) -> Node<T> {
    //     let mut insertions: Vec<T> = Vec::new();
    //     order(self, insertions);
    //     let new_root = Self::new(insertions[0].clone());
    //     for payload in insertions.drain(1..) {
    //         new_root.insert(payload);
    //     }
    //     new_root
    // }
    pub fn visit(&self) {
        if let Some(ref left) = self.left {
            left.visit()
        }
        println!("{:?}", self.payload);
        if let Some(ref right) = self.right {
            right.visit()
        }
    }
}
// fn order<T>(current: &Node<T>, mut insertions: Vec<T>) {
//     if let Some(ref left) = current.left {
//         let next = Box::leak(current.left.unwrap());
//         order(next, insertions);
//     }
//     insertions.push(current.payload);
//     if let Some(ref right) = current.right {
//         let next = Box::leak(current.left.unwrap());
//         order(next, insertions);
//     }
// }

fn main() {

}
