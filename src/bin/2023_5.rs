use adventofcode::{day, ext::range::RangeExt};

fn solve(input: &str) -> (u64, u64) {
    let mut blocks = input.split("\n\n");

    let mut seeds = blocks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = blocks.map(|s| {
        let mut iter = s.lines();
        iter.next(); // dont care about the map name
        iter.map(|l| {
            let mut nums = l.split(' ').map(|s| s.parse::<u64>().unwrap());

            let dest_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let len = nums.next().unwrap();

            (src_start..src_start + len, dest_start..dest_start + len)
        })
        .collect::<Vec<_>>()
    });

    maps.for_each(|map| {
        seeds.iter_mut().for_each(|a| {
            for (from, to) in &map {
                if from.contains(a) {
                    *a = *a - from.start + to.start;
                    break;
                }
            }
        });
    });
    let answer_1 = seeds.into_iter().min().unwrap();

    // part 2

    let mut blocks = input.split("\n\n");

    let mut seeds = blocks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|s| {
            let start = s[0].parse::<u64>().unwrap();
            let len = s[1].parse::<u64>().unwrap();
            start..start + len
        })
        .collect::<Vec<_>>();

    let maps = blocks.map(|s| {
        let mut iter = s.lines();
        iter.next(); // dont care about the map name
        iter.map(|l| {
            let mut nums = l.split(' ').map(|s| s.parse::<u64>().unwrap());

            let dest_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let len = nums.next().unwrap();

            (src_start..src_start + len, dest_start..dest_start + len)
        })
        .collect::<Vec<_>>()
    });

    maps.for_each(|map| {
        let mut i = 0;
        while i < seeds.len() {
            let mut a = &mut seeds[i];
            if a.is_empty() {
                seeds.remove(i);
                continue;
            }

            for (from, to) in &map {
                if a.overlaps_with(from) {
                    if a.start < from.start {
                        let part = a.start..from.start;
                        a.start = from.start;
                        if a.is_empty() {
                            // replace
                            *a = part;
                        } else if !part.is_empty() {
                            seeds.push(part);
                            a = &mut seeds[i];
                        }
                    }
                    if a.end > from.end {
                        let part = from.end..a.end;
                        a.end = from.end;
                        if a.is_empty() {
                            // replace
                            *a = part;
                        } else if !part.is_empty() {
                            seeds.push(part);
                            a = &mut seeds[i];
                        }
                    }

                    debug_assert!(a.start >= from.start && a.end <= from.end);
                    *a = a.start - from.start + to.start..a.end - from.start + to.start;
                    break;
                }
            }
            i += 1;
        }
    });
    let answer_2 = seeds.into_iter().map(|r| r.start).min().unwrap();

    (answer_1, answer_2)
}

day!(2023 5, 218513636, 81956384);
