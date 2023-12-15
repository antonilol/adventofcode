use adventofcode::day;

fn hash(sl: &[u8]) -> u8 {
    sl.iter()
        .fold(0, |h, &b| h.wrapping_add(b).wrapping_mul(17))
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

fn solve(input: &str) -> (usize, usize) {
    let answer_1 = input
        .trim_end()
        .split(',')
        .fold(0, |acc, s| acc + hash(s.as_bytes()) as usize);

    let mut boxes: [Vec<Lens>; 256] = core::array::from_fn(|_i| Vec::new());
    input.trim_end().split(',').for_each(|s| {
        let Some(label_end) = s.bytes().position(|c| matches!(c, b'=' | b'-')) else {
            unreachable!();
        };
        let label = &s[..label_end];
        let box_number = hash(label.as_bytes()) as usize;
        let place = s.as_bytes()[label_end] == b'=';
        if let Some(pos) = boxes[box_number].iter().position(|l| l.label == label) {
            if place {
                boxes[box_number][pos] = Lens {
                    label,
                    focal_length: s.as_bytes()[label_end + 1] - b'0',
                };
            } else {
                boxes[box_number].remove(pos);
            }
        } else if place {
            boxes[box_number].push(Lens {
                label,
                focal_length: s.as_bytes()[label_end + 1] - b'0',
            });
        }
    });

    let answer_2 = boxes.into_iter().enumerate().fold(0, |acc, (box_num, b)| {
        b.into_iter().enumerate().fold(acc, |acc, (i, l)| {
            acc + (box_num + 1) * (i + 1) * l.focal_length as usize
        })
    });

    (answer_1, answer_2)
}

day!(2023 15, 517965, 267372);
