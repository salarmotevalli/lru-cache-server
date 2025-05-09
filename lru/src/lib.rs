pub(crate) mod list;
pub mod value;

use list::{List, NodeMap};
use std::collections::HashMap;
use value::Value;

#[derive(Debug)]
struct LruConfig {
    capacity: usize,
}

#[derive(Debug)]
pub struct Lru {
    refs: NodeMap,
    data: List,
    config: LruConfig,
}

unsafe impl Send for Lru {}
unsafe impl Sync for Lru {}

impl Lru {
    pub fn new() -> Self {
        Self {
            refs: HashMap::new(),
            data: List::new(),
            config: LruConfig { capacity: 5 },
        }
    }

    pub async fn set(&mut self, key: String, value: Value) {
        if self.config.capacity == self.data.len() {
            let poped_key = self.data.pop_back();
            poped_key.map(|key| {
                self.refs.remove(&key);
            });
        }

        let ptr = self.data.push_front(key.clone(), value);

        self.refs.insert(key, ptr);
    }

    pub async fn get(&mut self, key: String) -> Option<Value> {
        let rf: Option<&std::ptr::NonNull<list::Node>> = self.refs.get(&key);

        if rf.is_none() {
            return None;
        }

        let ptr: &std::ptr::NonNull<list::Node> = rf.unwrap();

        self.data.move_front(ptr.clone());

        unsafe {
            let node = ptr.as_ref();
            Some(node.value.clone()) // Requires Value: Clone
        }
    }
}

