use std::{collections::HashMap, str};

const INPUT: &str = include_str!("../../input/2023_3.txt");

enum CharType {
    Empty,
    Symbol { gear: bool },
    Digit,
}

impl CharType {
    fn from(ch: u8) -> Self {
        match ch {
            b'.' => Self::Empty,
            b'0'..=b'9' => Self::Digit,
            ch => Self::Symbol { gear: ch == b'*' },
        }
    }
}

fn main() {
    let mut lines = INPUT.lines().map(|s| {
        let mut ret = String::with_capacity(s.len() + 2);
        ret.push('.');
        ret.push_str(s);
        ret.push('.');
        ret
    });
    let first_line = lines.next().unwrap();
    let len = first_line.len();
    let mut padded = vec![".".repeat(len), first_line];
    for l in lines {
        padded.push(l);
    }
    padded.push(".".repeat(len));

    let mut answer_1 = 0;
    padded.windows(3).fold(0, |acc, lines| {
        let [prev, curr, next] = lines else {
            unreachable!();
        };

        let [prev, curr, next] = [prev.as_bytes(), curr.as_bytes(), next.as_bytes()];

        let mut digit_start = None;

        'a: for (i, &b) in curr.iter().enumerate() {
            if let CharType::Digit = CharType::from(b) {
                digit_start.get_or_insert(i);
            } else if let Some(digit_start) = digit_start.take() {
                let digit: u32 = str::from_utf8(&curr[digit_start..i])
                    .unwrap()
                    .parse()
                    .unwrap();

                for j in digit_start - 1..i + 1 {
                    if let CharType::Symbol { .. } = CharType::from(prev[j]) {
                        answer_1 += digit;
                        continue 'a;
                    }
                    if let CharType::Symbol { .. } = CharType::from(next[j]) {
                        answer_1 += digit;
                        continue 'a;
                    }
                }
                if let CharType::Symbol { .. } = CharType::from(curr[digit_start - 1]) {
                    answer_1 += digit;
                    continue 'a;
                }
                if let CharType::Symbol { .. } = CharType::from(curr[i]) {
                    answer_1 += digit;
                    continue 'a;
                }
            }
        }

        assert!(digit_start.is_none());

        acc
    });

    let mut gears = HashMap::new();

    padded.windows(3).enumerate().fold(0, |acc, (y, lines)| {
        let [prev, curr, next] = lines else {
            unreachable!();
        };

        let [prev, curr, next] = [prev.as_bytes(), curr.as_bytes(), next.as_bytes()];

        let mut digit_start = None;

        'a: for (i, &b) in curr.iter().enumerate() {
            if let CharType::Digit = CharType::from(b) {
                digit_start.get_or_insert(i);
            } else if let Some(digit_start) = digit_start.take() {
                let digit: u32 = str::from_utf8(&curr[digit_start..i])
                    .unwrap()
                    .parse()
                    .unwrap();

                for j in digit_start - 1..i + 1 {
                    if let CharType::Symbol { gear } = CharType::from(prev[j]) {
                        // answer_2 += digit;
                        if gear {
                            gears.entry((j, y - 1)).or_insert(Vec::new()).push(digit);
                        }
                        continue 'a;
                    }
                    if let CharType::Symbol { gear } = CharType::from(next[j]) {
                        // answer_2 += digit;
                        if gear {
                            gears.entry((j, y + 1)).or_insert(Vec::new()).push(digit);
                        }
                        continue 'a;
                    }
                }
                if let CharType::Symbol { gear } = CharType::from(curr[digit_start - 1]) {
                    // answer_2 += digit;
                    if gear {
                        gears
                            .entry((digit_start - 1, y))
                            .or_insert(Vec::new())
                            .push(digit);
                    }
                    continue 'a;
                }
                if let CharType::Symbol { gear } = CharType::from(curr[i]) {
                    // answer_2 += digit;
                    if gear {
                        gears.entry((i, y)).or_insert(Vec::new()).push(digit);
                    }
                    continue 'a;
                }
            }
        }

        assert!(digit_start.is_none());

        acc
    });

    let answer_2 = gears
        .into_iter()
        .filter_map(|((_x, _y), numbers)| {
            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("answers: {answer_1} {answer_2}");
}
