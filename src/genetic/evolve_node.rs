use crate::config::{self, Config, mutation::Mutation};

use super::build_node::Node;

const CONFIG_MUTATION: Mutation = Mutation::default();

pub fn evolve_node(node: Node) -> () {
	let stats = node.stats;
	let mut evolve_stats = vec![];

//TODO: ~~ ?
	let evolve_count = node.stats.len() * configMutation.evolveStatPercentage;

	for i in 0..evolve_count {
		let pick_stat = None;
		while (pickStat.is_none() || evolve_stats.some(|f| f.stat == pickStat.stat) || pick_stat.r#static) {
			//TODO: ~~ ?
			pickStat = stats[(Math.random() * stats.len())];
		}

		evolve_stats.push(pickStat);
	}

	evolve_stats.forEach(|s| {
		let delta = (Math.random() * configMutation.statMutateMaxMult * 2) - configMutation.statMutateMaxMult;
		s.value += delta;
		if (s.value < s.min)
			s.value = s.min;
	});
};

module.exports = evolveNode;
