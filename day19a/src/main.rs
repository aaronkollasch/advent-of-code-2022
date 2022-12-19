use rustc_hash::{FxHashMap, FxHashSet};

type SimType = u32;
type CostVal = SimType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cost {
    ore: CostVal,
    clay: CostVal,
    obs: CostVal,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SimState {
    // time: SimType,
    ore: SimType,
    clay: SimType,
    obs: SimType,
    geo: SimType,
    rob_ore: SimType,
    rob_clay: SimType,
    rob_obs: SimType,
    rob_geo: SimType,
}

// fn sim_blueprint_recursive(state: SimState, costs: [Cost; 4], cache: &mut FxHashMap<SimState, SimState>) -> SimState {
//     if state.time == 4 {
//         println!("{} {:?}", state.time, state);
//     }
//     if let Some(s) = cache.get(&state) {
//         return *s;
//     }
//     let input_state = state;
//     if state.time == 23 {
//         let mut state = state;
//         state.ore += state.rob_ore;
//         state.clay += state.rob_clay;
//         state.obs += state.rob_obs;
//         state.geo += state.rob_geo;
//         state.time += 1;
//         cache.insert(input_state, state);
//         return state;
//     }
//     let result = (0..=costs.len()).filter_map(|i| {
//         // continue sim if this robot was purchased
//         let mut state = state;
//         if i < costs.len() {
//             let cost = costs[i];
//             if cost.ore <= state.ore && cost.clay <= state.clay && cost.obs <= state.obs {
//                 state.ore -= cost.ore;
//                 state.clay -= cost.clay;
//                 state.obs -= cost.obs;
//                 state.ore += state.rob_ore;
//                 state.clay += state.rob_clay;
//                 state.obs += state.rob_obs;
//                 state.geo += state.rob_geo;
//                 match i {
//                     0 => { state.rob_ore += 1; },
//                     1 => { state.rob_clay += 1; },
//                     2 => { state.rob_obs += 1; },
//                     3 => { state.rob_geo += 1; },
//                     _ => unreachable!(),
//                 }
//                 state.time += 1;
//                 Some(sim_blueprint_recursive(state, costs, cache))
//             } else {
//                 None
//             }
//         } else {
//             state.ore += state.rob_ore;
//             state.clay += state.rob_clay;
//             state.obs += state.rob_obs;
//             state.geo += state.rob_geo;
//             state.time += 1;
//             Some(sim_blueprint_recursive(state, costs, cache))
//         }
//     }).max_by(|a, b| a.rob_geo.cmp(&b.rob_geo)).unwrap();
//     cache.insert(input_state, result);
//     result
// }
//
fn sim_blueprint(init_state: SimState, minutes: usize, costs: [Cost; 4]) -> SimState {
    let mut prev_states: FxHashSet<SimState> = Default::default();
    prev_states.insert(init_state);
    let mut new_states: FxHashSet<SimState> = Default::default();
    for min in 1..minutes {
        println!("min: {}, {}", min, prev_states.len());
        for state in prev_states.drain() {
            for i in 0..=costs.len() {
                // continue sim as if robot i was purchased
                let mut state = state;
                if i < costs.len() {
                    let cost = costs[i];
                    if state.ore >= cost.ore && state.clay >= cost.clay && state.obs >= cost.obs {
                        state.ore -= cost.ore;
                        state.clay -= cost.clay;
                        state.obs -= cost.obs;
                        state.ore += state.rob_ore;
                        state.clay += state.rob_clay;
                        state.obs += state.rob_obs;
                        state.geo += state.rob_geo;
                        match i {
                            0 => {
                                state.rob_ore += 1;
                            }
                            1 => {
                                state.rob_clay += 1;
                            }
                            2 => {
                                state.rob_obs += 1;
                            }
                            3 => {
                                state.rob_geo += 1;
                            }
                            _ => unreachable!(),
                        }
                        new_states.insert(state);
                    }
                } else {
                    state.ore += state.rob_ore;
                    state.clay += state.rob_clay;
                    state.obs += state.rob_obs;
                    state.geo += state.rob_geo;
                    new_states.insert(state);
                }
            }
        }
        (new_states, prev_states) = (prev_states, new_states);
        // for state in prev_states.iter() {
        //     if min == 5 && *state == (SimState {
        //         ore: 1,
        //         clay: 2,
        //         obs: 0,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 2,
        //         rob_obs: 0,
        //         rob_geo: 0,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 8 && *state == (SimState {
        //         ore: 2,
        //         clay: 9,
        //         obs: 0,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 3,
        //         rob_obs: 0,
        //         rob_geo: 0,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 10 && *state == (SimState {
        //         ore: 4,
        //         clay: 15,
        //         obs: 0,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 3,
        //         rob_obs: 0,
        //         rob_geo: 0,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 12 && *state == (SimState {
        //         ore: 1,
        //         clay: 7,
        //         obs: 1,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 4,
        //         rob_obs: 1,
        //         rob_geo: 0,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 13 && *state == (SimState {
        //         ore: 2,
        //         clay: 11,
        //         obs: 2,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 4,
        //         rob_obs: 1,
        //         rob_geo: 0,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 18 && *state == (SimState {
        //         ore: 2,
        //         clay: 17,
        //         obs: 3,
        //         geo: 0,
        //         rob_ore: 1,
        //         rob_clay: 4,
        //         rob_obs: 2,
        //         rob_geo: 1,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        //     if min == 23 && *state == (SimState {
        //         ore: 5,
        //         clay: 37,
        //         obs: 6,
        //         geo: 7,
        //         rob_ore: 1,
        //         rob_clay: 4,
        //         rob_obs: 2,
        //         rob_geo: 2,
        //     }) {
        //         println!("    {:?}", state);
        //     }
        // }
        println!(
            "min: {}, {:?}",
            min,
            prev_states
                .iter()
                .max_by(|a, b| a.rob_geo.cmp(&b.rob_geo))
                .unwrap()
        );
    }
    println!("min: {}, {}", minutes, prev_states.len());
    prev_states
        .drain()
        .map(|mut state| {
            state.ore += state.rob_ore;
            state.clay += state.rob_clay;
            state.obs += state.rob_obs;
            state.geo += state.rob_geo;
            // if state == (SimState {
            //     ore: 6,
            //     clay: 41,
            //     obs: 8,
            //     geo: 9,
            //     rob_ore: 1,
            //     rob_clay: 4,
            //     rob_obs: 2,
            //     rob_geo: 2,
            // }) {
            //     println!("    {:?}", state);
            // }
            state
        })
        .max_by(|a, b| a.geo.cmp(&b.geo))
        .unwrap()
}

pub fn main() {
    let blueprints = include_str!("../input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut words = l.split(' ');
            let id = words
                .nth(1)
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<CostVal>()
                .unwrap();
            let ore_cost = words.nth(4).unwrap().parse::<CostVal>().unwrap();
            let clay_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let obs_ore_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let obs_clay_cost = words.nth(2).unwrap().parse::<CostVal>().unwrap();
            let geode_ore_cost = words.nth(5).unwrap().parse::<CostVal>().unwrap();
            let geode_obs_cost = words.nth(2).unwrap().parse::<CostVal>().unwrap();

            (
                id,
                [
                    Cost {
                        ore: ore_cost,
                        clay: 0,
                        obs: 0,
                    },
                    Cost {
                        ore: clay_cost,
                        clay: 0,
                        obs: 0,
                    },
                    Cost {
                        ore: obs_ore_cost,
                        clay: obs_clay_cost,
                        obs: 0,
                    },
                    Cost {
                        ore: geode_ore_cost,
                        clay: 0,
                        obs: geode_obs_cost,
                    },
                ],
            )
        })
        .collect::<Vec<_>>();
    for blueprint in blueprints.iter() {
        println!("{:?}", blueprint);
    }

    let state = SimState {
        // time: 0,
        ore: 0,
        clay: 0,
        obs: 0,
        geo: 0,
        rob_ore: 1,
        rob_clay: 0,
        rob_obs: 0,
        rob_geo: 0,
    };
    let results = blueprints
        .into_iter()
        .map(|(i, costs)| {
            println!("testing blueprint {}", i);
            // let mut cache: FxHashMap<SimState, SimState> = Default::default();
            // cache.reserve(1024);
            // let state = sim_blueprint_recursive(state, costs, &mut cache);
            let state = sim_blueprint(state, 24, costs);
            println!("{:?}", state);
            println!("blueprint {} had at most {} geodes", i, state.geo);
            // println!("cache len: {}", cache.len());
            (i, state.geo)
        })
        .collect::<Vec<_>>();
    let result = results.iter().map(|(i, geo)| i * geo).sum::<SimType>();
    print!("{} ", result);
}
