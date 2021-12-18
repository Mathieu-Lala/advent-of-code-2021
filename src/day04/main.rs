#[derive(Debug, Copy, Clone)]
enum Bingo {
    Set(i64),
    NotSet(i64),
}

#[derive(Debug)]
struct Grid {
    data: [[Bingo; 5]; 5],
    won: bool,
}

impl Grid {
    fn try_set(&mut self, v: i64) {
        let mut changed = false;
        self.data.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|i| match i {
                Bingo::NotSet(k) if *k == v => {
                    *i = Bingo::Set(v);
                    changed = true;
                }
                _ => (),
            })
        });
        self.won = changed && self.check_won();
    }

    fn check_won(&self) -> bool {
        self.data.iter().any(|line| {
            line.iter().all(|i| match i {
                Bingo::Set(_) => true,
                Bingo::NotSet(_) => false,
            })
        }) || (0..self.data[0].len()).fold(false, |prev, x| {
            prev || self.data.iter().map(|i| i[x]).all(|i| match i {
                Bingo::Set(_) => true,
                Bingo::NotSet(_) => false,
            })
        })
    }

    fn score(&self) -> i64 {
        self.data.iter().fold(0, |sum, line| {
            line.iter().fold(sum, |sum, i| {
                sum + match i {
                    Bingo::NotSet(i) => *i,
                    Bingo::Set(_) => 0,
                }
            })
        })
    }
}

fn main() {
    println!("day04");

    let mut input = include_str!("input.txt").split("\n\n");
    let random_numbers = input
        .next()
        .unwrap()
        .split(",")
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let mut grids = input
        .into_iter()
        .map(|raw| {
            raw.lines().enumerate().fold(
                Grid {
                    data: [[Bingo::NotSet(0); 5]; 5],
                    won: false,
                },
                |mut grid, (i, v)| {
                    grid.data[i] = v
                        .split_whitespace()
                        .map(str::parse::<i64>)
                        .filter_map(Result::ok)
                        .map(Bingo::NotSet)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    grid
                },
            )
        })
        .collect::<Vec<_>>();

    let mut won = Vec::new();

    for i in &random_numbers {
        for grid in grids.iter_mut().filter(|i| !i.won) {
            grid.try_set(*i);
            if grid.won {
                won.push(grid.score() * i);
            }
        }
    }

    println!("{:?}", won);
}
