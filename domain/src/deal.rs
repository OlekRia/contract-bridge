pub mod hand;

use hand::card::Card;

#[derive(Debug, Clone)]
pub struct Trick {
    pub cards: Vec<Card>,
}
