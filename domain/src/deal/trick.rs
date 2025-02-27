use super::hand::card::Card;
use crate::game::Side;
use std::collections::HashMap;

/// We put all the tricks here. Up to 4 cards and determin a winner
pub struct Trick {
    pub side_cards: HashMap<Side, Option<Card>>,
    pub winner: Option<Side>,
}

impl Trick {
    /// Creates a new `Trick` instance.
    /// # Example:
    /// let trick = Trick::new();
    pub fn new() -> Self {
        let side_cards = Side::iter().map(|side| (side, None)).collect();
        Self {
            side_cards,
            winner: None,
        }
    }

    pub fn set_winner(&mut self, winner: Side) {
        self.winner = Some(winner);
    }

    pub fn set_card(&mut self, side: Side, card: Card) {
        self.side_cards.insert(side, Some(card));
    }
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let trick = Trick::new();
        assert_eq!(trick.winner, None);
        assert_eq!(trick.side_cards.len(), 4);
        assert_eq!(trick.side_cards[&Side::North], None);
        assert_eq!(trick.side_cards[&Side::East], None);
        assert_eq!(trick.side_cards[&Side::South], None);
        assert_eq!(trick.side_cards[&Side::West], None);
    }

    #[test]
    fn set_winner_test() {
        let mut trick = Trick::new();
        assert_eq!(trick.winner, None);
        trick.set_winner(Side::North);
        assert_eq!(trick.winner, Some(Side::North));
    }

    #[test]
    fn set_card_test() {
        let mut trick = Trick::new();
        trick.set_card(Side::North, Card::new(51));
        assert_eq!(
            trick.side_cards[&Side::North].as_ref().unwrap().to_string(),
            "sA"
        );
    }
}
