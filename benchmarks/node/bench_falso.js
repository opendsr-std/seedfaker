// Benchmark: @ngneat/falso
// Usage: node bench_falso.js N TIER

const falso = require('@ngneat/falso');
const N = parseInt(process.argv[2]) || 100000;
const TIER = parseInt(process.argv[3]) || 3;

const GEN = {
    1:  () => falso.randFullName({ length: 1 })[0].length,
    3:  () => falso.randFullName({ length: 1 })[0].length + falso.randEmail({ length: 1 })[0].length + falso.randPhoneNumber({ length: 1 })[0].length,
    5:  () => GEN[3]() + falso.randCity({ length: 1 })[0].length + falso.randPastDate({ length: 1 })[0].toISOString().length,
    10: () => GEN[5]() + falso.randCountry({ length: 1 })[0].length + falso.randUserName({ length: 1 })[0].length + falso.randZipCode({ length: 1 })[0].length + falso.randIp({ length: 1 })[0].length + String(falso.randLatitude({ length: 1 })[0]).length,
    15: () => GEN[10]() + String(falso.randLongitude({ length: 1 })[0]).length + falso.randCompanyName({ length: 1 })[0].length + falso.randJobTitle({ length: 1 })[0].length + falso.randCreditCardNumber({ length: 1 })[0].length + String(falso.randNumber({ min: 100000000, max: 999999999, length: 1 })[0]).length,
    20: () => GEN[15]() + falso.randUuid({ length: 1 })[0].length + falso.randPassword({ length: 1 })[0].length + falso.randAddress({ length: 1 })[0].length + String(falso.randAmount({ length: 1 })[0]).length + falso.randGender({ length: 1 })[0].length,
};

const gen = GEN[TIER];
if (!gen) { process.stderr.write(`unsupported tier: ${TIER}\n`); process.exit(1); }

let sink = 0;
const start = process.hrtime.bigint();
for (let i = 0; i < N; i++) sink += gen();

const elapsed = Number(process.hrtime.bigint() - start) / 1e9;
const rps = Math.round(N / elapsed);
console.log(JSON.stringify({ tool: '@ngneat/falso', n: N, tier: TIER, elapsed: elapsed.toFixed(3), rps, sink }));
