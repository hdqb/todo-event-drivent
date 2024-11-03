mod data;
mod events;
mod unit;
mod traits;

use std::sync::{Arc, Mutex};
use std::thread;
use unit::store::Store;
use unit::work::Work;
use unit::handler::Handler;
use events::link::Link;
use data::item::Item;

// file main.rs

fn main() {
    let link = Link::new();
    let store = Store::<Item>::initialize();
    let work = Arc::new(Mutex::new(Work::initialize(store)));
    let handler = Handler::new(link, work);

    handler.initialize();

    // Publish events to simulate CRUD operations with specific topics
    handler.publish("item.add", "1,Sample Item");
    handler.publish("item.get", "1"); // Lấy item với ID 1
    handler.publish("item.set", "1,Updated Item"); // Cập nhật item với ID 1
    handler.publish("item.drop", "1"); // Xóa item với ID 1

    // Thêm một khoảng thời gian ngủ để chờ xử lý sự kiện
    thread::sleep(std::time::Duration::from_secs(2));

    // Kết thúc chương trình
    println!("Exiting program.");
}
