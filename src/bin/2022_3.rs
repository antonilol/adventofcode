use adventofcode::day;

fn priority(item_type: u8) -> u32 {
    (match item_type {
        b'a'..=b'z' => item_type - b'a' + 1,
        b'A'..=b'Z' => item_type - b'A' + 27,
        _ => unreachable!(),
    }) as u32
}

fn solve(input: &str) -> (u32, u32) {
    let answer_1 = input.lines().fold(0, |acc, line| {
        let (c1, c2) = line.as_bytes().split_at(line.len() / 2);

        acc + c1
            .iter()
            .enumerate()
            .filter(|&(index, item)| !c1[index + 1..].contains(item) && c2.contains(item))
            .map(|(_index, &item)| priority(item))
            .sum::<u32>()
    });

    let answer_2 = input
        .lines()
        .map(|str| str.as_bytes())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .fold(0, |acc, line| {
            let &[r1, r2, r3] = line else { unreachable!() };

            let mut iter = r1.iter().enumerate().filter(|&(index, item)| {
                !r1[index + 1..].contains(item) && r2.contains(item) && r3.contains(item)
            });
            let (_index, &badge) = iter.next().unwrap();
            assert!(iter.next().is_none());

            acc + priority(badge)
        });

    (answer_1, answer_2)
}

day!(2022 3, 7889, 2825);
