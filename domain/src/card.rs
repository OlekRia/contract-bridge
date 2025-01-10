use core::panic;

use crate::{CardValue, Suits};

/// Represents a playing card in the game.
///
/// This struct holds the `suit` and `value` of the card,
/// which are essential to identifying the card in the deck.
pub struct Card {
    pub suit: Suits,
    pub value: CardValue,
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
    /// # Example
    /// ```
    /// use domain::Card;
    /// use domain::{CardValue, Suits};
    ///
    /// let card = Card::new(51);
    /// assert_eq!(card.suit, Suits::Spades);
    /// assert_eq!(card.value, CardValue::Ace);
    /// ```
    ///
    pub fn new(card_num: u8) -> Self {
        if card_num > 51 {
            panic!("card number must be less than 51")
        }

        let suit = match card_num / 13 {
            0 => Suits::Clubs,
            1 => Suits::Diamonds,
            2 => Suits::Hearts,
            3 => Suits::Spades,
            _ => unreachable!(), // Since card_num is guaranteed to be <= 51, this is safe
        };

        let value = match card_num % 13 {
            0 => CardValue::Two,
            1 => CardValue::Three,
            2 => CardValue::Four,
            3 => CardValue::Five,
            4 => CardValue::Six,
            5 => CardValue::Seven,
            6 => CardValue::Eight,
            7 => CardValue::Nine,
            8 => CardValue::Ten,
            9 => CardValue::Jack,
            10 => CardValue::Queen,
            11 => CardValue::King,
            12 => CardValue::Ace,
            _ => unreachable!(), // Should never be reached
        };

        Self { suit, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_numbers(num: u8, suit: Suits, value: CardValue) {
        let card = Card::new(num);
        assert_eq!(card.suit, suit);
        assert_eq!(card.value, value);
    }

    #[test]
    fn card_new() {
        test_numbers(0, Suits::Clubs, CardValue::Two);
        test_numbers(1, Suits::Clubs, CardValue::Three);
        test_numbers(2, Suits::Clubs, CardValue::Four);
        test_numbers(3, Suits::Clubs, CardValue::Five);
        test_numbers(4, Suits::Clubs, CardValue::Six);
        test_numbers(5, Suits::Clubs, CardValue::Seven);
        test_numbers(6, Suits::Clubs, CardValue::Eight);
        test_numbers(7, Suits::Clubs, CardValue::Nine);
        test_numbers(8, Suits::Clubs, CardValue::Ten);
        test_numbers(9, Suits::Clubs, CardValue::Jack);
        test_numbers(10, Suits::Clubs, CardValue::Queen);
        test_numbers(11, Suits::Clubs, CardValue::King);
        test_numbers(12, Suits::Clubs, CardValue::Ace);

        test_numbers(13, Suits::Diamonds, CardValue::Two);
        test_numbers(14, Suits::Diamonds, CardValue::Three);
        test_numbers(15, Suits::Diamonds, CardValue::Four);
        test_numbers(16, Suits::Diamonds, CardValue::Five);
        test_numbers(17, Suits::Diamonds, CardValue::Six);
        test_numbers(18, Suits::Diamonds, CardValue::Seven);
        test_numbers(19, Suits::Diamonds, CardValue::Eight);
        test_numbers(20, Suits::Diamonds, CardValue::Nine);
        test_numbers(21, Suits::Diamonds, CardValue::Ten);
        test_numbers(22, Suits::Diamonds, CardValue::Jack);
        test_numbers(23, Suits::Diamonds, CardValue::Queen);
        test_numbers(24, Suits::Diamonds, CardValue::King);
        test_numbers(25, Suits::Diamonds, CardValue::Ace);

        test_numbers(26, Suits::Hearts, CardValue::Two);
        test_numbers(27, Suits::Hearts, CardValue::Three);
        test_numbers(28, Suits::Hearts, CardValue::Four);
        test_numbers(29, Suits::Hearts, CardValue::Five);
        test_numbers(30, Suits::Hearts, CardValue::Six);
        test_numbers(31, Suits::Hearts, CardValue::Seven);
        test_numbers(32, Suits::Hearts, CardValue::Eight);
        test_numbers(33, Suits::Hearts, CardValue::Nine);
        test_numbers(34, Suits::Hearts, CardValue::Ten);
        test_numbers(35, Suits::Hearts, CardValue::Jack);
        test_numbers(36, Suits::Hearts, CardValue::Queen);
        test_numbers(37, Suits::Hearts, CardValue::King);
        test_numbers(38, Suits::Hearts, CardValue::Ace);

        test_numbers(39, Suits::Spades, CardValue::Two);
        test_numbers(40, Suits::Spades, CardValue::Three);
        test_numbers(41, Suits::Spades, CardValue::Four);
        test_numbers(42, Suits::Spades, CardValue::Five);
        test_numbers(43, Suits::Spades, CardValue::Six);
        test_numbers(44, Suits::Spades, CardValue::Seven);
        test_numbers(45, Suits::Spades, CardValue::Eight);
        test_numbers(46, Suits::Spades, CardValue::Nine);
        test_numbers(47, Suits::Spades, CardValue::Ten);
        test_numbers(48, Suits::Spades, CardValue::Jack);
        test_numbers(49, Suits::Spades, CardValue::Queen);
        test_numbers(50, Suits::Spades, CardValue::King);
        test_numbers(51, Suits::Spades, CardValue::Ace);
    }

    #[test]
    #[should_panic(expected = "card number must be less than 51")]
    fn card_new_panics() {
        Card::new(52);
    }
}
