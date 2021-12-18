fn exercise_one() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|s| s[..12].chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (gamma, epsilon) = (0..matrix.len())
        .fold(Vec::new(), |matrix_inverse, j| {
            (0..matrix[0].len()).fold(matrix_inverse, |mut matrix_inverse, i| {
                if i >= matrix_inverse.len() {
                    matrix_inverse.push(vec![]);
                }
                matrix_inverse[i].push(matrix[j][i]);
                matrix_inverse
            })
        })
        .into_iter()
        .map(|line| {
            line.iter().filter(|i| **i == '0').count() < line.iter().filter(|i| **i == '1').count()
        })
        .fold((0, 0), |(g, e), i| {
            ((g << 1) + (i as i64), (e << 1) + (!i as i64))
        });

    println!("{}", gamma * epsilon);
}

fn exercise_two() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|s| {
            s[..12]
                .chars()
                .map(|x| match x {
                    '1' => true,
                    '0' => false,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn get_result(mut matrix: Vec<Vec<bool>>, oxygen_generator_rating_method: bool) -> i64 {
        let mut i = 0;
        while i != matrix[0].len() && matrix.len() > 1 {
            let element_at_i = matrix.iter().map(|line| line[i]).collect::<Vec<_>>();

            let criteria = if oxygen_generator_rating_method {
                element_at_i.iter().filter(|i| !(**i)).count()
                    <= element_at_i.iter().filter(|i| **i).count()
            } else {
                element_at_i.iter().filter(|i| **i).count()
                    < element_at_i.iter().filter(|i| !(**i)).count()
            };

            matrix = matrix
                .into_iter()
                .filter(|line| line[i] == criteria)
                .collect::<Vec<_>>();
            i += 1;
        }

        matrix[0]
            .iter()
            .fold((0, 0), |(g, e), i| {
                ((g << 1) + (*i as i64), (e << 1) + (!i as i64))
            })
            .0
    }

    println!(
        "{:?}",
        get_result(matrix.clone(), true) * get_result(matrix, false)
    );
}

fn main() {
    println!("day03");

    exercise_one();

    exercise_two();
}
