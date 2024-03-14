const buildNode = require('./buildNode');

const produceOffspring = async (node, parentA, parentB) => {
	node.stats.forEach(s => {
		let chooseParent = parentA;

		const roll = Math.random();
		if (roll > 0.5)
			chooseParent = parentB;

		s.value = chooseParent.stats.find(f => f.stat === s.stat).value;
	});

	await node.run({
		type: 'setStats',
		data: node.stats
	});

	return node;
};

module.exports = produceOffspring;
