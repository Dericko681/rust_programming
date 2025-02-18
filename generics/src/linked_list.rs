use std::thread::current;

use crate::adt_list::ADTList;

#[derive(Debug, Clone)]
pub struct Node<T: PartialEq + Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}
#[derive(Debug, Clone)]
pub struct LinkedList<T: PartialEq + Copy> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy + PartialEq> ADTList<T> for LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn get(&self, pos: u32) -> Option<T> {
        let mut iterator = 0_u32;
        let mut current_node = &self.head;
        while iterator <= pos {
            if let Some(node) = current_node {
                if iterator == pos {
                    return Some(node.data);
                }
                if let Some(_) = &node.next {
                    current_node = &node.next;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iterator = iterator - 1;
        }
        None
    }

    fn insert(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.clone(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn remove(&mut self, pos: u32, data: T) {
        if self.head.is_none() {
            return;
        }
        let head = self.head.as_ref().unwrap();
        while let Some(ref mut curr_node) = node {
            if let Some(next_node) = &mut curr_node.next {
                if next_node.data == data {
                    curr_node.next = next_node.next.clone();
                    break;
                }
            }
            node = &mut curr_node.next
        }
    }

    fn pop(&mut self) {
        //    let mut node = &mut self.head;
        //    if node.is_none(){
        //     return;
        //    }
        //    if node.as_ref().unwrap().next.is_none(){
        //     *node = None;
        //     return;
        //    }
        //    while let Some(ref mut curr_node) = node {
        //     if let Some(next_node) = &curr_node.next{
        //         if next_node.next.is_none(){
        //             curr_node.next = None;
        //             break;
        //         }else {
        //             node = &mut curr_node.next  ;
        //         }
        //     }
        // }
    }

    fn remove_at(&self, pos: u32) -> Option<T> {
        let mut iterator = 0;
        let mut curr_node = self.head.clone();
        while iterator >= pos - 1 {
            if iterator == pos - 1 {
                let mut var = (curr_node.unwrap().next).unwrap().next;
                curr_node.unwrap().next.unwrap().next = None;
                curr_node.unwrap().next = var;
            } else {
                iterator + 1;
            }
        }
        return Some(&curr_node.unwrap().data);
    }

    fn replace(&self, data: T, pos: u32) -> Self {
        todo!()
    }

    fn size(&self) -> u32 {
        todo!()
    }

    fn isEmpty(&self) -> bool {
        todo!()
    }
}
