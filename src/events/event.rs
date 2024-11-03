#[derive(Debug, Clone)]
pub struct Event {
    pub topic: String,  // Chủ đề của sự kiện
    pub payload: String, // Dữ liệu đi kèm với sự kiện
}
