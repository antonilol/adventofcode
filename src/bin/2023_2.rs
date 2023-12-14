use adventofcode::day;
use core::cmp::max;

fn solve(input: &str) -> (u32, u32) {
    let answer_1 = input.lines().fold(0, |acc, line| {
        let mut a = line.split(": ");
        let game = a.next().unwrap();
        debug_assert!(game.starts_with("Game "));
        let game_id: u32 = game[5..].parse().unwrap();
        let items = a
            .next()
            .unwrap()
            .split([' ', ',', ';'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let mut items = items.chunks_exact(2);

        for item in items.by_ref() {
            let [amount, color] = item else {
                unreachable!();
            };
            let amount: u32 = amount.parse().unwrap();
            if amount
                > match *color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!(),
                }
            {
                return acc;
            }
        }

        debug_assert!(items.remainder().is_empty());

        acc + game_id
    });

    let answer_2 = input.lines().fold(0, |acc, line| {
        let mut a = line.split(": ");
        a.next();
        let items = a
            .next()
            .unwrap()
            .split([' ', ',', ';'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let mut items = items.chunks_exact(2);

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for item in items.by_ref() {
            let [amount, color] = item else {
                unreachable!();
            };
            let amount: u32 = amount.parse().unwrap();
            match *color {
                "red" => r = max(amount, r),
                "green" => g = max(amount, g),
                "blue" => b = max(amount, b),
                _ => unreachable!(),
            }
        }

        debug_assert!(items.remainder().is_empty());

        acc + r * g * b
    });

    (answer_1, answer_2)
}

day!(2023 2, 2545, 78111);
