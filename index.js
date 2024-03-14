const { init, run } = require('./genetic');

(async () => {
	await init();
		
	await run();
})();
