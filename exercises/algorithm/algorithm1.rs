/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /*pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        //TODO
        let merged = LinkedList::<T>::new();
        for index in 0..list_a.length {
            merged.add((list_a.get(index as i32)).unwrap());
        }
        for index in 0..list_b.length {
            merged.add((list_b.get(index as i32)).unwrap());
        }
        merged
    }*/

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        // 获取两个链表的起始节点
        let mut node_a = list_a.start.take();
        let mut node_b = list_b.start.take();

        // 合并两个链表
        while let (Some(ptr_a), Some(ptr_b)) = (node_a, node_b) {
            unsafe {
                if (*ptr_a.as_ptr()).val <= (*ptr_b.as_ptr()).val {
                    let next_a = (*ptr_a.as_ptr()).next.take();
                    merged_list.append_node(ptr_a);
                    node_a = next_a;
                } else {
                    let next_b = (*ptr_b.as_ptr()).next.take();
                    merged_list.append_node(ptr_b);
                    node_b = next_b;
                }
            }
        }

        // 如果 `list_a` 还有剩余节点，直接链接到合并链表中
        while let Some(ptr_a) = node_a {
            unsafe {
                let next_a = (*ptr_a.as_ptr()).next.take();
                merged_list.append_node(ptr_a);
                node_a = next_a;
            }
        }

        // 如果 `list_b` 还有剩余节点，直接链接到合并链表中
        while let Some(ptr_b) = node_b {
            unsafe {
                let next_b = (*ptr_b.as_ptr()).next.take();
                merged_list.append_node(ptr_b);
                node_b = next_b;
            }
        }

        merged_list
    }

    // 辅助方法：将节点直接追加到链表末尾
    fn append_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            if let Some(end_ptr) = self.end {
                (*end_ptr.as_ptr()).next = Some(node);
            } else {
                self.start = Some(node);
            }
            self.end = Some(node);
            self.length += 1;
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
