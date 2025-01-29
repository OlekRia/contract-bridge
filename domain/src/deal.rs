pub mod hand;
pub mod trick;

use crate::game::Side;
use hand::card::suits::Suits;
use hand::Hand;
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
    pub zone: (bool, bool),

    pub trump: Option<Suits>,
    pub player: Option<Side>,

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
        zone: (bool, bool),
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
            player: None,
            trump: None,
            zone,
            tricks: Vec::with_capacity(13),
        }
    }
}
