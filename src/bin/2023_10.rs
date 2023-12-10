const INPUT: &str = include_str!("../../input/2023/10.txt");

#[derive(Clone, Copy, Default)]
struct Pipe {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    main_loop: bool,
}

impl Pipe {
    fn from_char(ch: u8) -> Self {
        let def = Default::default();
        match ch {
            b'S' => Self {
                main_loop: true,
                ..def
            },
            b'.' => def,
            b'|' => Self {
                north: true,
                south: true,
                ..def
            },
            b'-' => Self {
                east: true,
                west: true,
                ..def
            },
            b'L' => Self {
                north: true,
                east: true,
                ..def
            },
            b'J' => Self {
                north: true,
                west: true,
                ..def
            },
            b'7' => Self {
                south: true,
                west: true,
                ..def
            },
            b'F' => Self {
                east: true,
                south: true,
                ..def
            },
            _ => unreachable!(),
        }
    }
}

fn get<T: Copy>(map: &[Vec<T>], index: (usize, usize)) -> Option<T> {
    map.get(index.1).and_then(|v| v.get(index.0)).copied()
}

fn connecting_pipes(pos: (usize, usize), pipe_network: &[Vec<Pipe>]) -> [(usize, usize); 2] {
    let (x, y) = pos;
    let p = pipe_network[pos.1][pos.0];

    let mut pos = [(0, 0), (0, 0)];
    let mut pos_i = 0;

    if p.south {
        pos[pos_i] = (x, y + 1);
        pos_i += 1;
    }
    if p.east {
        pos[pos_i] = (x + 1, y);
        pos_i += 1;
    }
    if p.north {
        pos[pos_i] = (x, y - 1);
        pos_i += 1;
    }
    if p.west {
        pos[pos_i] = (x - 1, y);
        pos_i += 1;
    }

    assert!(pos_i == 2);

    pos
}

fn walk(
    pipe_network: &[Vec<Pipe>],
    prev_pos: &mut (usize, usize),
    current_pos: &mut (usize, usize),
) {
    let connections = connecting_pipes(*current_pos, pipe_network);
    let eq_to_previous = (connections[0] == *prev_pos, connections[1] == *prev_pos);
    *prev_pos = *current_pos;
    *current_pos = match eq_to_previous {
        (true, false) => connections[1],
        (false, true) => connections[0],
        _ => unreachable!(),
    };
}

fn is_enclosed(pipe_network: &[Vec<Pipe>], x: usize, y: usize) -> bool {
    pipe_network[y][..x].iter().fold(false, |enclosed, p| {
        if p.main_loop && p.north {
            !enclosed
        } else {
            enclosed
        }
    })
}

fn main() {
    let mut start = None;
    let mut pipe_network = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &b)| {
                    if b == b'S' {
                        assert!(start.is_none());
                        start = Some((x, y));
                    }

                    Pipe::from_char(b)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (sx, sy) = start.unwrap();

    let mut pos = [(0, 0), (0, 0)];
    let mut pos_length = 0;

    if get(&pipe_network, (sx, sy - 1)).is_some_and(|p| p.south) {
        pos[pos_length] = (sx, sy - 1);
        pos_length += 1;
        pipe_network[sy][sx].north = true;
    }
    if get(&pipe_network, (sx - 1, sy)).is_some_and(|p| p.east) {
        pos[pos_length] = (sx - 1, sy);
        pos_length += 1;
        pipe_network[sy][sx].west = true;
    }
    if get(&pipe_network, (sx, sy + 1)).is_some_and(|p| p.north) {
        pos[pos_length] = (sx, sy + 1);
        pos_length += 1;
        pipe_network[sy][sx].south = true;
    }
    if get(&pipe_network, (sx + 1, sy)).is_some_and(|p| p.west) {
        pos[pos_length] = (sx + 1, sy);
        pos_length += 1;
        pipe_network[sy][sx].east = true;
    }

    assert!(pos_length == 2);

    let [mut pos1, mut pos2] = pos;
    let mut prev_pos1 = (sx, sy);
    let mut prev_pos2 = (sx, sy);

    let mut answer_1 = 1;
    while pos1 != pos2 {
        pipe_network[pos1.1][pos1.0].main_loop = true;
        pipe_network[pos2.1][pos2.0].main_loop = true;

        walk(&pipe_network, &mut prev_pos1, &mut pos1);
        walk(&pipe_network, &mut prev_pos2, &mut pos2);

        answer_1 += 1;
    }
    pipe_network[pos1.1][pos1.0].main_loop = true;

    let mut answer_2 = 0;

    for (y, row) in pipe_network.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if !pipe.main_loop && is_enclosed(&pipe_network, x, y) {
                answer_2 += 1;
            }
        }
    }

    println!("answers: {answer_1} {answer_2}");
}
