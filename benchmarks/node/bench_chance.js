// Benchmark: chance.js
// Usage: node bench_chance.js N TIER

const Chance = require('chance');
const N = parseInt(process.argv[2]) || 100000;
const TIER = parseInt(process.argv[3]) || 3;
const chance = new Chance(42);

const GEN = {
    1:  () => chance.name().length,
    3:  () => chance.name().length + chance.email().length + chance.phone().length,
    5:  () => GEN[3]() + chance.city().length + chance.birthday({string: true}).length,
    10: () => GEN[5]() + chance.country({full: true}).length + chance.word().length + chance.zip().length + chance.ip().length + String(chance.latitude()).length,
    15: () => GEN[10]() + String(chance.longitude()).length + chance.company().length + chance.profession().length + chance.cc().length + chance.ssn().length,
    20: () => GEN[15]() + chance.guid().length + chance.hash().length + chance.address().length + String(chance.floating({min: 0, max: 10000, fixed: 2})).length + chance.pickone(["Male", "Female"]).length,
};

const gen = GEN[TIER];
if (!gen) { process.stderr.write(`unsupported tier: ${TIER}\n`); process.exit(1); }

let sink = 0;
const start = process.hrtime.bigint();
for (let i = 0; i < N; i++) sink += gen();

const elapsed = Number(process.hrtime.bigint() - start) / 1e9;
const rps = Math.round(N / elapsed);
console.log(JSON.stringify({ tool: 'chance', n: N, tier: TIER, elapsed: elapsed.toFixed(3), rps, sink }));
