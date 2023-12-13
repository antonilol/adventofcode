use core::cmp::min;

const INPUT: &str = include_str!("../../input/2023/13.txt");

fn horizontal(lines: &[Vec<u8>], orig: usize) -> usize {
    for i in 1..lines.len() {
        if 100 * i != orig
            && lines[..i]
                .iter()
                .rev()
                .zip(lines[i..].iter())
                .all(|(a, b)| a == b)
        {
            return 100 * i;
        }
    }
    0
}

fn vertical(lines: &[Vec<u8>], orig: usize) -> usize {
    for i in 1..lines[0].len() {
        if i != orig
            && lines.iter().all(|line| {
                let len = min(i, line.len() - i);
                line[..i + len]
                    .iter()
                    .rev()
                    .zip(line[i - len..].iter())
                    .take(len)
                    .all(|(b1, b2)| b1 == b2)
            })
        {
            return i;
        }
    }
    0
}

fn main() {
    let (answer_1, answer_2) = INPUT.split("\n\n").fold((0, 0), |mut acc, pat| {
        let mut lines = pat
            .lines()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut orig = 0;
        let v = horizontal(&lines, 0);
        if v != 0 {
            orig = v;
        }
        let v = vertical(&lines, 0);
        if orig == 0 && v != 0 {
            orig = v;
        }

        if orig == 0 {
            panic!("no reflection line");
        }

        acc.0 += orig;

        for i in 0..lines.len() {
            for j in 0..lines[0].len() {
                lines[i][j] = match lines[i][j] {
                    b'#' => b'.',
                    b'.' => b'#',
                    _ => unreachable!(),
                };

                let v = horizontal(&lines, orig);
                if v != 0 {
                    acc.1 += v;
                    return acc;
                }
                let v = vertical(&lines, orig);
                if v != 0 {
                    acc.1 += v;
                    return acc;
                }

                lines[i][j] = match lines[i][j] {
                    b'#' => b'.',
                    b'.' => b'#',
                    _ => unreachable!(),
                };
            }
        }

        panic!("no new reflection line");
    });

    println!("answers: {answer_1} {answer_2}");
}
