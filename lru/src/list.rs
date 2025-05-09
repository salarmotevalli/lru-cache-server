use super::value::Value;
use std::{collections::HashMap, fmt::{self}, ptr::NonNull};

pub type NodeMap = HashMap<String, NonNull<Node>>;

// #[derive()]
pub struct List {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = f.debug_struct("List");
        result.field("len", &self.len);

        let mut values = Vec::new();
        let mut current = self.head;

        unsafe {
            while let Some(node_ptr) = current {
                let node = node_ptr.as_ref();
                values.push((&node.key, &node.value));
                current = node.next;
            }
        }

        result.field("nodes", &values);
        result.finish()
    }
}

#[derive(Clone)]
pub struct Node {
    pub next: Option<NonNull<Node>>,
    pub prev: Option<NonNull<Node>>,
    pub value: Value,
    key: String,
}

unsafe impl Send for Node {}
unsafe impl Sync for Node {}

impl Node {
    pub fn new(key: String, value: Value) -> Self {
        Self {
            next: None,
            prev: None,
            key,
            value,
        }
    }

    pub fn is_tail(&self) -> bool {
        self.next.is_none()
    }

    pub fn is_head(&self) -> bool {
        self.prev.is_none()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_front_node(&mut self, node_ptr: NonNull<Node>) {
        match self.head {
            None => self.tail = Some(node_ptr),
            // Not creating new mutable (unique!) references overlapping `element`.
            Some(mut head) => unsafe { head.as_mut().prev = Some(node_ptr) },
        }

        self.head = Some(node_ptr);
        self.len += 1;
    }

    pub fn move_front(&mut self, mut node_ptr: NonNull<Node>) {
        unsafe {
            let node = node_ptr.as_mut();

            if self.len == 1 || node.is_head() {
                return;
            }

            if node.is_tail() {
                self.pop_back();
            } else {
                if let Some(mut prev_node) = node.prev {
                    let prev_node_mut = prev_node.as_mut();
                    prev_node_mut.next = node.next;
                }
    
                // Update the next node's previous pointer, if applicable
                if let Some(mut next_node) = node.next {
                    let next_node_mut = next_node.as_mut();
                    next_node_mut.prev = node.prev;
                }

                self.len -= 1;
            }

            node.next = self.head;
            node.prev = None;

            self.push_front_node(node_ptr);
        }
    }

    pub fn push_front(&mut self, key: String, value: Value) -> NonNull<Node> {
        let mut node = Node::new(key, value);

        node.next = self.head;
        node.prev = None;

        let node = Box::new(node);
        let node_ptr = NonNull::from(Box::leak(node));

        self.push_front_node(node_ptr);

        node_ptr
    }

    pub fn pop_back(&mut self) -> Option<String> {
        self.tail.map(|node| unsafe {
            // let poped_node = Box::from_raw(node.as_ptr());
            let key = node.as_ref().key.clone();
            self.tail = node.as_ref().prev;

            match self.tail {
                None => self.head = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(mut tail) => tail.as_mut().next = None,
            }

            self.len -= 1;
            
            key
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}



#[cfg(test)]
mod test {
    use std::ptr::NonNull;

    use super::Node;

    
    #[test]
    fn test_node_is_tail_works_fine() {
        let mut n = Node::new("test".to_string(), crate::value::Value::Bool(true));

        assert_eq!(n.is_tail(), true);
    
        let n2= Node::new("test1".to_string(), crate::value::Value::Bool(false));

        let boxed_n = Box::new(n2);

        n.next =  NonNull::new(Box::leak(boxed_n.clone()));
        
        assert_eq!(n.is_tail(), false);

        assert_eq!((*boxed_n).is_tail(), true);

    }
}