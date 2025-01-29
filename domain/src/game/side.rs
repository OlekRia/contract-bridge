use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Side {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Side {
    /// Returns the next position in clockwise order.
    pub fn next(&self) -> Self {
        match self {
            Side::North => Side::East,
            Side::East => Side::South,
            Side::South => Side::West,
            Side::West => Side::North,
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::North => write!(f, "N"),
            Side::East => write!(f, "E"),
            Side::South => write!(f, "S"),
            Side::West => write!(f, "W"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_position() {
        assert_eq!(Side::North.to_string(), "N");
        assert_eq!(Side::East.to_string(), "E");
        assert_eq!(Side::South.to_string(), "S");
        assert_eq!(Side::West.to_string(), "W");
    }

    #[test]
    fn next_test() {
        assert_eq!(Side::North.next(), Side::East);
        assert_eq!(Side::East.next(), Side::South);
        assert_eq!(Side::South.next(), Side::West);
        assert_eq!(Side::West.next(), Side::North);
    }
}
