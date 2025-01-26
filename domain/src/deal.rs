pub mod hand;
pub mod position;

use std::collections::HashMap;
use hand::Hand;
use position::Position;

/// A struct representing a deal of cards in the game.
/// dealer is the position of the dealer, who deals card, he makes bidding first
/// player is the position of the player, who wins a bidding round
/// It's easy to calculate then who is LHO, RHO and Dummy
/// 
pub struct Deal {
    pub hands: HashMap<Position, Hand>,
    pub dealer: Position,
    pub player: Option<Position>
}

impl Deal {
    /// Creates a new `Deal` instance.
    ///
    /// # Arguments
    ///
    /// * `north`, `east`, `south`, `west`: The hands of the players in each position.
    /// * `dealer`: The position of the dealer.
    ///
    /// # Returns
    ///
    /// A `Deal` instance with empty `player` and `hands` set to the given hands.
    pub fn new(north: Hand, east: Hand, south: Hand, west: Hand, dealer: Position) -> Self {
        let hands: HashMap<Position, Hand> = HashMap::from([
            (Position::North, north), 
            (Position::East, east), 
            (Position::South, south), 
            (Position::West, west)
        ]);
        
        Self { hands, dealer, player: None }
    }
}
