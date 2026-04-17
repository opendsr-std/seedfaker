import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const { SeedFaker } = require("./index.js");
export { SeedFaker };
export default SeedFaker;
