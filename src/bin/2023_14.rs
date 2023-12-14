use adventofcode::ext::str::StrExt;

const INPUT: &str = include_str!("../../input/2023/14.txt");

#[derive(Clone, Copy, PartialEq)]
enum Object {
    RoundedRock,
    CubeRock,
    Empty,
}

fn slide_north(map: &mut [Vec<Object>]) {
    for x in 0..map[0].len() {
        let mut roll_to = 0;
        for y in 0..map.len() {
            match map[y][x] {
                Object::RoundedRock => {
                    let tmp = map[y][x];
                    map[y][x] = map[roll_to][x];
                    map[roll_to][x] = tmp;
                    roll_to += 1;
                }
                Object::CubeRock => {
                    roll_to = y + 1;
                }
                Object::Empty => {}
            }
        }
    }
}

fn slide_west(map: &mut [Vec<Object>]) {
    for y in 0..map.len() {
        let mut roll_to = 0;
        for x in 0..map[0].len() {
            match map[y][x] {
                Object::RoundedRock => {
                    map[y].swap(x, roll_to);
                    roll_to += 1;
                }
                Object::CubeRock => {
                    roll_to = x + 1;
                }
                Object::Empty => {}
            }
        }
    }
}

fn slide_south(map: &mut [Vec<Object>]) {
    for x in 0..map[0].len() {
        let mut roll_to = map.len() - 1;
        for y in (0..map.len()).rev() {
            match map[y][x] {
                Object::RoundedRock => {
                    let tmp = map[y][x];
                    map[y][x] = map[roll_to][x];
                    map[roll_to][x] = tmp;
                    roll_to = roll_to.wrapping_sub(1);
                }
                Object::CubeRock => {
                    roll_to = y.wrapping_sub(1);
                }
                Object::Empty => {}
            }
        }
    }
}

fn slide_east(map: &mut [Vec<Object>]) {
    for y in 0..map.len() {
        let mut roll_to = map[0].len() - 1;
        for x in (0..map[0].len()).rev() {
            match map[y][x] {
                Object::RoundedRock => {
                    map[y].swap(x, roll_to);
                    roll_to = roll_to.wrapping_sub(1);
                }
                Object::CubeRock => {
                    roll_to = x.wrapping_sub(1);
                }
                Object::Empty => {}
            }
        }
    }
}

fn slide(map: &mut [Vec<Object>]) {
    slide_north(map);
    slide_west(map);
    slide_south(map);
    slide_east(map);
}

fn main() {
    let mut map = INPUT.map_2d_vec(|b| match b {
        b'O' => Object::RoundedRock,
        b'#' => Object::CubeRock,
        b'.' => Object::Empty,
        _ => unreachable!(),
    });

    slide_north(&mut map);

    let answer_1 = map.iter().rev().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .filter(|o| matches!(o, Object::RoundedRock))
            .count()
            * (y + 1)
    });

    slide_west(&mut map);
    slide_south(&mut map);
    slide_east(&mut map);

    let mut snapshots = vec![map.clone()];
    let pos = loop {
        slide(&mut map);
        if let Some(pos) = snapshots.iter().position(|sn| sn == &map) {
            break pos;
        }
        snapshots.push(map.clone());
    };
    let map = &snapshots[pos + (999999999 - pos) % (snapshots.len() - pos)];

    let answer_2 = map.iter().rev().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .filter(|o| matches!(o, Object::RoundedRock))
            .count()
            * (y + 1)
    });

    println!("answers: {answer_1} {answer_2}");
}
