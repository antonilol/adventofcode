const INPUT: &str = include_str!("../../input/2023/9.txt");

fn main() {
    let (answer_1, answer_2) = INPUT.lines().fold((0, 0), |acc, line| {
        let mut all_zeros = true;
        let mut numbers = vec![line
            .split(' ')
            .map(|s| {
                let n = s.parse::<i64>().unwrap();
                if n != 0 {
                    all_zeros = false;
                }
                n
            })
            .collect::<Vec<_>>()];

        while !all_zeros {
            all_zeros = true;
            numbers.push(
                numbers
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|sl| {
                        let [a, b] = sl else {
                            unreachable!();
                        };
                        let diff = b - a;
                        if diff != 0 {
                            all_zeros = false;
                        }
                        diff
                    })
                    .collect(),
            );
        }

        let (v1, v2) = numbers
            .into_iter()
            .rev()
            .fold((0, 0), |acc, v| (acc.0 + v.last().unwrap(), v[0] - acc.1));

        (acc.0 + v1, acc.1 + v2)
    });

    println!("answers: {answer_1} {answer_2}");
}
