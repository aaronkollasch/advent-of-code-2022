use itertools::Itertools;
use rustc_hash::FxHashMap;

const NUM_LINES: usize = 60;

fn get_distance<'a>(
    depth: usize,
    valve1: usize,
    valve2: usize,
    valve_map: &FxHashMap<usize, Vec<usize>>,
    mut distance_memo: &mut FxHashMap<(usize, usize), isize>,
) -> Option<isize> {
    if valve1 == valve2 {
        return Some(0);
    } else if depth > 20 {
        return None;
    } else if let Some(distance) = distance_memo.get(&if valve1 < valve2 { (valve1, valve2) } else { (valve2, valve1) }) {
        return Some(*distance);
    } else if valve_map[&valve1].iter().contains(&valve2) {
        distance_memo.insert(if valve1 < valve2 { (valve1, valve2) } else { (valve2, valve1) }, 1);
        return Some(1);
    } else if let Some(distance) = valve_map[&valve1]
        .iter()
        .filter_map(|valve| get_distance(depth + 1, *valve, valve2, &valve_map, &mut distance_memo))
        .min()
    {
        let distance = distance.saturating_add(1);
        distance_memo.insert(if valve1 < valve2 { (valve1, valve2) } else { (valve2, valve1) }, distance);
        return Some(distance);
    } else {
        return None;
    }
}

fn calc_opportunity<'a>(
    current_valve: usize,
    valve_map: &FxHashMap<usize, Vec<usize>>,
    flow_rates: [isize; NUM_LINES],
    valve_values: [isize; NUM_LINES],
    available_time: isize,
    mut distance_memo: &mut FxHashMap<(usize, usize), isize>,
) -> (isize, Option<[isize; NUM_LINES]>) {
    #[cfg(debug_assertions)]
    if available_time >= 20 {
        println!("{} {}", current_valve, available_time);
    }
    if available_time < 1 {
        return (0, Some(valve_values));
    }
    let result = valve_values
        .iter()
        .enumerate()
        .filter(|(target_valve, _is_open)| flow_rates[*target_valve] > 0)
        .filter_map(|(target_valve, target_cur_value)| {
            let distance = get_distance(
                0,
                current_valve,
                target_valve,
                &valve_map,
                &mut distance_memo,
            );
            if distance.is_none() {
                return None;
            }
            let distance = distance.unwrap();
            let time_after_open = available_time - distance - 1;
            if time_after_open < 0 {
                return None;
            }
            let flow_rate = flow_rates[target_valve];
            let valve_value = flow_rate * time_after_open;
            if valve_value <= *target_cur_value {
                return None;
            }
            let mut valve_values = valve_values.to_owned();
            valve_values[target_valve] = valve_value;
            // #[cfg(debug_assertions)]
            // println!("opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
            let mut result = calc_opportunity(
                target_valve,
                &valve_map,
                flow_rates,
                valve_values,
                time_after_open,
                &mut distance_memo,
            );
            result.0 += valve_value - *target_cur_value;
            result.1 = result.1.to_owned();
            Some(result)
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((0, Some(valve_values)));
    result
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
            (valve, flow_rate, valves)
        })
        .collect::<Vec<_>>();
    let valve_indices = valve_map
        .to_owned()
        .into_iter()
        .sorted_unstable_by(|a, b| b.1.cmp(&a.1))
        .enumerate()
        .map(|(i, (valve, _, _))| (valve, i))
        .collect::<FxHashMap<_, _>>();
    #[cfg(debug_assertions)]
    println!("{:?}", valve_indices);

    let flow_rates = valve_map
        .iter()
        .sorted_unstable_by(|a, b| valve_indices[a.0].cmp(&valve_indices[b.0]))
        .map(|(_valve, flow_rate, _valves)| *flow_rate)
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    #[cfg(debug_assertions)]
    println!("{:?}", flow_rates);

    let valve_map = valve_map
        .into_iter()
        .map(|(valve, _flow_rate, valves)| {
            (
                *valve_indices.get(&valve).unwrap(),
                valves
                    .into_iter()
                    .map(|v| *valve_indices.get(&v).unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<FxHashMap<usize, Vec<usize>>>();
    #[cfg(debug_assertions)]
    println!("{:?}", valve_map);

    let mut distance_memo: FxHashMap<(usize, usize), isize> = Default::default();
    distance_memo.reserve(1024);
    let valve_values: [isize; NUM_LINES] = [0; NUM_LINES];

    let result1 = calc_opportunity(valve_indices["AA"], &valve_map, flow_rates, valve_values, 30, &mut distance_memo);
    #[cfg(debug_assertions)]
    {
        println!("memoization map size: {}", distance_memo.len());
        println!("{}\t{:?}", result1.0, result1.1);
    }
    print!(
        "{} ", result1.0
    );
}
