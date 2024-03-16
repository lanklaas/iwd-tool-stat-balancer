// const config = require('../../config');
// const configStats = require('../../config/stats');
// const setNodeFitness = require('./../setNodeFitness');
// const evolveNode = require('./../evolveNode');
// const logCharacterStats = require('./../logCharacterStats');

// let node = null;

use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
};

use crate::genetic::set_node_fitness::CONFIG;
use serde_json::Value;
use tracing::debug;

#[derive(Debug)]
pub struct Stats(pub Value);
impl Deref for Stats {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Stats {
    pub fn get(
        &self,
        stat_name: &str,
        level: Option<usize>,
        tier: Option<&str>,
        role: Option<&str>,
    ) -> f64 {
        let node = &self.0.as_array().unwrap();
        let found_stat = node
            .iter()
            .find(|x| x["stat"] == stat_name)
            .expect("Should be there");

        let mut res = found_stat["value"].as_f64().unwrap();

        if found_stat["noScale"]
            .as_bool()
            .is_some_and(|no_scale| !no_scale)
            && level.is_some()
        {
            let level = level.unwrap() as f64;
            res *= level
                * node
                    .iter()
                    .map(|x| x.as_object().unwrap())
                    .find(|x| x["stat"] == "statMultPerLevel")
                    .expect("statMultPerLevel to exist")["value"]
                    .as_f64()
                    .unwrap();
        }

        match (found_stat["roles"].as_object(), tier) {
            (Some(_), Some(tier)) => {
                let pieces_of_gear = CONFIG["piecesOfGear"].as_f64().unwrap();
                let useful_stats_per_gear_piece =
                    CONFIG["usefulStatsPerGearPiece"].as_object().unwrap()[tier]
                        .as_f64()
                        .unwrap();
                let role_count: f64 = node
                    .iter()
                    .filter(|s| {
                        let roles = s["roles"].as_object().unwrap();
                        roles[role.unwrap_or("")].as_f64().is_some()
                    })
                    .count() as f64;
                let roll_perfection_of_stats = CONFIG["rollPerfectionOfStats"].as_object().unwrap()
                    [tier]
                    .as_f64()
                    .unwrap();
                //Division by 0
                let role_count = if role_count == 0. { 1.0 } else { role_count };
                res *= pieces_of_gear
                    * (useful_stats_per_gear_piece / role_count)
                    * roll_perfection_of_stats;
            }
            other => {
                debug!("Stats get other condition: {other:?}");
            }
        }

        if let Some(max) = found_stat["max"].as_f64().borrow_mut() {
            if res > *max {
                res = *max;
            }
        }

        res
    }
    pub fn get_mob(&self, stat_name: &str, level: Option<usize>) -> f64 {
        let node = &self.0;
        let found_stat = self
            .0
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["stat"].as_str() == Some(stat_name))
            .expect("Should be there");

        let mut res = found_stat["value"].as_f64().unwrap();
        let no_scale = found_stat["noScale"].as_bool().unwrap_or_default();
        if !no_scale && level.is_some() {
            res *= level.unwrap() as f64
                * node
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|x| x["stat"].as_str() == Some("statMultPerLevel"))
                    .expect("statMultPerLevel to exist")
                    .as_object()
                    .unwrap()["value"]
                    .as_f64()
                    .unwrap();
        }

        res
    }
    pub fn get_unscaled(&self, stat_name: &str) -> f64 {
        self.get(stat_name, None, None, None)
    }
}

// const setStatsHelpers = () => {
// 	node.stats.get = (statName, level, tier, role) => {
// 		const foundStat = node.stats.find(s => s.stat === statName);

// 		let res = foundStat.value;

// 		if (!foundStat.noScale)
// 			res *= level * node.stats.get('statMultPerLevel');

// 		if (foundStat.roles) {
// 			res *= (
// 				config.piecesOfGear *
// 				(
// 					config.usefulStatsPerGearPiece[tier] /
// 					configStats.filter(s => s.roles?.[role]).length
// 				) *
// 				config.rollPerfectionOfStats[tier]
// 			);
// 		}

// 		if (foundStat.max && res > foundStat.max)
// 			res = foundStat.max;

// 		return res;
// 	};

// 	node.stats.getMob = (statName, level) => {
// 		const foundStat = node.stats.find(s => s.stat === statName);

// 		let res = foundStat.value;
// 		if (!foundStat.noScale) {
// 			res *= (
// 				level *
// 				node.stats.get('statMultPerLevel')
// 			);
// 		}

// 		return res;
// 	};
// };

// const handlers = {
// 	init: function () {
// 		node = JSON.parse(process.argv[2]);

// 		handlers.initStats();

// 		setStatsHelpers();
// 	},

// 	initStats: function () {
// 		node.stats.length = 0;

// 		configStats.forEach(({ initial, disabled, min, ...rest }) => {
// 			if (disabled)
// 				return;

// 			node.stats.push({
// 				...rest,
// 				min: min ?? 0,
// 				value: initial ?? (min ?? 0) + (Math.random() * 100)
// 			});
// 		});
// 	},

// 	simulate: () => {
// 		node.age++;

// 		setNodeFitness(node);

// 		return {
// 			set: [{
// 				key: 'fitness',
// 				value: node.fitness
// 			}]
// 		};
// 	},

// 	syncStats: () => {
// 		return {
// 			set: [{
// 				key: 'stats',
// 				value: node.stats
// 			}]
// 		};
// 	},

// 	syncFitnessResult: () => {
// 		return {
// 			set: [{
// 				key: 'fitnessResult',
// 				value: node.fitnessResult
// 			}]
// 		};
// 	},

// 	setStats: newStats => {
// 		node.age = 0;
// 		node.stats = newStats;

// 		setStatsHelpers();
// 	},

// 	evolveNode: () => evolveNode(node),

// 	logCharacterStats: ({ level, tier, role }) => logCharacterStats(node, { level, tier, role })
// };

// process.on('message', m => {
// 	const result = handlers[m.type](m.data);

// 	process.send({
// 		type: m.type,
// 		result
// 	});
// });
