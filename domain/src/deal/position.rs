use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Position {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Position {
    /// Returns the next position in clockwise order.
    pub fn next(&self) -> Self {
        match self {
            Position::North => Position::East,
            Position::East => Position::South,
            Position::South => Position::West,
            Position::West => Position::North,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::North => write!(f, "N"),
            Position::East => write!(f, "E"),
            Position::South => write!(f, "S"),
            Position::West => write!(f, "W"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_position() {
        assert_eq!(Position::North.to_string(), "N");
        assert_eq!(Position::East.to_string(), "E");
        assert_eq!(Position::South.to_string(), "S");
        assert_eq!(Position::West.to_string(), "W");
    }

    #[test]
    fn next_test() {
        assert_eq!(Position::North.next(), Position::East);
        assert_eq!(Position::East.next(), Position::South);
        assert_eq!(Position::South.next(), Position::West);
        assert_eq!(Position::West.next(), Position::North);
    }
}
