use std::fmt::Display;

/// The `Rank` enum represents the ten ranks of playing cards, each with an associated `u8` value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MajorMinor {
    Major,
    Minor,
}

/// The `Suits` enum represents the four playing card suits, each with an associated `u8` value.
///
/// # Variants
/// - `Clubs = 0`
/// - `Diamonds = 1`
/// - `Hearts = 2`
/// - `Spades = 3`
///
/// # Example
///
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suits {
    Clubs = 0u8,
    Diamonds = 1u8,
    Hearts = 2u8,
    Spades = 3u8,
}

impl Display for Suits {
    /// Formats the suit as a single character.
    ///
    /// # Examples
    ///
    /// - `Suits::Clubs` is formatted as `"c"`.
    /// - `Suits::Diamonds` is formatted as `"d"`.
    /// - `Suits::Hearts` is formatted as `"h"`.
    /// - `Suits::Spades` is formatted as `"s"`.
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suits::Clubs => write!(f, "c"),
            Suits::Diamonds => write!(f, "d"),
            Suits::Hearts => write!(f, "h"),
            Suits::Spades => write!(f, "s"),
        }
    }
}

impl Suits {
    /// Returns `MajorMinor::Major` if the suit is either `Spades` or `Hearts`, and `MajorMinor::Minor`
    /// if the suit is either `Clubs` or `Diamonds`.
    pub fn major_minor(&self) -> MajorMinor {
        match self {
            Suits::Spades | Suits::Hearts => MajorMinor::Major,
            Suits::Clubs | Suits::Diamonds => MajorMinor::Minor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Suits::Clubs.to_string(), "c");
        assert_eq!(Suits::Diamonds.to_string(), "d");
        assert_eq!(Suits::Hearts.to_string(), "h");
        assert_eq!(Suits::Spades.to_string(), "s");
    }

    #[test]
    fn test_suit_values() {
        assert_eq!(Suits::Clubs as u8, 0);
        assert_eq!(Suits::Diamonds as u8, 1);
        assert_eq!(Suits::Hearts as u8, 2);
        assert_eq!(Suits::Spades as u8, 3);
    }

    #[test]
    fn compare_suits() {
        assert_eq!(Suits::Clubs, Suits::Clubs);
        assert_eq!(Suits::Diamonds, Suits::Diamonds);
        assert_eq!(Suits::Hearts, Suits::Hearts);
        assert_eq!(Suits::Spades, Suits::Spades);

        assert!(Suits::Spades > Suits::Hearts);
        assert!(Suits::Hearts > Suits::Diamonds);
        assert!(Suits::Diamonds > Suits::Clubs);
    }

    #[test]
    fn test_major_minor() {
        assert_eq!(Suits::Spades.major_minor(), MajorMinor::Major);
        assert_eq!(Suits::Hearts.major_minor(), MajorMinor::Major);
        assert_eq!(Suits::Clubs.major_minor(), MajorMinor::Minor);
        assert_eq!(Suits::Diamonds.major_minor(), MajorMinor::Minor);
    }
}
