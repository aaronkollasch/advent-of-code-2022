use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::cmp::Ordering;
// use std::thread;
// use std::sync::Arc;

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
    my_valve: &'a str,
    el_valve: &'a str,
    valve_map: &HashMap<&'a str, (isize, Vec<&'a str>)>,
    open_valves: &Vec<&'a str>,
    my_available_time: isize,
    el_available_time: isize,
    mut distance_memo: &mut HashMap<(&'a str, &'a str), isize>,
) -> isize {
    if my_available_time >= 20 && el_available_time >= 20 {
        println!("{} {} {} {}", my_valve, el_valve, my_available_time, el_available_time);
    }
    if open_valves.len() == 0 {
        return 0;
    }
    if el_available_time < 1 && my_available_time < 1 {
        return 0;
    }
    return open_valves
        .iter()
        .enumerate()
        .filter_map(|(i, target_valve)| {
            let my_distance =
                get_distance(0, my_valve, target_valve, &valve_map, &mut distance_memo);
            let el_distance =
                get_distance(0, el_valve, target_valve, &valve_map, &mut distance_memo);
            let mut my_job;
            if my_distance.is_none() && el_distance.is_none() {
                return None;
            } else if my_distance.is_none() {
                my_job = false;
            } else if el_distance.is_none() {
                my_job = true;
            } else {
                let my_distance = my_distance.unwrap();
                let el_distance = el_distance.unwrap();
                my_job = match (my_available_time - my_distance).cmp(&(el_available_time - el_distance)) {
                    Ordering::Greater => true,
                    Ordering::Equal => true,
                    Ordering::Less => false,
                }
            }
            let mut result = isize::MIN;
            let flow_rate = valve_map[target_valve].0;
            let mut open_valves = open_valves.clone();
            open_valves.remove(i);
            if my_job {
                let distance = my_distance.unwrap();
                let time_after_open = my_available_time - distance - 1;
                // println!("my opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
                if time_after_open >= 0 {
                    result = max(
                        result,
                        calc_opportunity(
                            target_valve,
                            el_valve,
                            &valve_map,
                            &open_valves,
                            time_after_open,
                            el_available_time,
                            &mut distance_memo,
                        ) + flow_rate * time_after_open,
                    );
                }
            } else {
                let distance = el_distance.unwrap();
                let time_after_open = el_available_time - distance - 1;
                // println!("el opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
                if time_after_open >= 0 {
                    result = max(
                        result,
                        calc_opportunity(
                            my_valve,
                            target_valve,
                            &valve_map,
                            &open_valves,
                            my_available_time,
                            time_after_open,
                            &mut distance_memo,
                        ) + flow_rate * time_after_open,
                    );
                }
            }
            if result < 0 {
                None
            } else {
                Some(result)
            }
        })
        .max()
        .unwrap_or(0);
}

// fn calc_opportunity_par<'a>(
//     my_valve: Arc<&str>,
//     el_valve: Arc<&str>,
//     valve_map: Arc<HashMap<&'a str, (isize, Vec<&'a str>)>>,
//     open_valves: Arc<Vec<&'a str>>,
//     my_available_time: isize,
//     el_available_time: isize,
// ) -> isize {
//     if my_available_time >= 20 && el_available_time >= 20 {
//         println!("{} {} {} {}", my_valve, el_valve, my_available_time, el_available_time);
//     }
//     if open_valves.len() == 0 {
//         return 0;
//     }
//     if el_available_time < 1 && my_available_time < 1 {
//         return 0;
//     }
//     let handles = open_valves
//         .iter()
//         .enumerate()
//         .map(|(i, target_valve)| {
//             let my_valve = my_valve.clone();
//             let el_valve = el_valve.clone();
//             let valve_map = valve_map.clone();
//             let open_valves = open_valves.clone();
//             thread::spawn({

//                 move || {
//                     let mut distance_memo = &mut HashMap::with_capacity(256);

//                     let my_distance =
//                         get_distance(0, &my_valve, &target_valve, &valve_map, &mut distance_memo);
//                     let el_distance =
//                         get_distance(0, &el_valve, &target_valve, &valve_map, &mut distance_memo);
//                     if my_distance.is_none() && el_distance.is_none() {
//                         return None;
//                     }
//                     let mut result = isize::MIN;
//                     let flow_rate = valve_map[target_valve].0;
//                     let mut open_valves = (*open_valves).clone();
//                     open_valves.remove(i);
//                     if let Some(distance) = my_distance {
//                         let time_after_open = my_available_time - distance - 1;
//                         // println!("my opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
//                         if time_after_open >= 0 {
//                             result = max(
//                                 result,
//                                 calc_opportunity(
//                                     target_valve,
//                                     &el_valve,
//                                     &valve_map,
//                                     &open_valves,
//                                     time_after_open,
//                                     el_available_time,
//                                     &mut distance_memo,
//                                 ) + flow_rate * time_after_open,
//                             );
//                         }
//                     }
//                     if let Some(distance) = el_distance {
//                         let time_after_open = el_available_time - distance - 1;
//                         // println!("el opening {} at {}, {}", target_valve, 31 - time_after_open, flow_rate * time_after_open);
//                         if time_after_open >= 0 {
//                             result = max(
//                                 result,
//                                 calc_opportunity(
//                                     &my_valve,
//                                     target_valve,
//                                     &valve_map,
//                                     &open_valves,
//                                     my_available_time,
//                                     time_after_open,
//                                     &mut distance_memo,
//                                 ) + flow_rate * time_after_open,
//                             );
//                         }
//                     }
//                     if result < 0 {
//                         None
//                     } else {
//                         Some(result)
//                     }
//                 }
//             })
//         }).collect::<Vec<_>>();
//     handles.into_iter()
//         .filter_map(|handle| handle.join().unwrap())
//         .max()
//         .unwrap_or(0)
// }

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
            println!("{}; {}", a, b);
            (valve, (flow_rate, valves))
        })
        .collect::<HashMap<_, _>>();
    println!("{:?}", valve_map);

    let mut distance_memo = &mut HashMap::with_capacity(256);

    let open_valves = valve_map
        .iter()
        .filter(|(valve, _state)| valve_map[*valve].0 > 0)
        .map(|(key, _value)| *key)
        .sorted_unstable_by(|a, b| valve_map[*a].0.cmp(&valve_map[*b].0))
        .collect::<Vec<_>>();
    println!("{:?}", open_valves);
    // for target_valve in open_valves.iter() {
    //     println!(
    //         "AA - {}: {}",
    //         target_valve,
    //         get_distance(0, "AA", target_valve, &valve_map, &mut distance_memo).unwrap()
    //     );
    // }

    print!(
        "{} ",
        calc_opportunity(
            "AA",
            "AA",
            &valve_map,
            &open_valves,
            26,
            26,
            &mut distance_memo,
        )
    );
    // print!(
    //     "{} ",
    //     calc_opportunity_par(
    //         Arc::new("AA"),
    //         Arc::new("AA"),
    //         Arc::new(valve_map),
    //         Arc::new(open_valves),
    //         26,
    //         26,
    //     )
    // );
}
