use std::fmt::Display;

use super::direction::Direction;

#[derive(Debug, PartialEq, Clone)]
pub struct Map<T>
where
    T: Display + PartialEq + Clone,
{
    pub tiles: Vec<T>,
    pub line_length: usize,
}

impl<T: Display + PartialEq + Clone> Map<T> {
    pub fn get_rows(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        self.tiles.chunks(self.line_length).map(|x| x.to_vec())
    }

    pub fn replace_rows(&mut self, rows: Vec<Vec<T>>) {
        self.line_length = rows[0].len();
        self.tiles = rows.into_iter().flatten().collect();
    }

    pub fn get_columns(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.line_length).map(move |i| {
            let mut column = Vec::new();
            for j in 0..self.get_rows_count() {
                column.push(self.tiles[j * self.line_length + i].clone());
            }
            column
        })
    }

    pub fn get_rows_count(&self) -> usize {
        self.tiles.len() / self.line_length
    }

    pub fn replace_columns(&mut self, columns: Vec<Vec<T>>) {
        let mut tiles = Vec::new();
        for i in 0..columns[0].len() {
            for column in &columns {
                tiles.push(column[i].clone());
            }
        }
        self.tiles = tiles;
        self.line_length = columns.len();
    }

    pub fn to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.line_length, index / self.line_length)
    }

    pub fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.line_length + x
    }

    pub fn distance(&self, a: usize, b: usize) -> usize {
        let a_x = a % self.line_length;
        let a_y = a / self.line_length;
        let b_x = b % self.line_length;
        let b_y = b / self.line_length;
        ((a_x as isize - b_x as isize).abs() + (a_y as isize - b_y as isize).abs()) as usize
    }

    pub fn distance_to_border(&self, index: usize, direction: &Direction) -> usize {
        let (x, y) = self.to_xy(index);
        let y_size = self.tiles.len() / self.line_length;
        match direction {
            Direction::North => y,
            Direction::South => y_size - y - 1,
            Direction::East => self.line_length - x - 1,
            Direction::West => x,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    pub fn move_from(&self, from: usize, direction: &Direction) -> Option<usize> {
        direction.get_index(self.tiles.len(), self.line_length, from)
    }
}

impl<T: Display + PartialEq + Clone> Display for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .tiles
            .chunks(self.line_length)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|tile| tile.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl<T> From<&str> for Map<T>
where
    T: Display + PartialEq + Clone + From<char>,
{
    fn from(s: &str) -> Self {
        let mut line_length = 0;
        Map {
            tiles: s
                .lines()
                .flat_map(|line| {
                    line_length = line.len();
                    line.trim().chars().map(|c| c.into()).collect::<Vec<T>>()
                })
                .collect(),
            line_length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_xy() {
        let map = Map {
            tiles: vec!['a', 'b', 'c', 'd', 'e', 'f'],
            line_length: 3,
        };

        assert_eq!(map.to_xy(0), (0, 0));
        assert_eq!(map.to_xy(1), (1, 0));
        assert_eq!(map.to_xy(2), (2, 0));
        assert_eq!(map.to_xy(3), (0, 1));
        assert_eq!(map.to_xy(4), (1, 1));
        assert_eq!(map.to_xy(5), (2, 1));
    }

    #[test]
    fn distance_to_border() {
        let map = Map {
            tiles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            line_length: 3,
        };

        assert_eq!(map.distance_to_border(0, &Direction::North), 0);
        assert_eq!(map.distance_to_border(0, &Direction::South), 2);
        assert_eq!(map.distance_to_border(0, &Direction::West), 0);
        assert_eq!(map.distance_to_border(0, &Direction::East), 2);
        assert_eq!(map.distance_to_border(1, &Direction::North), 0);
        assert_eq!(map.distance_to_border(1, &Direction::South), 2);
        assert_eq!(map.distance_to_border(1, &Direction::West), 1);
        assert_eq!(map.distance_to_border(1, &Direction::East), 1);
        assert_eq!(map.distance_to_border(2, &Direction::North), 0);
        assert_eq!(map.distance_to_border(2, &Direction::South), 2);
        assert_eq!(map.distance_to_border(2, &Direction::West), 2);
        assert_eq!(map.distance_to_border(2, &Direction::East), 0);
        assert_eq!(map.distance_to_border(3, &Direction::North), 1);
        assert_eq!(map.distance_to_border(3, &Direction::South), 1);
        assert_eq!(map.distance_to_border(3, &Direction::West), 0);
        assert_eq!(map.distance_to_border(3, &Direction::East), 2);
        assert_eq!(map.distance_to_border(4, &Direction::North), 1);
        assert_eq!(map.distance_to_border(4, &Direction::South), 1);
        assert_eq!(map.distance_to_border(4, &Direction::West), 1);
        assert_eq!(map.distance_to_border(4, &Direction::East), 1);
        assert_eq!(map.distance_to_border(5, &Direction::North), 1);
        assert_eq!(map.distance_to_border(5, &Direction::South), 1);
        assert_eq!(map.distance_to_border(5, &Direction::West), 2);
        assert_eq!(map.distance_to_border(5, &Direction::East), 0);
        assert_eq!(map.distance_to_border(6, &Direction::North), 2);
        assert_eq!(map.distance_to_border(6, &Direction::South), 0);
        assert_eq!(map.distance_to_border(6, &Direction::West), 0);
        assert_eq!(map.distance_to_border(6, &Direction::East), 2);
        assert_eq!(map.distance_to_border(7, &Direction::North), 2);
        assert_eq!(map.distance_to_border(7, &Direction::South), 0);
        assert_eq!(map.distance_to_border(7, &Direction::West), 1);
        assert_eq!(map.distance_to_border(7, &Direction::East), 1);
    }
}
