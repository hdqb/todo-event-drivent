use crate::traits::initializable::Initializable;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: usize,
    pub title: String,
}

impl Initializable for Item {
    fn initialize(id: usize, title: String) -> Self {
        Item { id, title }
    }
}
