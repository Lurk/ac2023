use std::usize;

use crate::Runner;

struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();
        Point { x, y, z }
    }
}

struct Line {
    start: Point,
    end: Point,
}

fn read(i: impl Iterator<Item = String>) -> Vec<Line> {
    i.map(|s| {
        let (start, end) = s.split_once('~').expect("input must have ~");
        let start = Point::from(start);
        let end = Point::from(end);
        Line { start, end }
    })
    .collect()
}

pub fn run(runner: &Runner) {
    match runner.part {
        crate::Part::One => todo!(),
        crate::Part::Two => todo!(),
    }
}

mod tests {
    const INPUT: &str = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_read() {
        let input = super::read(INPUT.trim().split('\n').map(str::to_string));
        assert_eq!(input.len(), 7);
    }
}
