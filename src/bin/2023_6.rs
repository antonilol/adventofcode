const INPUT: &str = include_str!("../../input/2023/6.txt");

fn main() {
    let mut lines = INPUT.lines();
    let time = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let distance = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());

    let answer_1 = time
        .zip(distance)
        .map(|(t, d)| {
            (1..t)
                .filter(|&t_button| (t - t_button) * t_button > d)
                .count()
        })
        .product::<usize>();

    let mut lines = INPUT.lines();
    let time = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse::<u64>()
        .unwrap();

    let answer_2 = (1..time)
        .filter(|&t_button| (time - t_button) * t_button > distance)
        .count();

    println!("answers: {answer_1} {answer_2}");
}
