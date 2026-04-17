// Benchmark: json-schema-faker
// Usage: node bench_jsf.js N TIER

const jsf = require('json-schema-faker');
const N = parseInt(process.argv[2]) || 100000;
const TIER = parseInt(process.argv[3]) || 3;

jsf.option({ random: () => 0.42 });

const SCHEMAS = {
    1: {
        type: 'object', required: ['name'],
        properties: { name: { type: 'string', faker: 'person.fullName' } }
    },
    3: {
        type: 'object', required: ['name', 'email', 'phone'],
        properties: {
            name: { type: 'string', faker: 'person.fullName' },
            email: { type: 'string', format: 'email' },
            phone: { type: 'string', faker: 'phone.number' },
        }
    },
    5: {
        type: 'object', required: ['name', 'email', 'phone', 'city', 'dob'],
        properties: {
            name: { type: 'string', faker: 'person.fullName' },
            email: { type: 'string', format: 'email' },
            phone: { type: 'string', faker: 'phone.number' },
            city: { type: 'string', faker: 'location.city' },
            dob: { type: 'string', format: 'date' },
        }
    },
    10: {
        type: 'object', required: ['name', 'email', 'phone', 'city', 'dob', 'country', 'username', 'zip', 'ip', 'lat'],
        properties: {
            name: { type: 'string', faker: 'person.fullName' },
            email: { type: 'string', format: 'email' },
            phone: { type: 'string', faker: 'phone.number' },
            city: { type: 'string', faker: 'location.city' },
            dob: { type: 'string', format: 'date' },
            country: { type: 'string', faker: 'location.country' },
            username: { type: 'string', faker: 'internet.username' },
            zip: { type: 'string', faker: 'location.zipCode' },
            ip: { type: 'string', faker: 'internet.ip' },
            lat: { type: 'number', faker: 'location.latitude' },
        }
    },
    15: {
        type: 'object', required: ['name', 'email', 'phone', 'city', 'dob', 'country', 'username', 'zip', 'ip', 'lat', 'lng', 'company', 'job', 'cc', 'ssn'],
        properties: {
            name: { type: 'string', faker: 'person.fullName' },
            email: { type: 'string', format: 'email' },
            phone: { type: 'string', faker: 'phone.number' },
            city: { type: 'string', faker: 'location.city' },
            dob: { type: 'string', format: 'date' },
            country: { type: 'string', faker: 'location.country' },
            username: { type: 'string', faker: 'internet.username' },
            zip: { type: 'string', faker: 'location.zipCode' },
            ip: { type: 'string', faker: 'internet.ip' },
            lat: { type: 'number', faker: 'location.latitude' },
            lng: { type: 'number', faker: 'location.longitude' },
            company: { type: 'string', faker: 'company.name' },
            job: { type: 'string', faker: 'person.jobTitle' },
            cc: { type: 'string', faker: 'finance.creditCardNumber' },
            ssn: { type: 'string', pattern: '\\d{3}-\\d{2}-\\d{4}' },
        }
    },
    20: {
        type: 'object', required: ['name', 'email', 'phone', 'city', 'dob', 'country', 'username', 'zip', 'ip', 'lat', 'lng', 'company', 'job', 'cc', 'ssn', 'uuid', 'pw', 'addr', 'amt', 'gender'],
        properties: {
            name: { type: 'string', faker: 'person.fullName' },
            email: { type: 'string', format: 'email' },
            phone: { type: 'string', faker: 'phone.number' },
            city: { type: 'string', faker: 'location.city' },
            dob: { type: 'string', format: 'date' },
            country: { type: 'string', faker: 'location.country' },
            username: { type: 'string', faker: 'internet.username' },
            zip: { type: 'string', faker: 'location.zipCode' },
            ip: { type: 'string', faker: 'internet.ip' },
            lat: { type: 'number', faker: 'location.latitude' },
            lng: { type: 'number', faker: 'location.longitude' },
            company: { type: 'string', faker: 'company.name' },
            job: { type: 'string', faker: 'person.jobTitle' },
            cc: { type: 'string', faker: 'finance.creditCardNumber' },
            ssn: { type: 'string', pattern: '\\d{3}-\\d{2}-\\d{4}' },
            uuid: { type: 'string', format: 'uuid' },
            pw: { type: 'string', faker: 'internet.password' },
            addr: { type: 'string', faker: 'location.streetAddress' },
            amt: { type: 'number' },
            gender: { type: 'string', enum: ['Male', 'Female'] },
        }
    },
};

const schema = SCHEMAS[TIER];
if (!schema) { process.stderr.write(`unsupported tier: ${TIER}\n`); process.exit(1); }

let sink = 0;
const start = process.hrtime.bigint();
for (let i = 0; i < N; i++) {
    const r = jsf.generate(schema);
    sink += (r.name || '').length;
}

const elapsed = Number(process.hrtime.bigint() - start) / 1e9;
const rps = Math.round(N / elapsed);
console.log(JSON.stringify({ tool: 'json-schema-faker', n: N, tier: TIER, elapsed: elapsed.toFixed(3), rps, sink }));
