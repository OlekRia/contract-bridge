pub mod rank;
pub mod suits;

use rank::Rank;
use std::fmt::Display;
use suits::Suits;

/// Represents a playing card in the game.
///
/// This struct holds the `suit` and `value` of the card,
/// which are essential to identifying the card in the deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: u8,
    pub suit: Suits,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.suit.cmp(&other.suit) {
            std::cmp::Ordering::Equal => self.rank.cmp(&other.rank),
            other => other,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Card {
    /// Creates a Card from a u8 representing the card.
    ///
    /// The numbering is as follows:
    /// Clubs: 0-12
    /// Diamonds: 13-25
    /// Hearts: 26-38
    /// Spades: 39-51
    ///
    /// # Panics
    /// Panics if the card number is greater than 51.
    ///
    pub fn new(card_num: u8 ) -> Self {
        if card_num > 51 {
            panic!("card number must be less than 52")
        }

        let suit = match card_num / 13 {
            0 => Suits::Clubs,
            1 => Suits::Diamonds,
            2 => Suits::Hearts,
            3 => Suits::Spades,
            _ => unreachable!(), // Since card_num is guaranteed to be <= 51, this is safe
        };

        let value = match card_num % 13 {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => unreachable!(), // Should never be reached
        };

        Self {
            id: card_num,
            suit,
            rank: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_numbers(num: u8, suit: Suits, value: Rank) {
        let card = Card::new(num);
        assert_eq!(card.suit, suit);
        assert_eq!(card.rank, value);
    }

    #[test]
    fn card_new() {
        test_numbers(0, Suits::Clubs, Rank::Two);
        test_numbers(1, Suits::Clubs, Rank::Three);
        test_numbers(2, Suits::Clubs, Rank::Four);
        test_numbers(3, Suits::Clubs, Rank::Five);
        test_numbers(4, Suits::Clubs, Rank::Six);
        test_numbers(5, Suits::Clubs, Rank::Seven);
        test_numbers(6, Suits::Clubs, Rank::Eight);
        test_numbers(7, Suits::Clubs, Rank::Nine);
        test_numbers(8, Suits::Clubs, Rank::Ten);
        test_numbers(9, Suits::Clubs, Rank::Jack);
        test_numbers(10, Suits::Clubs, Rank::Queen);
        test_numbers(11, Suits::Clubs, Rank::King);
        test_numbers(12, Suits::Clubs, Rank::Ace);

        test_numbers(13, Suits::Diamonds, Rank::Two);
        test_numbers(14, Suits::Diamonds, Rank::Three);
        test_numbers(15, Suits::Diamonds, Rank::Four);
        test_numbers(16, Suits::Diamonds, Rank::Five);
        test_numbers(17, Suits::Diamonds, Rank::Six);
        test_numbers(18, Suits::Diamonds, Rank::Seven);
        test_numbers(19, Suits::Diamonds, Rank::Eight);
        test_numbers(20, Suits::Diamonds, Rank::Nine);
        test_numbers(21, Suits::Diamonds, Rank::Ten);
        test_numbers(22, Suits::Diamonds, Rank::Jack);
        test_numbers(23, Suits::Diamonds, Rank::Queen);
        test_numbers(24, Suits::Diamonds, Rank::King);
        test_numbers(25, Suits::Diamonds, Rank::Ace);

        test_numbers(26, Suits::Hearts, Rank::Two);
        test_numbers(27, Suits::Hearts, Rank::Three);
        test_numbers(28, Suits::Hearts, Rank::Four);
        test_numbers(29, Suits::Hearts, Rank::Five);
        test_numbers(30, Suits::Hearts, Rank::Six);
        test_numbers(31, Suits::Hearts, Rank::Seven);
        test_numbers(32, Suits::Hearts, Rank::Eight);
        test_numbers(33, Suits::Hearts, Rank::Nine);
        test_numbers(34, Suits::Hearts, Rank::Ten);
        test_numbers(35, Suits::Hearts, Rank::Jack);
        test_numbers(36, Suits::Hearts, Rank::Queen);
        test_numbers(37, Suits::Hearts, Rank::King);
        test_numbers(38, Suits::Hearts, Rank::Ace);
        
        test_numbers(39, Suits::Spades, Rank::Two);
        test_numbers(40, Suits::Spades, Rank::Three);
        test_numbers(41, Suits::Spades, Rank::Four);
        test_numbers(42, Suits::Spades, Rank::Five);
        test_numbers(43, Suits::Spades, Rank::Six);
        test_numbers(44, Suits::Spades, Rank::Seven);
        test_numbers(45, Suits::Spades, Rank::Eight);
        test_numbers(46, Suits::Spades, Rank::Nine);
        test_numbers(47, Suits::Spades, Rank::Ten);
        test_numbers(48, Suits::Spades, Rank::Jack);
        test_numbers(49, Suits::Spades, Rank::Queen);
        test_numbers(50, Suits::Spades, Rank::King);
        test_numbers(51, Suits::Spades, Rank::Ace);
    }

    #[test]
    #[should_panic(expected = "card number must be less than 52")]
    fn card_creation_panics_on_invalid_input() {
        Card::new(52);
    }

    #[test]
    fn card_display_test() {
        let card = Card::new(51);
        assert_eq!(card.to_string(), "sA");
    }

    

    #[test]
    fn compare_cards() {
        let card1 = Card::new(26);
        let card2 = Card::new(26);
        assert_eq!(card1, card2);
        
        let card3 = Card::new(51);
        assert_ne!(card1, card3);
        assert!(card1 < card3);
        
        
        let card2c = Card::new(0);
        let card3c = Card::new(1);
        assert!(card2c < card3c);
        assert!(card3c > card2c);
        
        let card2s = Card::new(51);
        assert!(card2s > card3c);
    }
}
