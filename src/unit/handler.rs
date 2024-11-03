use crate::events::link::Link;
use crate::events::event::Event;
use std::sync::{Arc, Mutex};
use crate::unit::work::Work;
use std::fmt::Debug;
use crate::traits::initializable::Initializable;

pub struct Handler<T> {
    link: Link,
    work: Arc<Mutex<Work<T>>>,
}

impl<T: Clone + Debug + Initializable + 'static + std::marker::Send> Handler<T> {
    pub fn new(link: Link, work: Arc<Mutex<Work<T>>>) -> Self {
        Handler { link, work }
    }

    pub fn initialize(&self) {
        let work = Arc::clone(&self.work);
        self.link.subscribe(move |event: Event| {
            let response = work.lock().unwrap().process(event, |id, title| {
                T::initialize(id, title)
            });
            println!("{}", response);
        });
    }

    pub fn publish(&self, topic: &str, payload: &str) {
        let event = Event {
            topic: topic.to_string(),
            payload: payload.to_string(),
        };
        self.link.publish(event);
    }
}
