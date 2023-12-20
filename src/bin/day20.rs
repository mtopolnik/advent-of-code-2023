use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

const BROADCASTER: &'static str = "broadcaster";

enum Module {
    Broadcaster {
        outputs: Vec<String>,
    },
    FlipFlop {
        outputs: Vec<String>,
        state: bool,
    },
    Conjunction {
        inputs: HashMap<String, bool>,
        outputs: Vec<String>,
    },
    Sink,
}
use Module::*;

struct Signal {
    sender_name: String,
    receiver_name: String,
    value: bool,
}

fn main() {
    let input = &read_to_string("input/day20.txt").unwrap();
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| {
            let (type_name, output_str) = line.split_once(" -> ").unwrap();
            let outputs = output_str
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            match type_name.chars().nth(0).unwrap() {
                '%' => (
                    type_name[1..].to_string(),
                    FlipFlop { outputs, state: false },
                ),
                '&' => (
                    type_name[1..].to_string(),
                    Conjunction { inputs: HashMap::new(), outputs },
                ),
                _ if type_name == BROADCASTER => (type_name.to_string(), Broadcaster { outputs }),
                _ => (type_name.to_string(), Sink),
            }
        })
        .collect();
    for name in modules.keys().cloned().collect::<Vec<_>>() {
        let empty_outputs = vec![];
        let outputs = match modules.get(&*name).unwrap() {
            Broadcaster { outputs } => outputs,
            FlipFlop { outputs, .. } => outputs,
            Conjunction { outputs, .. } => outputs,
            Sink => &empty_outputs,
        }
        .iter()
        .cloned()
        .collect::<Vec<_>>();
        for output_name in outputs {
            if let Some(output_module) = modules.get_mut(&output_name) {
                if let Conjunction { inputs, .. } = output_module {
                    inputs.insert(name.clone(), false);
                }
            } else {
                modules.insert(output_name.clone(), Sink);
            }
        }
    }
    let Some(Broadcaster { outputs }) = modules.get(BROADCASTER) else {
        panic!();
    };
    let broadcast_receivers = outputs.clone();
    let (mut low_signal_total, mut high_signal_total) = (0, 0);
    let mut cycle_lengths = [0_usize; 4];
    for i in 1.. {
        let (low_signal_count, high_signal_count) =
            press_button(&broadcast_receivers, &mut modules, &mut cycle_lengths, i);
        low_signal_total += low_signal_count;
        high_signal_total += high_signal_count;
        if i == 1000 {
            println!(
                "Part 1 low: {low_signal_total}, high: {high_signal_total}, product: {}",
                low_signal_total * high_signal_total
            );
        }
        if cycle_lengths.iter().all(|&it| it != 0) {
            println!(
                "Part 2 cycle lengths: {:?}, product: {}",
                cycle_lengths,
                cycle_lengths.iter().fold(1, |acc, n| acc * n)
            );
            break;
        }
    }
}

fn press_button(
    broadcast_receivers: &[String],
    modules: &mut HashMap<String, Module>,
    cycle_lengths: &mut [usize; 4],
    press_count: usize,
) -> (usize, usize) {
    let mut signal_queue = VecDeque::new();
    let signal_queue = &mut signal_queue;
    let (mut low_signal_count, mut high_signal_count) = (1, 0);
    low_signal_count += send_signal(signal_queue, BROADCASTER, broadcast_receivers, false);
    loop {
        let Some(Signal { sender_name, receiver_name, value }) = signal_queue.pop_front() else {
            break;
        };
        if !value {
            if receiver_name == "bh" && cycle_lengths[0] == 0 {
                cycle_lengths[0] = press_count;
            } else if receiver_name == "dl" && cycle_lengths[1] == 0 {
                cycle_lengths[1] = press_count;
            } else if receiver_name == "ns" && cycle_lengths[2] == 0 {
                cycle_lengths[2] = press_count;
            } else if receiver_name == "vd" && cycle_lengths[3] == 0 {
                cycle_lengths[3] = press_count;
            }
        }
        match modules.get_mut(&receiver_name).unwrap() {
            FlipFlop { outputs, state } => {
                if !value {
                    *state = !*state;
                    *(if *state {
                        &mut high_signal_count
                    } else {
                        &mut low_signal_count
                    }) += send_signal(signal_queue, &receiver_name, outputs, *state);
                }
            }
            Conjunction { inputs, outputs } => {
                inputs.insert(sender_name, value);
                let signal = !inputs.values().all(|v| *v);
                *(if signal {
                    &mut high_signal_count
                } else {
                    &mut low_signal_count
                }) += send_signal(signal_queue, &receiver_name, outputs, signal);
            }
            Sink => {}
            _ => {}
        };
    }
    (low_signal_count, high_signal_count)
}

fn send_signal(
    signal_queue: &mut VecDeque<Signal>,
    sender_name: &str,
    receiver_names: &[String],
    value: bool,
) -> usize {
    for receiver_name in receiver_names {
        signal_queue.push_back(Signal {
            sender_name: sender_name.to_string(),
            receiver_name: receiver_name.clone(),
            value,
        });
    }
    receiver_names.len()
}
