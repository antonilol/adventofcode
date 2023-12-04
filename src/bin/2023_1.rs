const INPUT: &str = include_str!("../../input/2023_1.txt");

fn digit(sl: &[u8]) -> Option<u8> {
    const NUMBERS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, &d) in NUMBERS.iter().enumerate() {
        if sl.starts_with(d.as_bytes()) {
            return Some((i + 1) as u8);
        }
    }
    None
}

fn main() {
    let answer_1 = INPUT.lines().fold(0, |acc, line| {
        let mut first = None;
        let mut last = None;

        for &b in line.as_bytes() {
            let b = b.wrapping_sub(b'0');
            if b < 10 {
                first.get_or_insert(b);
                let _ = last.insert(b);
            }
        }

        acc + (first.unwrap() * 10 + last.unwrap()) as u32
    });

    let answer_2 = INPUT.lines().fold(0, |acc, line| {
        let mut first = None;
        let mut last = None;

        for i in 0..line.len() {
            let sl = &line.as_bytes()[i..];

            let b = sl[0].wrapping_sub(b'0');
            if b < 10 {
                first.get_or_insert(b);
                let _ = last.insert(b);
            } else if let Some(n) = digit(sl) {
                first.get_or_insert(n);
                let _ = last.insert(n);
            }
        }

        acc + (first.unwrap() * 10 + last.unwrap()) as u32
    });

    println!("answers: {answer_1} {answer_2}");
}
