//Imports
const { 
	ticksTakenToDealDmgBase,
	ticksPerSecond,
	levelMin,
	levelMax,
	tiers,
	roles
} = require('../config');

const configRequirements = require('../config/requirements');

//Getters

//Dps
const getPlayerDps = (stats, level, tier, role) => {
	let dmg = stats.get('playerDmgBase', level, tier, role);
	dmg *= Math.max(1, stats.get('mainStat', level, tier, role));

	const dmgPercent = (
		100 +
		stats.get('globalDmgPercent', level, tier, role)
	);

	const critChance = (
		stats.get('baseCritChance') +
		(
			stats.get('globalCritChance', level, tier, role) * 0.05
		) +
		(
			stats.get('attackSpellCritChance', level, tier, role) * 0.05
		)
	);

	const critMultiplier = (
		stats.get('baseCritMultiplier') +
		stats.get('globalCritMultiplier', level, tier, role) +
		stats.get('attackSpellCritMultiplier', level, tier, role)
	);

	dmg *= dmgPercent / 100;

	dmg = (dmg * (critChance / 100) * (critMultiplier / 100)) + (dmg * (1 - (critChance / 100)));

	const dps = (dmg / ticksTakenToDealDmgBase) * ticksPerSecond;

	return dps;
};

const getMobDps = (stats, level, tier, role) => {
	let dmg = stats.getMob('mobDmgBase', level);
	dmg *= stats.getMob('mobDmgMult', level);

	const armorMultiplier = Math.max(
		0.5 + Math.max((1 - ((stats.get('armor', level, tier, role) || 0) / (level * 50))) / 2, -0.5),
		0.5
	) * 1;

	dmg *= armorMultiplier;

	const avoidChance = stats.get('avoidChance', level, tier, role);

	dmg = dmg * (1 - (avoidChance / 100));

	const dps = (dmg / ticksTakenToDealDmgBase) * ticksPerSecond;

	return dps;
};

const getBossDps = (stats, level, tier, role) => {
	let dmg = stats.getMob('bossDmgBase', level);
	dmg *= stats.getMob('bossDmgMult', level);

	const armorMultiplier = Math.max(
		0.5 + Math.max((1 - ((stats.get('armor', level, tier, role) || 0) / (level * 50))) / 2, -0.5),
		0.5
	);

	dmg *= armorMultiplier;

	const avoidChance = stats.get('avoidChance', level, tier, role);

	dmg = dmg * (1 - (avoidChance / 100));

	const dps = (dmg / ticksTakenToDealDmgBase) * ticksPerSecond;

	return dps;
};

//Hp
const getPlayerHp = (stats, level, tier, role) => {
	let hp = stats.get('playerHpBase');

	hp += stats.get('vit', level, tier, role) * stats.get('vitToHpMultiplier', level, tier, role);

	return hp;
};

const getMobHp = (stats, level, tier, role) => {
	let hp = stats.getMob('mobHpBase', level, tier, role);

	hp *= stats.getMob('mobHpMult', level, tier, role);

	return hp;
};

const getBossHp = (stats, level, tier, role) => {
	let hp = stats.getMob('bossHpBase', level, tier, role);

	hp *= stats.getMob('bossHpMult', level);

	return hp;
};

//Calculations
const getSecondsToKillMob = (stats, level, tier, role) => {
	const dps = getPlayerDps(stats, level, tier, role);
	const hp = getMobHp(stats, level, tier, role);

	return hp / dps;
};

const getSecondsToKillBoss = (stats, level, tier, role) => {
	const dps = getPlayerDps(stats, level, tier, role);
	let hp = getBossHp(stats, level, tier, role);

	return hp / dps;
};

const getSecondsToBeKilledByMob = (stats, level, tier, role) => {
	const dps = getMobDps(stats, level, tier, role);
	const hp = getPlayerHp(stats, level, tier, role);
	const regenPerSec = stats.get('regenHp', level, tier, role) * ticksPerSecond;

	const res = hp / Math.max(0.0000001, (dps - regenPerSec));

	return res;
};

const getSecondsToBeKilledByBoss = (stats, level, tier, role) => {
	const dps = getBossDps(stats, level, tier, role);
	const hp = getPlayerHp(stats, level, tier, role);
	const regenPerSec = stats.get('regenHp', level, tier, role) * ticksPerSecond;

	const res = hp / Math.max(0.0000001, (dps - regenPerSec));

	return res;
};

const heuristicHandlers = {
	secondsToKillMob: getSecondsToKillMob,
	secondsToKillBoss: getSecondsToKillBoss,
	secondsToBeKilledByMob: getSecondsToBeKilledByMob,
	secondsToBeKilledByBoss: getSecondsToBeKilledByBoss
};

//Method
const setNodeFitness = node => {
	const { stats } = node;

	const fitnessResult = [];

	let fitness = 0;

	roles.forEach(role => {
		tiers.forEach(tier => {
			Object.entries(configRequirements).forEach(([rqr, rqrConfig]) => {
				if (rqrConfig[tier]?.[role] === undefined)
					return;

				for (let level = levelMin; level <= levelMax; level++) {
					const res = heuristicHandlers[rqr](stats, level, tier, role);

					fitness += Math.abs(rqrConfig[tier][role] - res);

					fitnessResult.push({
						heuristic: rqr,
						level,
						tier,
						role,
						actual: res,
						req: rqrConfig[tier][role]
					});
				}
			});
		});
	});

	node.fitness = fitness;

	const getActual = (heuristic, tier, role) => {
		const filtered = fitnessResult.filter(f => f.heuristic === heuristic && f.tier === tier && f.role === role);
		const actual = filtered.reduce((prev, next) => prev + next.actual, 0) / filtered.length;

		return actual;
	};

	const heuristics = [
		'secondsToKillMob',
		'secondsToKillBoss',
		'secondsToBeKilledByMob',
		'secondsToBeKilledByBoss'
	];

	node.fitnessResult = [];

	roles.forEach(role => {
		tiers.forEach(tier => {
			heuristics.forEach(heuristic => {
				const entry = fitnessResult.find(f => f.heuristic === heuristic && f.tier === tier && f.role === role);
				if (!entry)
					return;

				node.fitnessResult.push({
					heuristic,
					tier,
					role,
					actual: getActual(heuristic, tier, role),
					req: entry.req
				});
			});
		});
	});
};

//Exports
module.exports = setNodeFitness;
