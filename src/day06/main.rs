const SIMULATION_DAYS: i32 = 256;

fn one() {
    let mut fishes = include_str!("input.txt")
        .replace("\n", "")
        .split(',')
        .filter_map(|word| word.parse::<u32>().ok())
        .collect::<Vec<_>>();

    (0..SIMULATION_DAYS).for_each(|_| {
        let mut new = vec![];
        fishes = fishes
            .iter()
            .map(|i| match i {
                0 => {
                    new.push(8);
                    6
                }
                i => i - 1,
            })
            .collect::<Vec<_>>();
        fishes.append(&mut new);
    });

    println!("count= {}", fishes.len());
}

fn two() {
    let fishes = include_str!("input.txt")
        .replace("\n", "")
        .split(',')
        .filter_map(|word| word.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut fishes: [usize; 9] = (0..8 + 1)
        .map(|c| fishes.iter().filter(|i| **i == c).count())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (0..SIMULATION_DAYS).for_each(|_| {
        let mut temp = (0..7 + 1).map(|c| fishes[c + 1]).collect::<Vec<_>>();

        temp[6] += fishes[0];
        temp.push(fishes[0]);

        fishes = temp.try_into().unwrap();
    });

    println!("count= {}", fishes.iter().sum::<usize>());
}

fn main() {
    println!("day06");

    one();
    two();
}
