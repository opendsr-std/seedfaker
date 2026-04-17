// Benchmark: seedfaker npm package (native NAPI-RS)
// Usage: node bench_seedfaker.js N TIER

const path = require('path');
const N = parseInt(process.argv[2]) || 10000;
const TIER = parseInt(process.argv[3]) || 3;

const { SeedFaker } = require(path.resolve(__dirname, '../../packages/npm/index.js'));

const FIELDS = {
    1:  ['name'],
    3:  ['name', 'email', 'phone'],
    5:  ['name', 'email', 'phone', 'city', 'birthdate'],
    10: ['name', 'email', 'phone', 'city', 'birthdate', 'country', 'username', 'postal-code', 'ssn', 'credit-card'],
    15: ['name', 'email', 'phone', 'city', 'birthdate', 'country', 'username', 'postal-code', 'ssn', 'credit-card',
         'address', 'company-name', 'job-title', 'iban', 'password'],
    20: ['name', 'email', 'phone', 'city', 'birthdate', 'country', 'username', 'postal-code', 'ssn', 'credit-card',
         'address', 'company-name', 'job-title', 'iban', 'password',
         'ip', 'uuid', 'timestamp', 'passport', 'national-id'],
};

const fields = FIELDS[TIER];
if (!fields) { process.stderr.write(`unsupported tier: ${TIER}\n`); process.exit(1); }

const f = new SeedFaker({ seed: 'bench' });

// Single NAPI call for entire batch
const start = process.hrtime.bigint();
const records = f.records(fields, { n: N });
let sink = 0;
for (const rec of records) {
    for (const v of Object.values(rec)) {
        sink += v.length;
    }
}
const elapsed = Number(process.hrtime.bigint() - start) / 1e9;
const rps = Math.round(N / elapsed);
console.log(JSON.stringify({ tool: 'seedfaker', n: N, tier: TIER, elapsed: elapsed.toFixed(3), rps, sink }));
