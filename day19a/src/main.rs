use priority_queue::DoublePriorityQueue;
use rayon::prelude::*;
use std::cmp::Ordering;

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
    ore: SimType,
    clay: SimType,
    obs: SimType,
    geo: SimType,
    rob_ore: SimType,
    rob_clay: SimType,
    rob_obs: SimType,
    rob_geo: SimType,
}

impl SimState {
    #[inline]
    fn priority(&self) -> SimType {
        self.geo.wrapping_shl(24)
            + self.rob_geo.wrapping_shl(24)
            + self.obs
            + self.rob_obs
            + self.rob_clay
    }

    #[inline]
    fn can_buy(&self, cost: Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obs >= cost.obs
    }

    #[inline]
    fn buy(&mut self, cost: Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obs -= cost.obs;
    }

    #[inline]
    fn step(&mut self) {
        self.ore += self.rob_ore;
        self.clay += self.rob_clay;
        self.obs += self.rob_obs;
        self.geo += self.rob_geo;
    }
}

impl Ord for SimState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for SimState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const QUEUE_SIZE: usize = 2 << 4; // if wrong answer found, try a larger QUEUE_SIZE

fn sim_blueprint(init_state: SimState, minutes: usize, costs: [Cost; 4]) -> SimState {
    let mut prev_states = DoublePriorityQueue::<SimState, SimType>::with_capacity(QUEUE_SIZE);
    prev_states.push(init_state, init_state.priority());
    let mut new_states = DoublePriorityQueue::<SimState, SimType>::with_capacity(QUEUE_SIZE);
    for _min in 1..minutes {
        #[cfg(debug_assertions)]
        println!("min: {}, {}", _min, prev_states.len());
        for (state, _priority) in prev_states.iter() {
            for i in 0..=costs.len() {
                // continue sim as if robot i was purchased
                let mut state = *state;
                if i < costs.len() {
                    let cost = costs[i];
                    if state.can_buy(cost) {
                        state.buy(cost);
                        state.step();
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
                        new_states.push(state, state.priority());
                    }
                } else {
                    state.step();
                    new_states.push(state, state.priority());
                }
                if new_states.len() >= QUEUE_SIZE {
                    new_states.pop_min();
                }
            }
        }
        prev_states.clear();
        (new_states, prev_states) = (prev_states, new_states);
        #[cfg(debug_assertions)]
        for (state, priority) in prev_states.iter() {
            if _min == 1
                && *state
                    == (SimState {
                        ore: 1,
                        clay: 0,
                        obs: 0,
                        geo: 0,
                        rob_ore: 1,
                        rob_clay: 0,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 4
                && *state
                    == (SimState {
                        ore: 4,
                        clay: 0,
                        obs: 0,
                        geo: 0,
                        rob_ore: 1,
                        rob_clay: 0,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 8
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 1,
                        obs: 0,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 2,
                        rob_obs: 0,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 16
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 14,
                        obs: 2,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 2,
                        rob_geo: 0,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 20
                && *state
                    == (SimState {
                        ore: 3,
                        clay: 14,
                        obs: 7,
                        geo: 0,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 4,
                        rob_geo: 1,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
            if _min == 24
                && *state
                    == (SimState {
                        ore: 2,
                        clay: 28,
                        obs: 5,
                        geo: 7,
                        rob_ore: 2,
                        rob_clay: 7,
                        rob_obs: 5,
                        rob_geo: 4,
                    })
            {
                println!("    {}: {:?}", priority, state);
            }
        }
    }
    #[cfg(debug_assertions)]
    println!("min: {}, {}", minutes, prev_states.len());
    prev_states
        .into_sorted_iter()
        .rev()
        .map(|(mut state, _priority)| {
            state.step();
            #[cfg(debug_assertions)]
            if state
                == (SimState {
                    ore: 6,
                    clay: 41,
                    obs: 8,
                    geo: 9,
                    rob_ore: 1,
                    rob_clay: 4,
                    rob_obs: 2,
                    rob_geo: 2,
                })
            {
                println!("    {:?}", state);
            }
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
        .par_iter()
        .map(|(i, costs)| {
            let state = sim_blueprint(state, 24, *costs);
            #[cfg(debug_assertions)]
            {
                println!("{:?}", state);
                println!("blueprint {} had at most {} geodes", i, state.geo);
            }
            (*i, state.geo)
        })
        .collect::<Vec<_>>();
    let result = results.iter().map(|(i, geo)| i * geo).sum::<SimType>();
    print!("{} ", result);
}
