use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    North,
    East,
    South,
    West
}

impl Display for Side{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn side_format() {
        assert_eq!(Side::North.to_string(), "N");
        assert_eq!(Side::East.to_string(), "E");
        assert_eq!(Side::South.to_string(), "S");
        assert_eq!(Side::West.to_string(), "W");
    }
}