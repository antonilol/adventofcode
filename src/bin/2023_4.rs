use adventofcode::ext::CellExt;
use core::cell::Cell;

const INPUT: &str = include_str!("../../input/2023_4.txt");

fn main() {
    let answer_1 = INPUT.lines().fold(0, |acc, line| {
        let mut split = line.split(' ').filter(|s| !s.is_empty());

        let next = split.next();
        debug_assert_eq!(next, Some("Card"));
        split.next().unwrap(); // number and ':'

        let mut my_numbers = Vec::new();

        loop {
            let n = split.next().unwrap();
            if n == "|" {
                break;
            }
            let num: u32 = n.parse().unwrap();
            my_numbers.push(num);
        }

        let winning_numbers = split.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

        let mut score = 0u32;
        for n in my_numbers {
            if winning_numbers.contains(&n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        acc + score
    });

    let cards = INPUT
        .lines()
        .map(|line| {
            let mut split = line.split(' ').filter(|s| !s.is_empty());

            let next = split.next();
            debug_assert_eq!(next, Some("Card"));
            split.next().unwrap(); // number and ':'

            let mut my_numbers = Vec::new();

            loop {
                let n = split.next().unwrap();
                if n == "|" {
                    break;
                }
                let num: u32 = n.parse().unwrap();
                my_numbers.push(num);
            }

            let winning_numbers = split.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

            let mut win_amount = 0u32;
            for n in my_numbers {
                if winning_numbers.contains(&n) {
                    win_amount += 1;
                }
            }

            (
                // amount of winning numbers on this card
                win_amount,
                // amount of cards of this
                Cell::new(1),
            )
        })
        .collect::<Vec<_>>();

    let mut answer_2 = 0u32;
    for (i, card1) in cards.iter().enumerate() {
        let am = card1.1.get();
        answer_2 += am;
        for card2 in cards.iter().skip(i + 1).take(card1.0 as usize) {
            card2.1.add(am);
        }
    }

    println!("answers: {answer_1} {answer_2}");
}
