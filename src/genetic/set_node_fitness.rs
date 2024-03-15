use crate::config::{
    self,
    requirements::{Requirements, Role},
    stats::Stats,
    Config,
};

//Imports
const CONFIG: Config = config::Config::default();

const TICKS_TAKEN_TO_DEAL_DMG_BASE: f32 = CONFIG.ticks_taken_to_deal_dmg_base as f32;
const TICKS_PER_SECOND: f32 = CONFIG.ticks_per_second as f32;
const LEVEL_MIN: f32 = CONFIG.level_min as f32;
const LEVEL_MAX: f32 = CONFIG.level_max as f32;
const TIERS: [&str; 4] = CONFIG.tiers;
const ROLES: [&str; 2] = CONFIG.roles;

const CONFIG_REQUIREMENTS: Requirements = config::requirements::Requirements::default();

//Getters

//Dps
pub fn get_player_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let mut dmg = stats.get("playerDmgBase", level, tier, role);
    let main_stat = stats.get("mainStat", level, tier, role);
    dmg *= if main_stat < 1.0 { 1.0 } else { main_stat };

    let dmg_percent = 100.0 + stats.get("globalDmgPercent", level, tier, role);

    let crit_chance = stats.get("baseCritChance", None, None, None)
        + (stats.get("globalCritChance", level, tier, role) * 0.05)
        + (stats.get("attackSpellCritChance", level, tier, role) * 0.05);

    let crit_multiplier = stats.get("baseCritMultiplier", None, None, None)
        + stats.get("globalCritMultiplier", level, tier, role)
        + stats.get("attackSpellCritMultiplier", level, tier, role);

    dmg *= dmg_percent / 100.0;

    dmg = (dmg * (crit_chance / 100.0) * (crit_multiplier / 100.0))
        + (dmg * (1.0 - (crit_chance / 100.0)));

    (dmg / TICKS_TAKEN_TO_DEAL_DMG_BASE) * TICKS_PER_SECOND
}

pub fn get_mob_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let mut dmg = stats.get("mobDmgBase", level, tier, role);

    let mob_dmg_mult = stats.get_mob("mobDmgMult", level);
    dmg *= mob_dmg_mult;

    let armor_multiplier = {
        let armor_stat = stats.get("armor", level, tier, role);
        let max_armor = f32::max(
            0.5,
            0.5 + f32::max(
                (1.0 - (armor_stat / (level.unwrap() as f32 * 50.0))) / 2.0,
                -0.5,
            ),
        );
        max_armor * 1.0
    };

    dmg *= armor_multiplier;

    let avoid_chance = stats.get("avoidChance", level, tier, role);

    dmg *= 1.0 - (avoid_chance / 100.0);

    (dmg / TICKS_TAKEN_TO_DEAL_DMG_BASE) * TICKS_PER_SECOND
}

pub fn get_boss_dps(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let mut dmg = stats.get("bossDmgBase", level, tier, role);

    let mob_dmg_mult = stats.get_mob("bossDmgMult", level);
    dmg *= mob_dmg_mult;

    let armor_multiplier = {
        let armor_stat = stats.get("armor", level, tier, role);
        let max_armor = f32::max(
            0.5,
            0.5 + f32::max(
                (1.0 - (armor_stat / (level.unwrap() as f32 * 50.0))) / 2.0,
                -0.5,
            ),
        );
        max_armor * 1.0
    };

    dmg *= armor_multiplier;

    let avoid_chance = stats.get("avoidChance", level, tier, role);

    dmg *= 1.0 - (avoid_chance / 100.0);

    (dmg / TICKS_TAKEN_TO_DEAL_DMG_BASE) * TICKS_PER_SECOND
}

// //Hp
pub fn get_player_hp(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let mut hp = stats.get_unscaled("playerHpBase");

    hp += stats.get("vit", level, tier, role) * stats.get("vitToHpMultiplier", level, tier, role);

    hp
}

pub fn get_mob_hp(stats: &Stats, level: Option<usize>) -> f32 {
    let mut hp = stats.get_unscaled("mobHpBase");

    hp *= stats.get_mob("mobHpMult", level);

    hp
}

pub fn get_boss_hp(stats: &Stats, level: Option<usize>) -> f32 {
    let mut hp = stats.get_unscaled("bossHpBase");

    hp *= stats.get_mob("bossHpMult", level);

    hp
}

// //Calculations
pub fn get_seconds_to_kill_mob(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let dps = get_player_dps(stats, level, tier, role);
    let hp = get_mob_hp(stats, level);

    hp / dps
}

pub fn get_seconds_to_kill_boss(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let dps = get_player_dps(stats, level, tier, role);
    let hp = get_boss_hp(stats, level);

    hp / dps
}

pub fn get_seconds_to_be_killed_by_mob(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let dps = get_mob_dps(stats, level, tier, role);
    let hp = get_player_hp(stats, level, tier, role);

    let regen_per_sec = stats.get("regenHp", level, tier, role) * TICKS_PER_SECOND;

    hp / f32::max(0.000001, dps - regen_per_sec)
}

pub fn get_seconds_to_be_killed_by_boss(
    stats: &Stats,
    level: Option<usize>,
    tier: Option<&str>,
    role: Option<Role>,
) -> f32 {
    let dps = get_boss_dps(stats, level, tier, role);
    let hp = get_player_hp(stats, level, tier, role);

    let regen_per_sec = stats.get("regenHp", level, tier, role) * TICKS_PER_SECOND;

    hp / f32::max(0.000001, dps - regen_per_sec)
}

// const heuristicHandlers = {
// 	secondsToKillMob: getSecondsToKillMob,
// 	secondsToKillBoss: getSecondsToKillBoss,
// 	secondsToBeKilledByMob: getSecondsToBeKilledByMob,
// 	secondsToBeKilledByBoss: getSecondsToBeKilledByBoss
// };

// //Method
// const setNodeFitness = node => {
// 	const { stats } = node;

// 	const fitnessResult = [];

// 	let fitness = 0;

// 	roles.forEach(role => {
// 		tiers.forEach(tier => {
// 			Object.entries(configRequirements).forEach(([rqr, rqrConfig]) => {
// 				if (rqrConfig[tier]?.[role] === undefined)
// 					return;

// 				for (let level = levelMin; level <= levelMax; level++) {
// 					const res = heuristicHandlers[rqr](stats, level, tier, role);

// 					fitness += Math.abs(rqrConfig[tier][role] - res);

// 					fitnessResult.push({
// 						heuristic: rqr,
// 						level,
// 						tier,
// 						role,
// 						actual: res,
// 						req: rqrConfig[tier][role]
// 					});
// 				}
// 			});
// 		});
// 	});

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
