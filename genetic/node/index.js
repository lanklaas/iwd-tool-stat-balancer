const config = require('../../config');
const configStats = require('../../config/stats');
const setNodeFitness = require('./../setNodeFitness');
const evolveNode = require('./../evolveNode');
const logCharacterStats = require('./../logCharacterStats');

let node = null;

const setStatsHelpers = () => {
	node.stats.get = (statName, level, tier, role) => {
		const foundStat = node.stats.find(s => s.stat === statName);

		let res = foundStat.value;

		if (!foundStat.noScale)
			res *= level * node.stats.get('statMultPerLevel');

		if (foundStat.roles) {
			res *= (
				config.piecesOfGear *
				(
					config.usefulStatsPerGearPiece[tier] /
					configStats.filter(s => s.roles?.[role]).length
				) *
				config.rollPerfectionOfStats[tier]
			);
		}

		if (foundStat.max && res > foundStat.max)
			res = foundStat.max;

		return res;
	};

	node.stats.getMob = (statName, level) => {
		const foundStat = node.stats.find(s => s.stat === statName);

		let res = foundStat.value;
		if (!foundStat.noScale) {
			res *= (
				level *
				node.stats.get('statMultPerLevel')
			);
		}

		return res;
	};
};

const handlers = {
	init: function () {
		node = JSON.parse(process.argv[2]);

		handlers.initStats();

		setStatsHelpers();
	},

	initStats: function () {
		node.stats.length = 0;

		configStats.forEach(({ initial, disabled, min, ...rest }) => {
			if (disabled)
				return;

			node.stats.push({
				...rest,
				min: min ?? 0,
				value: initial ?? (min ?? 0) + (Math.random() * 100)
			});
		});
	},

	simulate: () => {
		node.age++;

		setNodeFitness(node);

		return {
			set: [{
				key: 'fitness',
				value: node.fitness
			}]
		};
	},

	syncStats: () => {
		return {
			set: [{
				key: 'stats',
				value: node.stats
			}]
		};
	},

	syncFitnessResult: () => {
		return {
			set: [{
				key: 'fitnessResult',
				value: node.fitnessResult
			}]
		};
	},

	setStats: newStats => {
		node.age = 0;
		node.stats = newStats;

		setStatsHelpers();
	},

	evolveNode: () => evolveNode(node),

	logCharacterStats: ({ level, tier, role }) => logCharacterStats(node, { level, tier, role })
};

process.on('message', m => {
	const result = handlers[m.type](m.data);

	process.send({
		type: m.type,
		result
	});
});
