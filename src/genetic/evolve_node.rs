use std::borrow::BorrowMut;

// use crate::config::mutation::Mutation;
use lazy_static::lazy_static;
use rand::Rng;
use serde_json::Value;

use super::build_node::Node;

const CONFIG_MUTATION_JSON: &str = include_str!("../config/mutation.json");
lazy_static! {
    static ref CONFIG_MUTATION: Value = serde_json::from_str(CONFIG_MUTATION_JSON).unwrap();
}

pub fn evolve_node(node: Node) {
    let stats = node.stats;
    let mut evolve_stats: Vec<&'static _> = vec![];
    let stats_len = stats.as_array().unwrap().len();
    let evolve_count = ((stats_len as f64)
        * CONFIG_MUTATION["evolveStatPercentage"].as_f64().unwrap())
    .floor() as usize;

    let mut rng = rand::thread_rng();
    for _ in 0..evolve_count {
        let pick_stat: &'static _ = loop {
            let random_stat = &stats[rng.gen_range(0..stats_len)];
            if !evolve_stats.contains(&random_stat) {
                break random_stat;
            }
        };

        evolve_stats.push(pick_stat);
    }

    evolve_stats.iter_mut().for_each(|s| {
        let delta =
            (rng.gen::<f64>() * CONFIG_MUTATION["statMutateMaxMult"].as_f64().unwrap() * 2.0)
                - CONFIG_MUTATION["statMutateMaxMult"].as_f64().unwrap();
        let mut binding = s["value"].as_f64().unwrap();
        let value = binding.borrow_mut();
        *value += delta;
        if let Some(min) = s["min"].as_f64() {
            if *value < min {
                *value = min;
            }
        }
    });
}
