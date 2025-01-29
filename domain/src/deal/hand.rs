pub mod card;

use card::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Creates a Hand from a vector of card numbers.
    /// Sorts the cards in descending order for optimal performance and string view.
    ///
    /// # Panics
    /// If the card numbers vector has more than 13 elements.
    pub fn new(card_numbers: Vec<u8>) -> Self {
        assert!(
            card_numbers.len() <= 13,
            "A hand cannot have more than 13 cards"
        );

        let mut cards = card_numbers
            .into_iter()
            .map(Card::new)
            .collect::<Vec<Card>>();

        cards.sort_by(|a, b| b.cmp(a));

        Self { cards }
    }

    /// Removes a card from the hand, if it exists.
    pub fn remove_card(&mut self, card: Card) {
        // Use binary_search_by for descending order
        if let Ok(pos) = self.cards.binary_search_by(|c| c.cmp(&card).reverse()) {
            self.cards.remove(pos);
        }
    }

    /// Returns true if the given card is in the hand, false otherwise.
    pub fn contains(&self, card: &Card) -> bool {
        self.cards.binary_search_by(|c| c.cmp(card).reverse()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deal::hand::card::rank::Rank;
    use crate::deal::hand::card::suits::Suits;

    #[test]
    fn hand_new() {
        let hand = Hand::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(hand.cards.len(), 13);
        assert_eq!(hand.cards[0].rank, Rank::Ace);
        assert_eq!(hand.cards[12].rank, Rank::Two);
    }

    #[test]
    fn hand_remove_card() {
        let mut hand = Hand::new(vec![31, 22, 1, 2, 51, 3]);

        assert_eq!(hand.cards[0].rank, Rank::Ace);
        assert_eq!(hand.cards[0].suit, Suits::Spades);

        hand.remove_card(Card::new(22));
        
        assert_eq!(hand.cards.len(), 5);
        assert_eq!(hand.cards[0].to_string(), "sA");
        assert_eq!(hand.cards[1].to_string(), "h7");
        assert_eq!(hand.cards[2].to_string(), "c5");
        assert_eq!(hand.cards[3].to_string(), "c4");
        assert_eq!(hand.cards[4].to_string(), "c3");
    }
    
    #[test]
    fn hand_contains_test() {
        let hand = Hand::new(vec![31, 22, 1, 2, 51, 3]);
        assert!(hand.contains(&Card::new(31)));
        assert!(hand.contains(&Card::new(22)));
        assert!(hand.contains(&Card::new(1)));
        assert!(hand.contains(&Card::new(2)));
        assert!(hand.contains(&Card::new(51)));
        assert!(hand.contains(&Card::new(3)));
        assert!(!hand.contains(&Card::new(30)));
    }
}
