use adventofcode::ext::{iter::IterExt, str::StrExt};
use cached::proc_macro::cached;

const INPUT: &str = include_str!("../../input/2023/12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Broken,
    Unknown,
}

#[cached]
fn arrangements(pat: &'static [Status], n: &'static [u64]) -> u64 {
    if let Some((&last, n_rest)) = n.split_last() {
        if let Some((&status, pat_rest)) = pat.split_last() {
            (if matches!(status, Status::Operational | Status::Unknown) {
                arrangements(pat_rest, n)
            } else {
                0
            }) + (if matches!(status, Status::Broken | Status::Unknown) {
                if pat.len() >= last as usize
                    && pat_rest[pat.len() - last as usize..]
                        .iter()
                        .all(|&s| matches!(s, Status::Broken | Status::Unknown))
                    && (pat.len() == last as usize
                        || matches!(
                            pat_rest[pat.len() - 1 - last as usize],
                            Status::Operational | Status::Unknown
                        ))
                {
                    if pat.len() != last as usize {
                        arrangements(&pat_rest[..pat.len() - 1 - last as usize], n_rest)
                    } else if n_rest.is_empty() {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            })
        } else {
            0
        }
    } else if pat
        .iter()
        .all(|p| matches!(p, Status::Operational | Status::Unknown))
    {
        1
    } else {
        0
    }
}

fn main() {
    let answer_1 = INPUT.lines().fold(0, |acc, line| {
        let mut split = line.split(' ');
        let l = split.next().unwrap().map_1d_vec(|b| match b {
            b'#' => Status::Broken,
            b'.' => Status::Operational,
            b'?' => Status::Unknown,
            _ => unreachable!(),
        });

        let n = split.next().unwrap().split(',').parse_all::<u64>().unwrap();
        let z = split.next();
        debug_assert!(z.is_none());

        acc + arrangements(Vec::leak(l), Vec::leak(n))
    });

    let answer_2 = INPUT.lines().fold(0, |acc, line| {
        let mut split = line.split(' ');
        let l = split.next().unwrap().map_1d_vec(|b| match b {
            b'#' => Status::Broken,
            b'.' => Status::Operational,
            b'?' => Status::Unknown,
            _ => unreachable!(),
        });

        let mut l_x5 = Vec::new();
        l_x5.extend_from_slice(&l);
        l_x5.push(Status::Unknown);
        l_x5.extend_from_slice(&l);
        l_x5.push(Status::Unknown);
        l_x5.extend_from_slice(&l);
        l_x5.push(Status::Unknown);
        l_x5.extend_from_slice(&l);
        l_x5.push(Status::Unknown);
        l_x5.extend_from_slice(&l);
        let l = l_x5;

        let n = split.next().unwrap().split(',').parse_all::<u64>().unwrap();

        let mut n_x5 = Vec::new();
        n_x5.extend_from_slice(&n);
        n_x5.extend_from_slice(&n);
        n_x5.extend_from_slice(&n);
        n_x5.extend_from_slice(&n);
        n_x5.extend_from_slice(&n);
        let n = n_x5;

        let z = split.next();
        debug_assert!(z.is_none());

        acc + arrangements(Vec::leak(l), Vec::leak(n))
    });

    println!("answers: {answer_1} {answer_2}");
}
