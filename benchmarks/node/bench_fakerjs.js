// Benchmark: @faker-js/faker
// Usage: node bench_fakerjs.js N TIER

const { faker } = require('@faker-js/faker');
const N = parseInt(process.argv[2]) || 100000;
const TIER = parseInt(process.argv[3]) || 3;
faker.seed(42);

const GEN = {
    1:  () => faker.person.fullName().length,
    3:  () => faker.person.fullName().length + faker.internet.email().length + faker.phone.number().length,
    5:  () => GEN[3]() + faker.location.city().length + faker.date.birthdate().toISOString().length,
    10: () => GEN[5]() + faker.location.country().length + faker.internet.username().length + faker.location.zipCode().length + faker.internet.ip().length + String(faker.location.latitude()).length,
    15: () => GEN[10]() + String(faker.location.longitude()).length + faker.company.name().length + faker.person.jobTitle().length + faker.finance.creditCardNumber().length + faker.string.numeric(9).length,
    20: () => GEN[15]() + faker.string.uuid().length + faker.internet.password().length + faker.location.streetAddress().length + faker.finance.amount().length + faker.person.sex().length,
};

const gen = GEN[TIER];
if (!gen) { process.stderr.write(`unsupported tier: ${TIER}\n`); process.exit(1); }

let sink = 0;
const start = process.hrtime.bigint();
for (let i = 0; i < N; i++) sink += gen();

const elapsed = Number(process.hrtime.bigint() - start) / 1e9;
const rps = Math.round(N / elapsed);
console.log(JSON.stringify({ tool: 'faker-js', n: N, tier: TIER, elapsed: elapsed.toFixed(3), rps, sink }));
