#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn get_index(&self, total_len: usize, line_length: usize, index: usize) -> Option<usize> {
        match self {
            Direction::North => {
                if index >= line_length {
                    Some(index - line_length)
                } else {
                    None
                }
            }
            Direction::NorthEast => {
                if index >= line_length && index % line_length != line_length - 1 {
                    Some(index - line_length + 1)
                } else {
                    None
                }
            }
            Direction::East => {
                if index % line_length != line_length - 1 {
                    Some(index + 1)
                } else {
                    None
                }
            }
            Direction::SouthEast => {
                if index < total_len - line_length && index % line_length != line_length - 1 {
                    Some(index + line_length + 1)
                } else {
                    None
                }
            }
            Direction::South => {
                if index < total_len - line_length {
                    Some(index + line_length)
                } else {
                    None
                }
            }
            Direction::SouthWest => {
                if index < total_len - line_length && index % line_length != 0 {
                    Some(index + line_length - 1)
                } else {
                    None
                }
            }
            Direction::West => {
                if index % line_length != 0 {
                    Some(index - 1)
                } else {
                    None
                }
            }
            Direction::NorthWest => {
                if index >= line_length && index % line_length != 0 {
                    Some(index - line_length - 1)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Direction;

    #[test]
    fn direction_get_index() {
        assert_eq!(Direction::North.get_index(9, 3, 4), Some(1));
        assert_eq!(Direction::NorthEast.get_index(9, 3, 4), Some(2));
        assert_eq!(Direction::East.get_index(9, 3, 4), Some(5));
        assert_eq!(Direction::SouthEast.get_index(9, 3, 4), Some(8));
        assert_eq!(Direction::South.get_index(9, 3, 4), Some(7));
        assert_eq!(Direction::SouthWest.get_index(9, 3, 4), Some(6));
        assert_eq!(Direction::West.get_index(9, 3, 4), Some(3));
        assert_eq!(Direction::NorthWest.get_index(9, 3, 4), Some(0));
        assert_eq!(Direction::North.get_index(9, 3, 1), None);
    }
}
