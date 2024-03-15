pub mod build_node;
pub mod evolve_node;
pub mod produce_offspring;
pub mod set_node_fitness;
// /* eslint-disable no-console */
// //Config
// const config = require('../config');
// const configMutation = require('../config/mutation');

// //Helpers
// const buildNode = require('./buildNode');
// const produceOffspring = require('./produceOffspring');

// //Internal
// const nodes = [];

// //Methods
// const init = async () => {
// 	for (let i = 0; i < configMutation.initialPopulationSize; i++) {
// 		const node = await buildNode();

// 		nodes.push(node);
// 	}
// };

// const run = async () => {
// 	const ta = +new Date();

// 	let generationsLeft = configMutation.runForGenerations;

// 	while (generationsLeft-- > 0) {
// 		await Promise.all(nodes.map(async n => await n.run({
// 			type: 'simulate'
// 		})));

// 		nodes.sort((a, b) => {
// 			return a.fitness - b.fitness;
// 		});

// 		const keepCount = nodes.length * configMutation.elitismPercentage;

// 		await Promise.all(nodes.map(async n => await n.run({
// 			type: 'syncStats'
// 		})));

// 		const elitismCount = nodes.length * configMutation.elitismPercentage;
// 		const crossoverCount = nodes.length * configMutation.crossoverPercentage;

// 		for (let i = 0; i < crossoverCount; i++) {
// 			let parentA = nodes[0];
// 			let parentB = nodes[0];
// 			while (parentA === parentB) {
// 				parentA = nodes[~~(Math.random() * keepCount)];
// 				parentB = nodes[~~(Math.random() * keepCount)];
// 			}

// 			await produceOffspring(nodes[elitismCount + i], parentA, parentB);
// 		}

// 		const otherCount = configMutation.initialPopulationSize - elitismCount - crossoverCount;

// 		for (let i = 0; i < otherCount; i++) {
// 			await nodes[elitismCount + crossoverCount + i].run({
// 				type: 'initStats'
// 			});
// 		}

// 		for (let i = 0; i < keepCount; i++) {
// 			await nodes[i].run({
// 				type: 'evolveNode'
// 			});
// 		}
// 	}

// 	const tb = +new Date();

// 	const winner = nodes[0];

// 	await winner.run({
// 		type: 'syncFitnessResult'
// 	});

// 	winner.fitnessResult.forEach(f => {
// 		f.fit = ~~((1 - (Math.abs(f.req - f.actual) / f.req)) * 100);
// 		f.actual = +f.actual.toFixed(2);
// 	});

// 	const fitnessPercent = ~~(
// 		winner.fitnessResult.reduce(
// 			(prev, next) => prev + next.fit,
// 			0
// 		) / winner.fitnessResult.length
// 	) + '%';

// 	winner.fitnessResult.sort((a, b) => a.fit - b.fit);

// 	winner.fitnessResult.forEach(f => {
// 		f.fit = f.fit + '%';
// 	});

// 	console.log();
// 	console.log('Population Size:');
// 	console.log(configMutation.initialPopulationSize);
// 	console.log();
// 	console.log('Generations Simulated:');
// 	console.log(configMutation.runForGenerations);
// 	console.log();
// 	console.log('Simulation Time (ms):');
// 	console.log(tb - ta);
// 	console.log();
// 	console.log('Top Node Fitness:', fitnessPercent);
// 	console.table(winner.fitnessResult);
// 	console.log();
// 	//console.log('Stats:');
// 	//console.table(winner.stats);

// 	await winner.run({
// 		type: 'logCharacterStats',
// 		data: {
// 			level: config.levelMax,
// 			tier: 'entry',
// 			role: 'dps'
// 		}
// 	});
// };

// //Eports
// module.exports = {
// 	init,
// 	run
// };
