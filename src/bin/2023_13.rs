use adventofcode::{day, ext::str::StrExt};
use core::cmp::min;

#[derive(Clone, Copy, PartialEq)]
enum TerrainItem {
    Ash,
    Rocks,
}

impl TerrainItem {
    fn other(self) -> Self {
        match self {
            Self::Ash => Self::Rocks,
            Self::Rocks => Self::Ash,
        }
    }
}

fn horizontal(lines: &[Vec<TerrainItem>], orig: usize) -> usize {
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

fn vertical(lines: &[Vec<TerrainItem>], orig: usize) -> usize {
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

fn solve(input: &str) -> (usize, usize) {
    input.split("\n\n").fold((0, 0), |mut acc, pat| {
        let mut lines = pat.map_2d_vec(|b| match b {
            b'.' => TerrainItem::Ash,
            b'#' => TerrainItem::Rocks,
            _ => unreachable!(),
        });

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
                lines[i][j] = lines[i][j].other();

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

                lines[i][j] = lines[i][j].other();
            }
        }

        panic!("no new reflection line");
    })
}

day!(2023 13, 40006, 28627);
