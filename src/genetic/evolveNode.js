const configMutation = require('../config/mutation');

const evolveNode = node => {
	const stats = node.stats;
	const evolveStats = [];

	const evolveCount = ~~node.stats.length * configMutation.evolveStatPercentage;

	for (let i = 0; i < evolveCount; i++) {
		let pickStat = null;
		while (pickStat === null || evolveStats.some(f => f.stat === pickStat.stat) || pickStat.static) 
			pickStat = stats[~~(Math.random() * stats.length)];

		evolveStats.push(pickStat);
	}

	evolveStats.forEach(s => {
		const delta = (Math.random() * configMutation.statMutateMaxMult * 2) - configMutation.statMutateMaxMult;
		s.value += delta;
		if (s.value < s.min)
			s.value = s.min;
	});
};

module.exports = evolveNode;
