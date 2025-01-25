pub mod card;

use card::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Creates a Hand from a vector of card numbers and sorts it in descending order.
    ///
    /// Panics if the given vector has more than 13 elements.
    ///
    pub fn new(card_numbers: Vec<u8>) -> Self {
        if card_numbers.len() > 13 {
            panic!("A hand cannot have more than 13 cards");
        }

        let mut cards = card_numbers
            .into_iter()
            .map(Card::new)
            .collect::<Vec<Card>>();

        cards.sort_by(|a, b| b.partial_cmp(a).unwrap());

        Self { cards }
    }
}

#[cfg(test)]
mod tests {
    use crate::deal::hand::card::rank::Rank;
    use super::*;

    #[test]
    fn hand_new() {
        let hand = Hand::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(hand.cards.len(), 13);
        assert_eq!(hand.cards[0].value, Rank::Ace);
        assert_eq!(hand.cards[12].value, Rank::Two);
    }
}