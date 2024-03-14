const logCharacterStats = (node, { level, tier, role }) => {
	const ignoreStats = [
		'mobDmgMult',
		'bossDmgMult',
		'mobHpMult',
		'bossHpMult',
		'vitToHpMultiplier',
		'statMultPerLevel',
		//Remove later
		'lifeOnHit',
		'manaMax',
		'regenMana',
		'elementAllResist',
		'elementResist',
		'attackSpellSpeed'
	];

	console.log(`Character stats | Level ${level} | Tier ${tier} | Role ${role}`);
	console.table(
		node.stats
			.filter(s => !ignoreStats.includes(s.stat))
			.map(s => {
				return {
					stat: s.stat,
					value: ~~node.stats.get(s.stat, level - 1, tier, role)
				};
			})
			.sort((a, b) => {
				if (a.stat > b.stat)
					return 1;

				return -1;
			})
	);
};

module.exports = logCharacterStats;
