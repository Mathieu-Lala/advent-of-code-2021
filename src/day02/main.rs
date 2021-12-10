use std::str::FromStr;

enum Operation {
    Horizontal(u64),
    Vertical(i64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["up", value] => Ok(Operation::Vertical(value.parse::<i64>().unwrap() * -1)),
            ["down", value] => Ok(Operation::Vertical(value.parse::<i64>().unwrap())),
            ["forward", value] => Ok(Operation::Horizontal(value.parse::<u64>().unwrap())),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("day02");

    let (x, y, _aim) = include_str!("input.txt")
        .lines()
        .map(Operation::from_str)
        .map(Result::unwrap)
        .fold((0, 0, 0), |(x, y, aim), i| match i {
            Operation::Horizontal(xx) => (x + xx, y + aim * xx as i64, aim),
            Operation::Vertical(yy) => (x, y, aim + yy),
        });

    println!("x: {} * y: {} = {}", x, y, x as i64 * y)
}
