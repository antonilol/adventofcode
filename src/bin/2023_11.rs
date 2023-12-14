use adventofcode::day;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Galaxy,
    Emptyx999999,
}

fn solve(input: &str) -> (usize, usize) {
    let mut image = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|ch| match ch {
                    b'.' => Field::Empty,
                    b'#' => Field::Galaxy,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // expand
    let mut i = 0;
    while i < image.len() {
        let row = &image[i];
        if row.iter().all(|&g| g == Field::Empty) {
            image.insert(i, vec![Field::Emptyx999999; row.len()]);
            i += 1;
        }
        i += 1;
    }
    let mut i = 0;
    while i < image[0].len() {
        if image
            .iter()
            .all(|row| matches!(row[i], Field::Empty | Field::Emptyx999999))
        {
            image.iter_mut().for_each(|v| {
                v.insert(i, Field::Emptyx999999);
            });
            i += 1;
        }
        i += 1;
    }

    let mut y_p1 = 0usize;
    let mut y_p2 = 0usize;
    let (coords_p1, coords_p2) =
        image
            .into_iter()
            .fold((Vec::new(), Vec::new()), |(mut p1, mut p2), row| {
                let mut x_p1 = 0usize;
                let mut x_p2 = 0usize;
                row.iter().for_each(|&g| match g {
                    Field::Empty => {
                        x_p1 += 1;
                        x_p2 += 1;
                    }
                    Field::Galaxy => {
                        p1.push((x_p1, y_p1));
                        p2.push((x_p2, y_p2));
                        x_p1 += 1;
                        x_p2 += 1;
                    }
                    Field::Emptyx999999 => {
                        x_p1 += 1;
                        x_p2 += 999999;
                    }
                });
                if row.into_iter().all(|g| g == Field::Emptyx999999) {
                    y_p1 += 1;
                    y_p2 += 999999;
                } else {
                    y_p1 += 1;
                    y_p2 += 1;
                }
                (p1, p2)
            });

    let mut answer_1 = 0;
    let mut answer_2 = 0;

    for (i, (&c1_p1, &c1_p2)) in coords_p1.iter().zip(coords_p2.iter()).enumerate() {
        for (&c2_p1, &c2_p2) in coords_p1[..i].iter().zip(coords_p2[..i].iter()) {
            answer_1 += c1_p1.0.abs_diff(c2_p1.0) + c1_p1.1.abs_diff(c2_p1.1);
            answer_2 += c1_p2.0.abs_diff(c2_p2.0) + c1_p2.1.abs_diff(c2_p2.1);
        }
    }

    (answer_1, answer_2)
}

day!(2023 11, 9684228, 483844716556);
