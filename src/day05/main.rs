struct Map(Vec<Vec<usize>>);

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for i in line {
                match i {
                    0 => f.write_str(".")?,
                    i => f.write_fmt(format_args!("{}", i))?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn count_dangerous_area(&self, threshold: usize) -> usize {
        self.0.iter().fold(0_usize, |sum, line| {
            sum + line.iter().filter(|i| **i >= threshold).count()
        })
    }
}

fn main() {
    println!("day05");

    let map = include_str!("input.txt")
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            assert!(words.len() == 3, "invalid input");

            fn token_to_coordinate(word: &str) -> (usize, usize) {
                word.split_once(",")
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                    .unwrap()
            }

            (token_to_coordinate(words[0]), token_to_coordinate(words[2]))
        })
        .fold(Map { 0: vec![] }, |mut map, ((x1, y1), (x2, y2))| {
            let x_max = std::cmp::max(x1, x2);
            let y_max = std::cmp::max(y1, y2);

            for i in &mut map.0 {
                if i.len() < x_max + 1 {
                    i.resize(x_max + 1, 0);
                }
            }
            if map.0.len() < y_max + 1 {
                map.0.resize(y_max + 1, vec![0; x_max + 1]);
            }
            match (x1 == x2, y1 == y2) {
                (true, true) => {
                    map.0[y1][x1] += 1;
                    map
                }
                (true, false) => {
                    ((std::cmp::min(y1, y2))..y_max + 1).for_each(|y| {
                        map.0[y][x1] += 1;
                    });
                    map
                }
                (false, true) => {
                    ((std::cmp::min(x1, x2))..x_max + 1).for_each(|x| {
                        map.0[y1][x] += 1;
                    });
                    map
                }
                (false, false) => {
                    let x_min = std::cmp::min(x1, x2);

                    (0..x_max - x_min + 1).for_each(|i| {
                        map.0[if y1 > y2 { y1 - i } else { y1 + i }]
                            [if x1 > x2 { x1 - i } else { x1 + i }] += 1;
                    });

                    map
                }
            }
        });

    println!("{}", map);

    println!("{}", map.count_dangerous_area(2));
}
