use core::mem::swap;
use std::collections::{HashSet, VecDeque};

use adventofcode::{day, dir::Direction, ext::intoiterator::IntoIteratorExt};

#[derive(Clone)]
struct Tile {
    dug_out: bool,
}

fn solve(input: &str) -> (usize, usize) {
    let instructions = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let dir = match split.next().unwrap() {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                _ => unreachable!(),
            };
            let dist = split.next().unwrap().parse::<u64>().unwrap();

            (dir, dist)
        })
        .collect::<Vec<_>>();

    let mut digger = (0, 0);
    let mut map = {
        let mut v = VecDeque::new();
        let mut row = VecDeque::new();
        row.push_back(Tile { dug_out: true });
        v.push_back(row);
        v
    };
    for &(dir, dist) in instructions.iter() {
        for _ in 0..dist {
            dir.move_pos(&mut digger);
            if digger.0 < 0 {
                map.iter_mut()
                    .for_each(|row| row.push_front(Tile { dug_out: false }));
                debug_assert_eq!(digger.0, -1);
                digger.0 = 0;
            } else if digger.0 as usize >= map[0].len() {
                map.iter_mut()
                    .for_each(|row| row.push_back(Tile { dug_out: false }));
            }
            if digger.1 < 0 {
                map.push_front({
                    let mut v = VecDeque::new();
                    for _ in 0..map[0].len() {
                        v.push_back(Tile { dug_out: false });
                    }
                    v
                });
                debug_assert_eq!(digger.1, -1);
                digger.1 = 0;
            } else if digger.1 as usize >= map.len() {
                map.push_back({
                    let mut v = VecDeque::new();
                    for _ in 0..map[0].len() {
                        v.push_back(Tile { dug_out: false });
                    }
                    v
                });
            }
            map[digger.1 as usize][digger.0 as usize].dug_out = true;
        }
    }

    let mut ends = HashSet::new();
    ends.insert((map[0].iter().position(|t| t.dug_out).unwrap() + 1, 1));
    let mut ends2 = HashSet::new();
    while !ends.is_empty() {
        for &(x, y) in ends.iter() {
            map[y][x].dug_out = true;
            ends2.insert((x + 1, y));
            ends2.insert((x, y + 1));
            ends2.insert((x - 1, y));
            ends2.insert((x, y - 1));
        }
        ends.clear();
        swap(&mut ends, &mut ends2);
        ends.retain(|&(x, y)| !map[y][x].dug_out);
    }

    let answer_1 = map.fold_2d(0, |acc, tile, _| acc + if tile.dug_out { 1 } else { 0 });

    let instructions = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            split.next();
            split.next();
            let color = split.next().unwrap();
            let dist = u64::from_str_radix(&color[2..color.len() - 2], 16).unwrap();
            let dir = match color.as_bytes()[color.len() - 2] {
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                _ => unreachable!(),
            };

            (dir, dist)
        })
        .collect::<Vec<_>>();

    let mut digger = (0usize, 0usize);
    let mut map = VecDeque::from([VecDeque::from([Tile { dug_out: true }])]);
    let mut row_map = VecDeque::new();
    let mut col_map = VecDeque::new();
    for &(dir, mut dist) in instructions.iter() {
        let d = dir.as_pos();

        if dir.is_horizontal() {
            loop {
                let mut long_edge = digger.0.wrapping_add(d.0 as usize);
                digger.0 = digger.0.wrapping_add((d.0 * 2) as usize);
                if digger.0 <= map[0].len() {
                    let b = long_edge / 2;
                    if dist <= row_map[b] {
                        let z = row_map[b];
                        row_map[b] = dist - 1;
                        let ins = if dir.is_positive() { b + 1 } else { b };
                        row_map.insert(ins, z - dist);

                        map.iter_mut().for_each(|row| {
                            let dug_out = row[long_edge].dug_out;
                            row.insert(long_edge, Tile { dug_out });
                            row.insert(long_edge, Tile { dug_out });
                        });

                        if dir.is_negative() {
                            digger.0 += 2;
                            long_edge += 2;
                        }
                    } else {
                        dist -= row_map[b] + 1;
                        if dist != 0 {
                            map[digger.1][digger.0].dug_out = true;
                            map[digger.1][long_edge].dug_out = true;
                            continue;
                        }
                    }
                } else if digger.0 == usize::MAX - 1 {
                    map.iter_mut().for_each(|row| {
                        row.push_front(Tile { dug_out: false });
                        row.push_front(Tile { dug_out: false });
                    });

                    row_map.push_front(dist - 1);
                    digger.0 = 0;
                    long_edge = 1;
                } else if digger.0 >= map[0].len() {
                    map.iter_mut().for_each(|row| {
                        row.push_back(Tile { dug_out: false });
                        row.push_back(Tile { dug_out: false });
                    });

                    row_map.push_back(dist - 1);
                }
                map[digger.1][digger.0].dug_out = true;
                map[digger.1][long_edge].dug_out = true;
                break;
            }
        } else {
            loop {
                let mut long_edge = digger.1.wrapping_add(d.1 as usize);
                digger.1 = digger.1.wrapping_add((d.1 * 2) as usize);
                if digger.1 <= map.len() {
                    let b = long_edge / 2;
                    if dist <= col_map[b] {
                        let z = col_map[b];
                        col_map[b] = dist - 1;
                        let ins = if dir.is_positive() { b + 1 } else { b };
                        col_map.insert(ins, z - dist);

                        map.insert(long_edge, map[long_edge].clone());
                        map.insert(long_edge, map[long_edge].clone());

                        if dir.is_negative() {
                            digger.1 += 2;
                            long_edge += 2;
                        }
                    } else {
                        dist -= col_map[b] + 1;
                        if dist != 0 {
                            map[digger.1][digger.0].dug_out = true;
                            map[long_edge][digger.0].dug_out = true;
                            continue;
                        }
                    }
                } else if digger.1 == usize::MAX - 1 {
                    map.push_front(VecDeque::from(vec![Tile { dug_out: false }; map[0].len()]));
                    map.push_front(VecDeque::from(vec![Tile { dug_out: false }; map[0].len()]));

                    col_map.push_front(dist - 1);
                    digger.1 = 0;
                    long_edge = 1;
                } else if digger.1 >= col_map.len() {
                    map.push_back(VecDeque::from(vec![Tile { dug_out: false }; map[0].len()]));
                    map.push_back(VecDeque::from(vec![Tile { dug_out: false }; map[0].len()]));

                    col_map.push_back(dist - 1);
                }
                map[digger.1][digger.0].dug_out = true;
                map[long_edge][digger.0].dug_out = true;
                break;
            }
        }
    }

    let mut ends = HashSet::new();
    ends.insert((map[0].iter().position(|t| t.dug_out).unwrap() + 1, 1));
    let mut ends2 = HashSet::new();
    while !ends.is_empty() {
        for &(x, y) in ends.iter() {
            map[y][x].dug_out = true;
            ends2.insert((x + 1, y));
            ends2.insert((x, y + 1));
            ends2.insert((x - 1, y));
            ends2.insert((x, y - 1));
        }
        ends.clear();
        swap(&mut ends, &mut ends2);
        ends.retain(|&(x, y)| !map[y][x].dug_out);
    }

    let answer_2 = map.fold_2d(0, |acc, tile, (x, y)| {
        acc + if tile.dug_out {
            (if x % 2 == 1 {
                row_map[x / 2] as usize
            } else {
                1
            }) * if y % 2 == 1 {
                col_map[y / 2] as usize
            } else {
                1
            }
        } else {
            0
        }
    });

    (answer_1, answer_2)
}

day!(2023 18, 33491, 87716969654406);
