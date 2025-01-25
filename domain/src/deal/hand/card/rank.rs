use std::fmt::Display;

 
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    Two = 0u8,
    Three = 1u8,
    Four = 2u8,
    Five = 3u8,
    Six = 4u8,
    Seven = 5u8,
    Eight = 6u8,
    Nine = 7u8,
    Ten = 8u8,
    Jack = 9u8,
    Queen = 10u8,
    King = 11u8,
    Ace = 12u8,
}

impl Display for Rank {
    /// Format the rank as a single character.
    ///
    /// # Examples
    ///
    /// - `Rank::Two` is formatted as `"2"`.
    /// - `Rank::Jack` is formatted as `"J"`.
    /// - `Rank::Ace` is formatted as `"A"`.
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}

impl Rank {
    /// Returns the number of points that this rank is worth in a cribbage hand.
    ///
    /// # Examples
    ///
    /// - `Rank::Ace` is worth 4 points.
    /// - `Rank::King` is worth 3 points.
    /// - `Rank::Queen` is worth 2 points.
    /// - `Rank::Jack` is worth 1 point.
    /// - All other ranks are worth 0 points.
    ///
    pub fn points_count(self) -> u8 {
        match self {
            Rank::Ace => 4,
            Rank::King => 3,
            Rank::Queen => 2,
            Rank::Jack => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Rank::Two.to_string(), "2");
        assert_eq!(Rank::Three.to_string(), "3");
        assert_eq!(Rank::Four.to_string(), "4");
        assert_eq!(Rank::Five.to_string(), "5");
        assert_eq!(Rank::Six.to_string(), "6");
        assert_eq!(Rank::Seven.to_string(), "7");
        assert_eq!(Rank::Eight.to_string(), "8");
        assert_eq!(Rank::Nine.to_string(), "9");
        assert_eq!(Rank::Ten.to_string(), "T");
        assert_eq!(Rank::Jack.to_string(), "J");
        assert_eq!(Rank::Queen.to_string(), "Q");
        assert_eq!(Rank::King.to_string(), "K");
        assert_eq!(Rank::Ace.to_string(), "A");
    }

    #[test]
    fn points_count_test() {
        assert_eq!(Rank::Ace.points_count(), 4);
        assert_eq!(Rank::King.points_count(), 3);
        assert_eq!(Rank::Queen.points_count(), 2);
        assert_eq!(Rank::Jack.points_count(), 1);
        assert_eq!(Rank::Ten.points_count(), 0);
        assert_eq!(Rank::Nine.points_count(), 0);
        assert_eq!(Rank::Eight.points_count(), 0);
        assert_eq!(Rank::Seven.points_count(), 0);
        assert_eq!(Rank::Six.points_count(), 0);
        assert_eq!(Rank::Five.points_count(), 0);
        assert_eq!(Rank::Four.points_count(), 0);
        assert_eq!(Rank::Three.points_count(), 0);
        assert_eq!(Rank::Two.points_count(), 0);
    }
    
    #[test]
    fn compare_ranks() {
        assert_eq!(Rank::Two, Rank::Two);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Ten < Rank::Jack);
        
    }
}
