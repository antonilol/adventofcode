use adventofcode::day;
use core::{
    cmp::{max, min, Ordering},
    ops::Range,
};
use std::collections::HashMap;

struct Rule<'a> {
    items: Vec<(u8, Ordering, u32, &'a str)>,
    otherwise: &'a str,
}

#[derive(Clone, Debug)]
struct RatingRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RatingRange {
    fn split_off(&mut self, k: u8, v: u32, ord: Ordering) -> Self {
        let mut ret = self.clone();

        let (range, range2) = match k {
            b'x' => (&mut self.x, &mut ret.x),
            b'm' => (&mut self.m, &mut ret.m),
            b'a' => (&mut self.a, &mut ret.a),
            b's' => (&mut self.s, &mut ret.s),
            _ => unreachable!(),
        };

        match ord {
            Ordering::Greater => {
                *range2 = min(v + 1, range.end)..range.end;
                *range = range.start..max(v + 1, range.start);
            }
            Ordering::Less => {
                *range2 = range.start..min(v, range.end);
                *range = max(v, range.start)..range.end;
            }
            Ordering::Equal => unreachable!(),
        }

        ret
    }

    fn combinations(&self) -> u64 {
        (self.x.end - self.x.start) as u64
            * (self.m.end - self.m.start) as u64
            * (self.a.end - self.a.start) as u64
            * (self.s.end - self.s.start) as u64
    }
}

fn solve(input: &str) -> (u32, u64) {
    let mut blocks = input.split("\n\n");
    let rules = blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let br = line.bytes().position(|b| b == b'{').unwrap();
            let name = &line[..br];
            let mut items = line[..line.len() - 1][br + 1..].split(',');
            let otherwise = items.next_back().unwrap();
            let items = items
                .map(|item| {
                    let r = item.as_bytes()[0];
                    let ord = match item.as_bytes()[1] {
                        b'<' => Ordering::Less,
                        b'>' => Ordering::Greater,
                        _ => unreachable!(),
                    };
                    let mut rest = item[2..].split(':');
                    let target = rest.next().unwrap().parse().unwrap();
                    let next_rule = rest.next().unwrap();
                    (r, ord, target, next_rule)
                })
                .collect();
            (name, Rule { items, otherwise })
        })
        .collect::<HashMap<_, _>>();

    let answer_1 = blocks.next().unwrap().lines().fold(0, |acc, line| {
        let m = line[1..line.len() - 1]
            .split([',', '='])
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|s| {
                let [k, v] = s else {
                    unreachable!();
                };
                debug_assert_eq!(k.len(), 1);
                (k.as_bytes()[0], v.parse::<u32>().unwrap())
            })
            .collect::<HashMap<_, _>>();

        let mut rule = "in";
        while rule != "A" && rule != "R" {
            let rule_items = &rules[rule];
            rule = rule_items.otherwise;
            for (k, ord, target, next_rule) in &rule_items.items {
                let v = m[&k];
                if v.cmp(target) == *ord {
                    rule = next_rule;
                    break;
                }
            }
        }

        acc + if rule == "A" {
            m.into_values().sum()
        } else {
            0
        }
    });

    let mut ranges = vec![(
        "in",
        RatingRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
    )];
    let mut new = Vec::new();

    let mut answer_2 = 0;
    while !ranges.is_empty() {
        ranges.retain_mut(|r| {
            let rule = &rules[&r.0];
            for (k, ord, v, next_rule) in &rule.items {
                let s = r.1.split_off(*k, *v, *ord);
                if s.combinations() == 0 {
                    continue;
                }
                if *next_rule == "A" {
                    answer_2 += s.combinations();
                } else if *next_rule != "R" {
                    new.push((*next_rule, s));
                }
            }
            r.0 = rule.otherwise;
            if r.0 == "A" {
                answer_2 += r.1.combinations();
                return false;
            }
            r.0 != "R"
        });
        ranges.append(&mut new);
    }

    (answer_1, answer_2)
}

day!(2023 19, 330820, 123972546935551);
