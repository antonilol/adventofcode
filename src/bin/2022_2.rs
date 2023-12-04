const INPUT: &str = include_str!("../../input/2022/2.txt");

#[derive(Debug, Clone, Copy)]
enum Outcome {
    L = 0,
    T = 3,
    W = 6,
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Outcome {
    fn from(char: u8) -> Self {
        use Outcome::*;

        match char {
            b'X' => L,
            b'Y' => T,
            b'Z' => W,
            _ => unreachable!(),
        }
    }

    fn decide(a: Shape, b: Shape) -> Self {
        use Outcome::*;

        match (b as i8 - a as i8 - 2).rem_euclid(3) as u32 {
            0 => L,
            1 => T,
            2 => W,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn from(char: u8) -> Self {
        use Shape::*;

        match char {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scissors,
            _ => unreachable!(),
        }
    }

    fn round_score(a: Self, b: Self) -> u32 {
        b as u32 + Outcome::decide(a, b) as u32
    }

    fn choose(a: Self, outcome: Outcome) -> Self {
        use Shape::*;

        match (outcome as u32 / 3 + 2 + a as u32).rem_euclid(3) {
            1 => Rock,
            2 => Paper,
            0 => Scissors,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let answer_1 = INPUT.lines().fold(0, |acc, line| {
        let &[a, _space, b] = line.as_bytes() else {
            unreachable!();
        };
        acc + Shape::round_score(Shape::from(a), Shape::from(b))
    });

    let answer_2 = INPUT.lines().fold(0, |acc, line| {
        let &[a, _space, outcome] = line.as_bytes() else {
            unreachable!();
        };
        let a = Shape::from(a);
        acc + Shape::round_score(a, Shape::choose(a, Outcome::from(outcome)))
    });

    println!("answers: {answer_1} {answer_2}");
}
