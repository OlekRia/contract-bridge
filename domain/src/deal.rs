use crate::deal::hand::Hand;

pub mod hand;

pub struct Deal {
    pub hands: [Hand; 4],
}

impl Deal {
    pub fn new(hands: [Hand; 4]) -> Self {
        Self { hands }
    }
}