#![allow(clippy::type_complexity)]

use adventofcode::{day, dir::Direction, ext::str::StrExt};

#[derive(Debug)]
struct Path {
    pos: (i32, i32),
    dir: Direction,
    same_dir_steps: u8,
    field_progress: u8,
}

#[derive(Debug)]
enum UpdateResult {
    Add(Path, Option<Path>),
    Keep,
    Discard,
}

impl UpdateResult {
    fn push(&mut self, this: &mut Path, p: Path) {
        match self {
            Self::Keep => *self = Self::Add(p, None),
            Self::Add(_, opt @ None) => *opt = Some(p),
            Self::Discard => {
                *this = p;
                *self = Self::Keep
            }
            _ => panic!("UpdateResult capacity exceeded"),
        }
    }
}

impl Path {
    fn update<const MIN_STEPS: usize, const MAX_STEPS: usize>(
        &mut self,
        map: &mut [Vec<(u8, [[bool; MAX_STEPS]; 4])>],
    ) -> UpdateResult {
        if self.field_progress != 0 {
            self.field_progress -= 1;
            return UpdateResult::Keep;
        }

        fn update_internal<const MAX_STEPS: usize>(
            p: &mut Path,
            map: &mut [Vec<(u8, [[bool; MAX_STEPS]; 4])>],
        ) -> bool {
            if p.same_dir_steps == MAX_STEPS as u8 {
                return false;
            }

            p.dir.move_pos(&mut p.pos);

            if p.pos.0 < 0
                || p.pos.0 as usize >= map[0].len()
                || p.pos.1 < 0
                || p.pos.1 as usize >= map.len()
            {
                return false;
            }

            let field = &mut map[p.pos.1 as usize][p.pos.0 as usize];
            if field.1[p.dir.as_index()][p.same_dir_steps as usize] {
                return false;
            }
            field.1[p.dir.as_index()][p.same_dir_steps as usize] = true;
            p.same_dir_steps += 1;

            p.field_progress = field.0 - 1;

            true
        }

        let mut p1 = Path {
            pos: self.pos,
            dir: self.dir.rotated(Direction::Right),
            same_dir_steps: 0,
            field_progress: 0,
        };
        let mut p2 = Path {
            pos: self.pos,
            dir: self.dir.rotated(Direction::Left),
            same_dir_steps: 0,
            field_progress: 0,
        };

        let st = self.same_dir_steps;
        let mut ret = if update_internal(self, map) {
            UpdateResult::Keep
        } else {
            UpdateResult::Discard
        };

        if st >= MIN_STEPS as u8 {
            if update_internal(&mut p1, map) {
                ret.push(self, p1);
            }
            if update_internal(&mut p2, map) {
                ret.push(self, p2);
            }
        }

        ret
    }
}

fn solve(input: &str) -> (usize, usize) {
    fn solve_part<const MIN_STEPS: usize, const MAX_STEPS: usize>(input: &str) -> usize {
        let mut map = input.map_2d_vec(|b| {
            (
                match b {
                    b'0'..=b'9' => b - b'0',
                    _ => unreachable!(),
                },
                [[false; MAX_STEPS]; 4],
            )
        });

        let mut ends = vec![
            Path {
                pos: (0, 0),
                dir: Direction::Right,
                same_dir_steps: 0,
                field_progress: 0,
            },
            Path {
                pos: (0, 0),
                dir: Direction::Down,
                same_dir_steps: 0,
                field_progress: 0,
            },
        ];
        let mut add = Vec::new();

        let mut heat_loss = 0;
        let w = map[0].len();
        let h = map.len();
        while !ends.iter().any(
            |&Path {
                 pos,
                 field_progress: heat_loss,
                 same_dir_steps,
                 ..
             }| {
                pos.0 as usize == w - 1
                    && pos.1 as usize == h - 1
                    && heat_loss == 0
                    && same_dir_steps >= MIN_STEPS as u8
            },
        ) {
            if ends.is_empty() {
                panic!("bottom right corner is unreachable");
            }
            ends.retain_mut(|p| match p.update::<MIN_STEPS, MAX_STEPS>(&mut map) {
                UpdateResult::Add(p2, p3) => {
                    add.push(p2);
                    if let Some(p3) = p3 {
                        add.push(p3);
                    }
                    true
                }
                UpdateResult::Keep => true,
                UpdateResult::Discard => false,
            });
            ends.append(&mut add);
            heat_loss += 1;
        }

        heat_loss
    }

    (solve_part::<0, 3>(input), solve_part::<4, 10>(input))
}

day!(2023 17, 668, 788);
