const { init, run } = require('./genetic');

module.exports = {
	name: 'Tool: Genetic Stat Balancer',

	disabled: true,

	init: async function () {
		await init();
		
		await run();
	}
};
