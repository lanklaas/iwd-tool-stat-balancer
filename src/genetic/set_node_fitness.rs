// use crate::config::{
//     self,
//     requirements::{Requirements, Role, Requirement},
//     stats::Stats,
//     Config,
// };

use super::{build_node::Node, node::Stats};
use lazy_static::lazy_static;
use serde_json::{json, Value};

const LEVEL_MIN: usize = 25;
const LEVEL_MAX: usize = 25;

//Imports
lazy_static! {
    pub static ref CONFIG: Value =
        serde_json::from_str(include_str!("../config/index.json")).unwrap();
    static ref TICKS_TAKEN_TO_DEAL_DMG_BASE: f64 =
        CONFIG["ticksTakenToDealDmgBase"].as_f64().unwrap();
    static ref TICKS_PER_SECOND: f64 = CONFIG["ticksPerSecond"].as_f64().unwrap();
    // static ref LEVEL_MIN: usize = CONFIG["levelMin"].as_i64().unwrap().try_into().unwrap();
    // static ref LEVEL_MAX: usize = CONFIG["levelMax"].as_i64().unwrap().try_into().unwrap();
    static ref TIERS: Vec<&'static str> = CONFIG["tiers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect();
    static ref ROLES: Vec<&'static str> = CONFIG["roles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect();
    static ref CONFIG_REQUIREMENTS: Value =
        serde_json::from_str(include_str!("../config/requirements.json")).unwrap();
    pub static ref STATS: Stats =
        Stats(serde_json::from_str(include_str!("../config/stats.json")).unwrap());
}

//Getters

//Dps
pub fn get_player_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let mut dmg = stats.get("playerDmgBase", level, tier, role);
    let main_stat = stats.get("mainStat", level, tier, role);
    dmg *= if main_stat < 1.0 { 1.0 } else { main_stat };

    let dmg_percent = 100.0 + stats.get("globalDmgPercent", level, tier, role);

    let crit_chance = stats.get_unscaled("baseCritChance")
        + (stats.get("globalCritChance", level, tier, role) * 0.05)
        + (stats.get("attackSpellCritChance", level, tier, role) * 0.05);

    let crit_multiplier = stats.get_unscaled("baseCritMultiplier")
        + stats.get("globalCritMultiplier", level, tier, role)
        + stats.get("attackSpellCritMultiplier", level, tier, role);

    dmg *= dmg_percent / 100.0;

    dmg = (dmg * (crit_chance / 100.0) * (crit_multiplier / 100.0))
        + (dmg * (1.0 - (crit_chance / 100.0)));

    (dmg / *TICKS_TAKEN_TO_DEAL_DMG_BASE) * *TICKS_PER_SECOND
}

pub fn get_mob_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let mut dmg = stats.get("mobDmgBase", level, tier, role);

    let mob_dmg_mult = stats.get_mob("mobDmgMult", level);
    dmg *= mob_dmg_mult;

    let armor_multiplier = {
        let armor_stat = stats.get("armor", level, tier, role);
        let max_armor = f64::max(
            0.5,
            0.5 + f64::max(
                (1.0 - (armor_stat / (level.unwrap() as f64 * 50.0))) / 2.0,
                -0.5,
            ),
        );
        max_armor * 1.0
    };

    dmg *= armor_multiplier;

    let avoid_chance = stats.get("avoidChance", level, tier, role);

    dmg *= 1.0 - (avoid_chance / 100.0);

    (dmg / *TICKS_TAKEN_TO_DEAL_DMG_BASE) * *TICKS_PER_SECOND
}

pub fn get_boss_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let mut dmg = stats.get("bossDmgBase", level, tier, role);

    let mob_dmg_mult = stats.get_mob("bossDmgMult", level);
    dmg *= mob_dmg_mult;

    let armor_multiplier = {
        let armor_stat = stats.get("armor", level, tier, role);
        let max_armor = f64::max(
            0.5,
            0.5 + f64::max(
                (1.0 - (armor_stat / (level.unwrap() as f64 * 50.0))) / 2.0,
                -0.5,
            ),
        );
        max_armor * 1.0
    };

    dmg *= armor_multiplier;

    let avoid_chance = stats.get("avoidChance", level, tier, role);

    dmg *= 1.0 - (avoid_chance / 100.0);

    (dmg / *TICKS_TAKEN_TO_DEAL_DMG_BASE) * *TICKS_PER_SECOND
}

// //Hp
pub fn get_player_hp(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let mut hp = stats.get_unscaled("playerHpBase");

    hp += stats.get("vit", level, tier, role) * stats.get("vitToHpMultiplier", level, tier, role);

    hp
}

pub fn get_mob_hp(stats: &Stats, level: Option<usize>) -> f64 {
    let mut hp = stats.get_unscaled("mobHpBase");

    hp *= stats.get_mob("mobHpMult", level);

    hp
}

pub fn get_boss_hp(stats: &Stats, level: Option<usize>) -> f64 {
    let mut hp = stats.get_unscaled("bossHpBase");

    hp *= stats.get_mob("bossHpMult", level);

    hp
}

// //Calculations
pub fn get_seconds_to_kill_mob(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let dps = get_player_dps(stats, level, tier, role);
    let hp = get_mob_hp(stats, level);

    hp / dps
}

pub fn get_seconds_to_kill_boss(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let dps = get_player_dps(stats, level, tier, role);
    let hp = get_boss_hp(stats, level);

    hp / dps
}

pub fn get_seconds_to_be_killed_by_mob(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let dps = get_mob_dps(stats, level, tier, role);
    let hp = get_player_hp(stats, level, tier, role);

    let regen_per_sec = stats.get("regenHp", level, tier, role) * *TICKS_PER_SECOND;

    hp / f64::max(0.000001, dps - regen_per_sec)
}

pub fn get_seconds_to_be_killed_by_boss(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<&str>,
) -> f64 {
    let dps = get_boss_dps(stats, level, tier, role);
    let hp = get_player_hp(stats, level, tier, role);

    let regen_per_sec = stats.get("regenHp", level, tier, role) * *TICKS_PER_SECOND;

    hp / f64::max(0.000001, dps - regen_per_sec)
}

const heuristicHandlers: [(
    &str,
    for<'a, 'b> fn(&'a Stats, Option<usize>, Option<&'b str>, Option<&str>) -> f64,
); 4] = [
    ("secondsToKillMob", get_seconds_to_kill_mob),
    ("secondsToKillBoss", get_seconds_to_kill_boss),
    ("secondsToBeKilledByMob", get_seconds_to_be_killed_by_mob),
    ("secondsToBeKilledByBoss", get_seconds_to_be_killed_by_boss),
];

// //Method
pub fn set_node_fitness(node: Node) {
    let Node { stats, .. } = node;

    let mut fitnessResult = vec![];

    let mut fitness = 0.;

    ROLES.iter().for_each(|role| {
        TIERS.iter().for_each(|tier| {
            CONFIG_REQUIREMENTS
                .as_object()
                .unwrap()
                .iter()
                .for_each(|(key, val)| {
                    let rqrConfig = val.as_object().unwrap();
                    let Some(found_tier) = rqrConfig
                        .values()
                        .find(|x| !x.as_object().unwrap()[*tier].is_null())
                        .map(|x| x.as_object().unwrap()[*tier].as_object().unwrap())
                    else {
                        return;
                    };

                    if found_tier["role"].is_null() {
                        return;
                    }

                    for level in LEVEL_MIN..=LEVEL_MAX {
                        let res = heuristicHandlers
                            .iter()
                            .find(|(name, _)| name == &key.as_str())
                            .expect("To have heuristics")
                            .1(
                            &stats, Some(level), Some(*tier), Some(role)
                        );

                        fitness += f64::abs(found_tier["role"].as_f64().unwrap() - res);

                        fitnessResult.push(json!({
                            "heuristic": key,
                            "level": level,
                            "tier":tier,
                            "role":role,
                            "actual": res,
                            "req": found_tier
                        }));
                    }
                });
        });
    });
}
// 	node.fitness = fitness;

// 	const getActual = (heuristic, tier, role) => {
// 		const filtered = fitnessResult.filter(f => f.heuristic === heuristic && f.tier === tier && f.role === role);
// 		const actual = filtered.reduce((prev, next) => prev + next.actual, 0) / filtered.length;

// 		return actual;
// 	};

// 	const heuristics = [
// 		"secondsToKillMob",
// 		"secondsToKillBoss",
// 		"secondsToBeKilledByMob",
// 		"secondsToBeKilledByBoss"
// 	];

// 	node.fitnessResult = [];

// 	roles.forEach(role => {
// 		tiers.forEach(tier => {
// 			heuristics.forEach(heuristic => {
// 				const entry = fitnessResult.find(f => f.heuristic === heuristic && f.tier === tier && f.role === role);
// 				if (!entry)
// 					return;

// 				node.fitnessResult.push({
// 					heuristic,
// 					tier,
// 					role,
// 					actual: getActual(heuristic, tier, role),
// 					req: entry.req
// 				});
// 			});
// 		});
// 	});
// };

// //Exports
// module.exports = setNodeFitness;
