use std::usize;

use crate::Runner;

#[derive(Debug)]
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

#[derive(Debug)]
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

fn render(lines: &[Line]) {
    let max_x = lines.iter().map(|l| l.start.x.max(l.end.x)).max().unwrap() + 1;
    let max_y = lines.iter().map(|l| l.start.y.max(l.end.y)).max().unwrap() + 1;
    let max_z = lines.iter().map(|l| l.start.z.max(l.end.z)).max().unwrap();
    let mut plains: Vec<Vec<usize>> = vec![vec![0; max_x * max_y]; max_z];
    for (i, line) in lines.iter().enumerate() {
        for z in line.start.z..=line.end.z {
            for y in line.start.y..=line.end.y {
                for x in line.start.x..=line.end.x {
                    plains[z - 1][y * max_x + x] = i + 1;
                }
            }
        }
    }
    for (i, p) in plains.iter().enumerate() {
        println!("z = {}", i);
        for y in 0..max_y {
            for x in 0..max_x {
                print!("{:3} ", p[y * max_x + x]);
            }
            println!();
        }
        println!();
    }
}

pub fn run(runner: &Runner) {
    match runner.part {
        crate::Part::One => todo!(),
        crate::Part::Two => todo!(),
    }
}

mod tests {
    use crate::twenty_second::{read, render};

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
        let input = read(INPUT.trim().split('\n').map(str::to_string));
        assert_eq!(input.len(), 7);
        render(&input);
    }
}
