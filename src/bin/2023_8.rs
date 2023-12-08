use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/2023/8.txt");

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut min, mut max) = if a < b { (a, b) } else { (b, a) };

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn steps(instr: &[u8], network: &HashMap<&str, (&str, &str)>, start: &str, part2: bool) -> u64 {
    let mut instr = instr.iter().cycle();
    let mut curr = start;
    let mut answer_1 = 0;
    while if part2 {
        !curr.ends_with('Z')
    } else {
        curr != "ZZZ"
    } {
        let dirs = network[curr];
        curr = match instr.next().unwrap() {
            b'L' => dirs.0,
            b'R' => dirs.1,
            _ => unreachable!(),
        };
        answer_1 += 1;
    }
    answer_1
}

fn main() {
    let mut split = INPUT.lines();
    let instr = split.next().unwrap().as_bytes();
    split.next(); //empty lines
    let network = split
        .map(|s| (&s[0..3], (&s[7..10], &s[12..15])))
        .collect::<HashMap<_, _>>();

    let answer_1 = steps(instr, &network, "AAA", false);

    let answer_2 = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| steps(instr, &network, s, true))
        .reduce(lcm)
        .unwrap();

    println!("answers: {answer_1} {answer_2}");
}
