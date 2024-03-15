use crate::config::mutation::Mutation;
use rand::Rng;

use super::build_node::Node;

const CONFIG_MUTATION: Mutation = Mutation::default();

pub fn evolve_node(node: Node) {
    let stats = &node.stats;
    let mut evolve_stats = vec![];

    let evolve_count =
        ((node.stats.len() as f32) * CONFIG_MUTATION.evolve_stat_percentage).floor() as usize;

    let mut rng = rand::thread_rng();
    for _ in 0..evolve_count {
        let pick_stat = loop {
            let random_stat = &stats[rng.gen_range(0..stats.len())];
            if !evolve_stats.contains(random_stat) {
                break random_stat;
            }
        };

        evolve_stats.push(*pick_stat);
    }

    evolve_stats.iter_mut().for_each(|s| {
        let delta = (rng.gen::<f32>() * CONFIG_MUTATION.stat_mutate_max_mult * 2.0)
            - CONFIG_MUTATION.stat_mutate_max_mult;
        s.value += delta;
        if let Some(min) = s.min {
            if s.value < min {
                s.value = min;
            }
        }
    });
}
