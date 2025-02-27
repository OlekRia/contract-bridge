pub mod hand;
pub mod trick;

use crate::deal::hand::card::Card;
use crate::game::Side;
use hand::Hand;
use hand::card::suits::Suits;
use std::collections::HashMap;
use trick::Trick;

/// A struct representing a deal of cards in the game.
/// * `dealer`: is the position of the dealer, who deals card, he makes bidding first
/// * `player`: is the position of the player, who wins a bidding round
/// * `zone`: a score depends on the zone, where the sides stay
/// * `trump`: STATE: is the suit of the trump card and determined by the bidding round
/// * `player`: STATE: is the position of the player, who wins a bidding round
pub struct Deal {
    pub hands: HashMap<Side, Hand>,
    pub dealer: Side,
    pub volnurable: (bool, bool),
    pub trump: Option<Suits>,
    pub declarer: Option<Side>,

    // Each player can have multiple tricks in a deal
    tricks: Vec<Trick>,
}

impl Deal {
    /// Creates a new `Deal` instance.
    ///
    /// # Arguments
    ///
    /// * `north`, `east`, `south`, `west`: The hands of the players in each position.
    /// * `dealer`: The position of the dealer (a guy who is dealing cards in the game).
    ///
    /// # Returns
    ///
    /// A `Deal` instance with empty `player` and `hands` set to the given hands.
    pub fn new(
        north: Hand,
        east: Hand,
        south: Hand,
        west: Hand,
        dealer: Side,
        volnurable: (bool, bool),
    ) -> Self {
        let hands = HashMap::from([
            (Side::North, north),
            (Side::East, east),
            (Side::South, south),
            (Side::West, west),
        ]);
        Self {
            hands,
            dealer,
            declarer: None,
            trump: None,
            volnurable,
            tricks: Vec::with_capacity(13),
        }
    }
}

impl Deal {
    pub fn encode(&self) -> Vec<u8> {
        let mut masks: [u64; 4] = [0; 4];

        // Convert each hand into a 52-bit bitmask
        for (side, hand) in &self.hands {
            let index = match side {
                Side::North => 0,
                Side::East => 1,
                Side::South => 2,
                Side::West => 3,
            };

            for card in &hand.cards {
                masks[index] |= 1 << card.id; // Assuming `Card::to_index()` returns 0-51
            }
        }

        let mut result = vec![0u8; 27];

        for (i, &mask) in masks.iter().enumerate() {
            let bytes = mask.to_le_bytes();
            result[i * 7..i * 7 + 7].copy_from_slice(&bytes[..7]);
        }

        // Encode trump suit (3 bits)
        let trump_value = match self.trump {
            None => 0,
            Some(Suits::Clubs) => 1,
            Some(Suits::Diamonds) => 2,
            Some(Suits::Hearts) => 3,
            Some(Suits::Spades) => 4,
        };

        // Encode dealer (2 bits)
        let dealer_value = match self.dealer {
            Side::North => 0,
            Side::East => 1,
            Side::South => 2,
            Side::West => 3,
        };

        // Encode declarer (3 bits, or 7 for None)
        let declarer_value = match self.declarer {
            None => 7,
            Some(Side::North) => 0,
            Some(Side::East) => 1,
            Some(Side::South) => 2,
            Some(Side::West) => 3,
        };

        // Encode zone (2 bits)
        let zone_value = ((self.volnurable.0 as u8) << 1) | (self.volnurable.1 as u8);

        // Store all values in the last byte
        result[26] = (trump_value << 5) | (dealer_value << 3) | declarer_value | (zone_value << 7);

        // Encode tricks
        for trick in &self.tricks {
            let mut trick_bytes = [0u8; 4];

            for (side, card) in &trick.side_cards {
                let side_index = match side {
                    Side::North => 0,
                    Side::East => 1,
                    Side::South => 2,
                    Side::West => 3,
                };

                if let Some(c) = card {
                    trick_bytes[side_index] = c.id;
                } else {
                    trick_bytes[side_index] = 63; // 63 = empty card placeholder
                }
            }

            let winner_value = match trick.winner {
                None => 3, // No winner (trick incomplete)
                Some(Side::North) => 0,
                Some(Side::East) => 1,
                Some(Side::South) => 2,
                Some(Side::West) => 3,
            };

            // Store winner in the first byte (2 bits at the top)
            trick_bytes[0] |= winner_value << 6;

            result.extend_from_slice(&trick_bytes);
        }

        result
    }

    pub fn decode(data: &[u8]) -> Self {
        let mut masks: [u64; 4] = [0; 4];

        // Extract 52-bit masks from 7-byte segments
        for i in 0..4 {
            let mut bytes = [0u8; 8]; // 8-byte buffer for conversion
            bytes[..7].copy_from_slice(&data[i * 7..i * 7 + 7]); // Copy only 7 bytes
            masks[i] = u64::from_le_bytes(bytes);
        }

        // Convert masks back into hands
        let mut hands = HashMap::new();
        for (i, &mask) in masks.iter().enumerate() {
            let mut cards = vec![];
            for card_id in 0..52 {
                if (mask & (1 << card_id)) != 0 {
                    cards.push(Card::new(card_id as u8)); // Convert back to `Card`
                }
            }

            let side = match i {
                0 => Side::North,
                1 => Side::East,
                2 => Side::South,
                _ => Side::West,
            };
            hands.insert(side, Hand { cards });
        }

        // Decode metadata from byte 26
        let last_byte = data[26];

        let trump = match (last_byte >> 5) & 0b111 {
            0 => None,
            1 => Some(Suits::Clubs),
            2 => Some(Suits::Diamonds),
            3 => Some(Suits::Hearts),
            4 => Some(Suits::Spades),
            _ => None, // Invalid data should default to None
        };

        let dealer = match (last_byte >> 3) & 0b11 {
            0 => Side::North,
            1 => Side::East,
            2 => Side::South,
            _ => Side::West,
        };

        let declarer = match last_byte & 0b111 {
            7 => None,
            0 => Some(Side::North),
            1 => Some(Side::East),
            2 => Some(Side::South),
            _ => Some(Side::West),
        };

        let zone = (((last_byte >> 7) & 0b1) != 0, ((last_byte >> 6) & 0b1) != 0);

        // Decode tricks
        let mut tricks = vec![];
        let mut index = 27;
        while index + 4 <= data.len() {
            let trick_data = &data[index..index + 4];

            let winner = match (trick_data[0] >> 6) & 0b11 {
                3 => None,
                0 => Some(Side::North),
                1 => Some(Side::East),
                2 => Some(Side::South),
                _ => Some(Side::West),
            };

            let mut side_cards = HashMap::new();
            for (i, &card_id) in trick_data.iter().enumerate() {
                if i == 0 {
                    side_cards.insert(
                        Side::North,
                        if card_id == 63 {
                            None
                        } else {
                            Some(Card::new(card_id))
                        },
                    );
                } else if i == 1 {
                    side_cards.insert(
                        Side::East,
                        if card_id == 63 {
                            None
                        } else {
                            Some(Card::new(card_id))
                        },
                    );
                } else if i == 2 {
                    side_cards.insert(
                        Side::South,
                        if card_id == 63 {
                            None
                        } else {
                            Some(Card::new(card_id))
                        },
                    );
                } else {
                    side_cards.insert(
                        Side::West,
                        if card_id == 63 {
                            None
                        } else {
                            Some(Card::new(card_id))
                        },
                    );
                }
            }

            tricks.push(Trick { side_cards, winner });

            index += 4;
        }

        Deal {
            hands,
            dealer,
            trump,
            declarer,
            volnurable: zone,
            tricks,
        }
    }
}
