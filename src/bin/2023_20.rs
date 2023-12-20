use adventofcode::{day, math::lcm};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
enum Pulse {
    Low,
    High,
}

enum ModuleType {
    FlipFlop { on: bool },
    Conjuction,
}

impl ModuleType {
    fn reset(&mut self) {
        match self {
            Self::FlipFlop { on } => {
                *on = false;
            }
            Self::Conjuction => {}
        }
    }
}

impl<'a> Module<'a> {
    fn process_signal(&mut self, pulse: Pulse) -> Option<Pulse> {
        match (&mut self.t, pulse) {
            (ModuleType::FlipFlop { on }, Pulse::Low) => {
                *on = !*on;
                Some(if *on { Pulse::High } else { Pulse::Low })
            }
            (ModuleType::FlipFlop { .. }, Pulse::High) => None,
            (ModuleType::Conjuction, _) => Some(
                if self.inputs.iter().all(|&(_, p)| matches!(p, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                },
            ),
        }
    }
}

struct Module<'a> {
    t: ModuleType,
    inputs: Vec<(&'a str, Pulse)>,
    output: Vec<&'a str>,
}

impl<'a: 'b, 'b> Module<'a> {
    fn reset(modules: impl Iterator<Item = &'b mut Self>) {
        modules.for_each(|m| {
            m.t.reset();
            m.inputs.iter_mut().for_each(|inp| inp.1 = Pulse::Low);
        })
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut broadcaster = None;
    let mut modules = input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();

            let output = b.split(", ").collect::<Vec<_>>();
            if a == "broadcaster" {
                broadcaster = Some(output);
                None
            } else {
                Some((
                    &a[1..],
                    Module {
                        t: match a.as_bytes()[0] {
                            b'&' => ModuleType::Conjuction,
                            b'%' => ModuleType::FlipFlop { on: false },
                            _ => unreachable!(),
                        },
                        inputs: Vec::new(),
                        output,
                    },
                ))
            }
        })
        .collect::<HashMap<_, _>>();

    let broadcaster = broadcaster.unwrap();

    for k in &broadcaster {
        modules
            .get_mut(k)
            .unwrap()
            .inputs
            .push(("broadcaster", Pulse::Low));
    }
    for (k, v) in modules
        .iter()
        .map(|(&k, v)| (k, v.output.clone()))
        .collect::<Vec<_>>()
    {
        for output in v {
            if let Some(v) = modules.get_mut(output) {
                v.inputs.push((k, Pulse::Low));
            }
        }
    }

    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        press_button(&mut low, &broadcaster, &mut modules, &mut high, None, 0);
    }

    let answer_1 = high * low;

    Module::reset(modules.values_mut());

    let mut module = "rx";
    let mut rx_source = loop {
        let inputs = &modules
            .iter()
            .find(|m| m.1.output.contains(&module))
            .unwrap()
            .1
            .inputs;
        if inputs.len() == 1 {
            module = inputs[0].0;
        } else {
            break inputs.iter().map(|&(s, _)| (s, None)).collect::<Vec<_>>();
        }
    };

    let mut i = 1;

    while {
        press_button(
            &mut 0,
            &broadcaster,
            &mut modules,
            &mut 0,
            Some(&mut rx_source),
            i,
        );
        rx_source.iter().any(|i| i.1.is_none())
    } {
        i += 1;
    }

    let answer_2 = rx_source
        .into_iter()
        .map(|(_s, p)| p.unwrap())
        .reduce(lcm)
        .unwrap();

    (answer_1, answer_2)
}

fn press_button(
    low: &mut usize,
    broadcaster: &Vec<&str>,
    modules: &mut HashMap<&str, Module<'_>>,
    high: &mut usize,
    mut part2_period: Option<&mut [(&str, Option<usize>)]>,
    i: usize,
) {
    *low += broadcaster.len() + 1;

    let mut pulses = broadcaster
        .iter()
        .map(|&d| (d, Pulse::Low))
        .collect::<VecDeque<_>>();

    while let Some((this, pulse)) = pulses.pop_front() {
        if let Some(m) = modules.get_mut(this) {
            if let Some(p) = Module::process_signal(m, pulse) {
                match p {
                    Pulse::High => *high += m.output.len(),
                    Pulse::Low => *low += m.output.len(),
                }
                pulses.extend(m.output.iter().map(|&d| (d, p)));
                let dest = m.output.to_vec();
                for d in dest {
                    if let Some(part2) = &mut part2_period {
                        if let Some(a) = part2.iter_mut().find(|&&mut (d2, _)| d2 == d) {
                            if matches!(p, Pulse::Low) && a.1.is_none() {
                                a.1 = Some(i);
                            }
                        }
                    }

                    if let Some(m) = modules.get_mut(d) {
                        if let Some(z) = m.inputs.iter_mut().find(|&&mut (d2, _)| d2 == this) {
                            z.1 = p;
                        }
                    }
                }
            }
        }
    }
}

day!(2023 20, 980457412, 232774988886497);
