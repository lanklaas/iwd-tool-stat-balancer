module.exports = {
	levelMin: 25,
	levelMax: 25,
	playerManaBase: 10,
	ticksTakenToDealDmgBase: 1,
	ticksPerSecond: 1000 / 350,
	statBase: 0,
	piecesOfGear: 11,
	tiers: [
		'entry',
		'mid',
		'high',
		'op'
	],
	roles: [
		'dps',
		'tank'
	],
	statsPerGearPiece: {
		entry: 1,
		mid: 3,
		high: 5,
		op: 8
	},
	usefulStatsPerGearPiece: {
		entry: 0.5,
		mid: 1.5,
		high: 4,
		op: 7
	},
	rollPerfectionOfStats: {
		entry: 0.5,
		mid: 0.6,
		high: 0.7,
		op: 0.8
	}
};
