use std::fmt::Display;

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

    pub fn get_columns(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.line_length).map(move |i| {
            self.tiles
                .iter()
                .enumerate()
                .filter(|(j, _)| j % self.line_length == i)
                .map(|(_, x)| x.clone())
                .collect()
        })
    }

    pub fn distance(&self, a: usize, b: usize) -> usize {
        let a_x = a % self.line_length;
        let a_y = a / self.line_length;
        let b_x = b % self.line_length;
        let b_y = b / self.line_length;
        ((a_x as isize - b_x as isize).abs() + (a_y as isize - b_y as isize).abs()) as usize
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
