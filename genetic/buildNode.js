//Imports
const childProcess = require('child_process');

//Internal
let lastNodeId = 0;

const onMessage = (node, msg) => {
	if (msg.result?.set) {
		msg.result.set.forEach(s => {
			node[s.key] = s.value;
		});
	}

	node.promises[msg.type].resolver(msg.result);
};

const run = async (node, { type, data }) => {
	const p = {
		promise: null,
		resolver: null
	};

	p.promise = new Promise(res => {
		p.resolver = res;
	});

	node.promises[type] = p;

	node.worker.send({
		type,
		data
	});

	return p.promise;
};

const buildNode = async () => {
	const node = {
		id: ++lastNodeId,
		age: 0,
		stats: [],
		fitness: null,
		fitnessResult: null,
		worker: null,
		promises: {}
	};	

	node.worker = childProcess.fork('./mods/iwd-balance/genetic/node', [JSON.stringify(node)]);
	node.worker.on('message', onMessage.bind(null, node));

	node.run = run.bind(null, node);

	await node.run({
		type: 'init'
	});

	return node;
};

module.exports = buildNode;
