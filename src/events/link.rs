use std::sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex};
use crate::events::event::Event;

pub struct Link {
    pub sender: Sender<Event>,
    pub receiver: Arc<Mutex<Receiver<Event>>>,
}

impl Link {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Link { sender, receiver: Arc::new(Mutex::new(receiver)) }
    }

    pub fn publish(&self, event: Event) {
        self.sender.send(event).expect("Failed to send event");
    }

    pub fn subscribe(&self, callback: impl Fn(Event) + Send + 'static) {
        let receiver = Arc::clone(&self.receiver);
        std::thread::spawn(move || {
            let receiver = receiver.lock().unwrap();
            while let Ok(event) = receiver.recv() {
                callback(event);
            }
        });
    }
}
