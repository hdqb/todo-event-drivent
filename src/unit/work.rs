// file work.rs

use std::collections::HashMap;
use crate::events::event::Event;
use crate::unit::store::Store;
use crate::traits::initializable::Initializable;

pub struct Work<T> {
    store: Store<T>,
}

impl<T: Initializable + Clone + std::fmt::Debug> Work<T> {
    pub fn initialize(store: Store<T>) -> Self {
        Work { store }
    }

    pub fn process(&mut self, event: Event, initializer: fn(usize, String) -> T) -> String {
        match event.topic.as_str() {
            "item.add" => {
                let parts: Vec<&str> = event.payload.split(',').collect();
                if parts.len() < 2 {
                    return "Invalid payload".to_string();
                }
                let id: usize = self.store.next_id();
                let title = parts[0].to_string(); // Giả sử title là phần đầu tiên
                let item = initializer(id, title);
                self.store.add(item);
                format!("Added item with id: {}", id)
            },
            "item.get" => {
                let parts: Vec<&str> = event.payload.split(',').collect();
                let id: usize = parts[0].parse().unwrap_or(0);
                if let Some(item) = self.store.get(id) {
                    format!("Found item: {:?}", item)
                } else {
                    "Item not found".to_string()
                }
            },
            "item.set" => {
                let parts: Vec<&str> = event.payload.split(',').collect();
                if parts.len() < 2 {
                    return "Invalid payload".to_string();
                }
                let id: usize = parts[0].parse().unwrap_or(0);
                let title = parts[1].to_string();
                if let Some(_) = self.store.set(id, title) {
                    "Item updated".to_string()
                } else {
                    "Item not found".to_string()
                }
            },
            "item.drop" => {
                let parts: Vec<&str> = event.payload.split(',').collect();
                let id: usize = parts[0].parse().unwrap_or(0);
                if self.store.drop(id).is_some() {
                    "Item deleted".to_string()
                } else {
                    "Item not found".to_string()
                }
            },
            _ => "Unknown event topic".to_string(),
        }
    }
}
