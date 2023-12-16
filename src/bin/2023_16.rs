use adventofcode::{
    day,
    dir::Direction,
    ext::{intoiterator::IntoIteratorExt, str::StrExt},
};
use core::{cmp::max, mem::take};

enum TileType {
    Empty,
    Splitter { horizontal: bool },
    Mirror { to_top_right: bool },
}

struct Tile {
    t: TileType,
    energized_dirs: [bool; 4],
}

fn solve(input: &str) -> (usize, usize) {
    let mut map = input.map_2d_vec(|b| Tile {
        t: match b {
            b'.' => TileType::Empty,
            b'/' | b'\\' => TileType::Mirror {
                to_top_right: b == b'/',
            },
            b'-' | b'|' => TileType::Splitter {
                horizontal: b == b'-',
            },
            _ => unreachable!(),
        },
        energized_dirs: [false; 4],
    });

    let answer_1 = energized_tiles(&mut map, ((0, 0), Direction::Right));

    let mut answer_2 = 0;
    let w = map[0].len();
    let h = map.len();
    (0..w).for_each(|i| {
        answer_2 = max(
            answer_2,
            energized_tiles(&mut map, ((i as i32, 0), Direction::Down)),
        );
        answer_2 = max(
            answer_2,
            energized_tiles(&mut map, ((i as i32, (h - 1) as i32), Direction::Up)),
        );
    });
    (0..h).for_each(|i| {
        answer_2 = max(
            answer_2,
            energized_tiles(&mut map, ((0, i as i32), Direction::Right)),
        );
        answer_2 = max(
            answer_2,
            energized_tiles(&mut map, (((w - 1) as i32, i as i32), Direction::Left)),
        );
    });

    (answer_1, answer_2)
}

fn energized_tiles(map: &mut [Vec<Tile>], start: ((i32, i32), Direction)) -> usize {
    let mut beams = vec![start];
    let mut new = Vec::new();
    while !beams.is_empty() {
        beams.retain_mut(|b| {
            if b.0 .0 < 0
                || b.0 .0 as usize >= map[0].len()
                || b.0 .1 < 0
                || b.0 .1 as usize >= map.len()
            {
                return false;
            }

            let tile = &mut map[b.0 .1 as usize][b.0 .0 as usize];
            let energized = &mut tile.energized_dirs[b.1.as_index()];
            if !*energized {
                *energized = true;
                match tile.t {
                    TileType::Empty => {}
                    TileType::Splitter { horizontal } => {
                        if horizontal != b.1.is_horizontal() {
                            let mut pos2 = b.0;
                            let dir1 = b.1.rotated(Direction::Right);
                            let dir2 = b.1.rotated(Direction::Left);
                            b.1 = dir1;
                            dir2.move_pos(&mut pos2);
                            new.push((pos2, dir2));
                        }
                    }
                    TileType::Mirror { to_top_right } => {
                        let dir = match b.1 {
                            Direction::Up => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                        };
                        if to_top_right {
                            b.1 = dir.flipped();
                        } else {
                            b.1 = dir;
                        }
                    }
                }
                b.1.move_pos(&mut b.0);

                true
            } else {
                false
            }
        });
        beams.append(&mut new);
    }

    map.iter_mut()
        .map(|row| row.iter_mut())
        .fold_2d(0, |acc, tile, _| {
            acc + if take(&mut tile.energized_dirs).iter().any(|&a| a) {
                1
            } else {
                0
            }
        })
}

day!(2023 16, 7543, 8231);
