pub mod card;
mod side;
mod position;

use card::Card;
use side::Side;
use position::Position;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub side: Side,
    pub position: Option<Position>
}

impl Hand {
    /// Creates a Hand from a vector of card numbers and a side.
    /// Has a position and a side.
    ///
    /// # Panics
    /// Panics if the card numbers vector has more than 13 elements.
    ///
    /// # Example
    ///
    ///
    pub fn new(card_numbers: Vec<u8>, side: Side, position: Option<Position>) -> Self {
        if card_numbers.len() > 13 {
            panic!("A hand cannot have more than 13 cards");
        }

        let mut cards = card_numbers
            .into_iter()
            .map(Card::new)
            .collect::<Vec<Card>>();

        cards.sort_by(|a, b| b.partial_cmp(a).unwrap());

        Self { cards, side, position }
    }
}

#[cfg(test)]
mod tests {
    use crate::deal::hand::card::rank::Rank;
    use super::*;

    #[test]
    fn hand_new() {
        let hand = Hand::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], Side::North, None);
        assert_eq!(hand.cards.len(), 13);
        assert_eq!(hand.cards[0].value, Rank::Ace);
        assert_eq!(hand.cards[12].value, Rank::Two);
    }
}