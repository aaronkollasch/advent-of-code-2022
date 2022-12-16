use itertools::Itertools;
use std::collections::HashMap;

fn get_distance<'a>(
    depth: usize,
    valve1: &'a str,
    valve2: &'a str,
    valve_map: &HashMap<&'a str, (isize, Vec<&'a str>)>,
    mut distance_memo: &mut HashMap<(&'a str, &'a str), isize>,
) -> Option<isize> {
    if valve1 == valve2 {
        return Some(0);
    } else if depth > 20 {
        return None;
    } else if let Some(distance) = distance_memo.get(&(valve1, valve2)) {
        return Some(*distance);
    } else if let Some(distance) = distance_memo.get(&(valve2, valve1)) {
        return Some(*distance);
    } else if valve_map[valve1].1.iter().contains(&valve2) {
        distance_memo.insert((valve1, valve2), 1);
        return Some(1);
    } else if let Some(distance) = valve_map[valve1]
        .1
        .iter()
        .filter_map(|valve| get_distance(depth + 1, valve, valve2, &valve_map, &mut distance_memo))
        .min()
    {
        let distance = distance.saturating_add(1);
        distance_memo.insert((valve1, valve2), distance);
        return Some(distance);
    } else {
        return None;
    }
}

fn calc_opportunity<'a>(
    current_valve: &'a str,
    valve_map: &HashMap<&'a str, (isize, Vec<&'a str>)>,
    open_valves: &Vec<&'a str>,
    available_time: isize,
    mut distance_memo: &mut HashMap<(&'a str, &'a str), isize>,
) -> isize {
    if open_valves.len() == 0 {
        return 0;
    }
    if available_time < 1 {
        return 0;
    }
    return open_valves
        .iter()
        .enumerate()
        .filter_map(|(i, target_valve)| {
            let distance = get_distance(0, current_valve, target_valve, &valve_map, &mut distance_memo);
            if distance.is_none() {
                return None;
            }
            let flow_rate = valve_map[target_valve].0;
            let distance = distance.unwrap();
            let mut open_valves = open_valves.clone();
            open_valves.remove(i);
            let time_after_open = available_time - distance - 1;
            if time_after_open < 0 {
                return None;
            }
            // println!("opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
            Some(
                calc_opportunity(
                    target_valve,
                    &valve_map,
                    &open_valves,
                    time_after_open,
                    &mut distance_memo,
                ) + flow_rate * time_after_open,
            )
        })
        .max()
        .unwrap_or(0);
}

pub fn main() {
    let s = include_str!("../input.txt");
    let valve_map = s
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.split_once("; ").unwrap())
        .map(|(a, b)| {
            let valve = &a["Valve ".len().."Valve ".len() + 2];
            let flow_rate: isize = a["Valve AA has flow rate=".len()..].parse().unwrap();
            let valves = match b.as_bytes()["tunnels".len() - 1] {
                b's' => b["tunnels lead to valves ".len()..]
                    .split(", ")
                    .collect::<Vec<_>>(),
                b' ' => b["tunnel leads to valve ".len()..]
                    .split(", ")
                    .collect::<Vec<_>>(),
                _ => unreachable!(),
            };
            (valve, (flow_rate, valves))
        })
        .collect::<HashMap<_, _>>();
    #[cfg(debug_assertions)]
    println!("{:?}", valve_map);

    let mut distance_memo = &mut HashMap::with_capacity(256);

    let open_valves = valve_map
        .iter()
        .filter(|(valve, _state)| valve_map[*valve].0 > 0)
        .map(|(key, _value)| *key)
        .sorted_unstable_by(|a, b| valve_map[*a].0.cmp(&valve_map[*b].0))
        .collect::<Vec<_>>();
    #[cfg(debug_assertions)]
    println!("{:?}", open_valves);
    // for target_valve in open_valves.iter() {
    //     println!("AA - {}: {}", target_valve, get_distance(0, "AA", target_valve, &valve_map, &mut distance_memo).unwrap());
    // }

    print!("{} ", calc_opportunity("AA", &valve_map, &open_valves, 30, &mut distance_memo));
}
