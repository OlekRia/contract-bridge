use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CardValue {
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

impl Display for CardValue {
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

impl CardValue {
    pub fn points_count(self) -> u8 {
        match self {
            CardValue::Ace => 4,
            CardValue::King => 3,
            CardValue::Queen => 2,
            CardValue::Jack => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(CardValue::Two.to_string(), "2");
        assert_eq!(CardValue::Three.to_string(), "3");
        assert_eq!(CardValue::Four.to_string(), "4");
        assert_eq!(CardValue::Five.to_string(), "5");
        assert_eq!(CardValue::Six.to_string(), "6");
        assert_eq!(CardValue::Seven.to_string(), "7");
        assert_eq!(CardValue::Eight.to_string(), "8");
        assert_eq!(CardValue::Nine.to_string(), "9");
        assert_eq!(CardValue::Ten.to_string(), "T");
        assert_eq!(CardValue::Jack.to_string(), "J");
        assert_eq!(CardValue::Queen.to_string(), "Q");
        assert_eq!(CardValue::King.to_string(), "K");
        assert_eq!(CardValue::Ace.to_string(), "A");
    }

    #[test]
    fn points_count_test() {
        assert_eq!(CardValue::Ace.points_count(), 4);
        assert_eq!(CardValue::King.points_count(), 3);
        assert_eq!(CardValue::Queen.points_count(), 2);
        assert_eq!(CardValue::Jack.points_count(), 1);
        assert_eq!(CardValue::Ten.points_count(), 0);
        assert_eq!(CardValue::Nine.points_count(), 0);
        assert_eq!(CardValue::Eight.points_count(), 0);
        assert_eq!(CardValue::Seven.points_count(), 0);
        assert_eq!(CardValue::Six.points_count(), 0);
        assert_eq!(CardValue::Five.points_count(), 0);
        assert_eq!(CardValue::Four.points_count(), 0);
        assert_eq!(CardValue::Three.points_count(), 0);
        assert_eq!(CardValue::Two.points_count(), 0);
    }
}
