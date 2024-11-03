// file store.rs

use std::collections::HashMap;
use crate::traits::initializable::Initializable;

pub struct Store<T> {
    items: HashMap<usize, T>,
    next_id: usize,
}

impl<T: Initializable> Store<T> {
    pub fn initialize() -> Self {
        Store {
            items: HashMap::new(),
            next_id: 1, // Bắt đầu ID từ 1
        }
    }

    pub fn add(&mut self, item: T) {
        let id = self.next_id;
        self.items.insert(id, item);
        self.next_id += 1; // Tăng ID cho item tiếp theo
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.items.get(&id)
    }

    pub fn set(&mut self, id: usize, title: String) -> Option<()> {
        if let Some(item) = self.items.get_mut(&id) {
            *item = T::initialize(id, title); // Cập nhật item
            Some(())
        } else {
            None
        }
    }

    pub fn drop(&mut self, id: usize) -> Option<T> {
        self.items.remove(&id)
    }

    pub fn next_id(&self) -> usize {
        self.next_id
    }
}
