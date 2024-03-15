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

// const getMobDps = (stats, level, tier, role) => {
// 	let dmg = stats.getMob("mobDmgBase", level);
// 	dmg *= stats.getMob("mobDmgMult", level);

// 	const armorMultiplier = Math.max(
// 		0.5 + Math.max((1 - ((stats.get("armor", level, tier, role) || 0) / (level * 50))) / 2, -0.5),
// 		0.5
// 	) * 1;

// 	dmg *= armorMultiplier;

// 	const avoidChance = stats.get("avoidChance", level, tier, role);

// 	dmg = dmg * (1 - (avoidChance / 100));

// 	const dps = (dmg / ticksTakenToDealDmgBase) * ticksPerSecond;

// 	return dps;
// };

// const getBossDps = (stats, level, tier, role) => {
// 	let dmg = stats.getMob("bossDmgBase", level);
// 	dmg *= stats.getMob("bossDmgMult", level);

// 	const armorMultiplier = Math.max(
// 		0.5 + Math.max((1 - ((stats.get("armor", level, tier, role) || 0) / (level * 50))) / 2, -0.5),
// 		0.5
// 	);

// 	dmg *= armorMultiplier;

// 	const avoidChance = stats.get("avoidChance", level, tier, role);

// 	dmg = dmg * (1 - (avoidChance / 100));

// 	const dps = (dmg / ticksTakenToDealDmgBase) * ticksPerSecond;

// 	return dps;
// };

// //Hp
// const getPlayerHp = (stats, level, tier, role) => {
// 	let hp = stats.get("playerHpBase");

// 	hp += stats.get("vit", level, tier, role) * stats.get("vitToHpMultiplier", level, tier, role);

// 	return hp;
// };

// const getMobHp = (stats, level, tier, role) => {
// 	let hp = stats.getMob("mobHpBase", level, tier, role);

// 	hp *= stats.getMob("mobHpMult", level, tier, role);

// 	return hp;
// };

// const getBossHp = (stats, level, tier, role) => {
// 	let hp = stats.getMob("bossHpBase", level, tier, role);

// 	hp *= stats.getMob("bossHpMult", level);

// 	return hp;
// };

// //Calculations
// const getSecondsToKillMob = (stats, level, tier, role) => {
// 	const dps = getPlayerDps(stats, level, tier, role);
// 	const hp = getMobHp(stats, level, tier, role);

// 	return hp / dps;
// };

// const getSecondsToKillBoss = (stats, level, tier, role) => {
// 	const dps = getPlayerDps(stats, level, tier, role);
// 	let hp = getBossHp(stats, level, tier, role);

// 	return hp / dps;
// };

// const getSecondsToBeKilledByMob = (stats, level, tier, role) => {
// 	const dps = getMobDps(stats, level, tier, role);
// 	const hp = getPlayerHp(stats, level, tier, role);
// 	const regenPerSec = stats.get("regenHp", level, tier, role) * ticksPerSecond;

// 	const res = hp / Math.max(0.0000001, (dps - regenPerSec));

// 	return res;
// };

// const getSecondsToBeKilledByBoss = (stats, level, tier, role) => {
// 	const dps = getBossDps(stats, level, tier, role);
// 	const hp = getPlayerHp(stats, level, tier, role);
// 	const regenPerSec = stats.get("regenHp", level, tier, role) * ticksPerSecond;

// 	const res = hp / Math.max(0.0000001, (dps - regenPerSec));

// 	return res;
// };

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
