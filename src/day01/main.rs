type Data = i64;

enum PreviousValue {
    NoPrevious,
    Increased(Data),
    NoChange(Data),
    Decreased(Data),
}
#[allow(dead_code)]
fn exercise_one() -> usize {
    include_str!("input.txt")
        .lines()
        .map(str::parse::<Data>)
        .filter_map(Result::ok)
        .scan(Option::<Data>::None, |prev, i| {
            let out = match prev {
                None => PreviousValue::NoPrevious,
                Some(prev) if *prev > i => PreviousValue::Decreased(i),
                Some(prev) if *prev < i => PreviousValue::Increased(i),
                Some(_) => PreviousValue::NoChange(i),
            };
            *prev = Some(i);
            Some(out)
        })
        .filter(|s| matches!(s, PreviousValue::Increased(_)))
        .count()
}

fn exercise_two() -> usize {
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse::<Data>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    (0..input.len())
        .filter_map(|i| {
            if i + 3 > input.len() {
                None
            } else {
                Some(input[i..i + 3].iter().sum())
            }
        })
        .scan(Option::<Data>::None, |prev, i| {
            let out = match prev {
                None => PreviousValue::NoPrevious,
                Some(prev) if *prev > i => PreviousValue::Decreased(i),
                Some(prev) if *prev < i => PreviousValue::Increased(i),
                Some(_) => PreviousValue::NoChange(i),
            };
            *prev = Some(i);
            Some(out)
        })
        .filter(|s| matches!(s, PreviousValue::Increased(_)))
        .count()
}

fn main() {
    println!("day01");

    println!("result 1 = {}", exercise_one());

    println!("result 2 = {}", exercise_two());
}
