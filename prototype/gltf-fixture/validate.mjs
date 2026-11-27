// GE-0019 format gate: run the official Khronos glTF-Validator on an exported GLB.
// Usage: node validate.mjs <path-to.glb>
import { readFileSync } from 'node:fs';
import validator from 'gltf-validator';

const path = process.argv[2];
if (!path) {
  console.error('usage: node validate.mjs <path-to.glb>');
  process.exit(2);
}
const bytes = readFileSync(path);
const result = await validator.validateBytes(new Uint8Array(bytes), {
  uriResolver: undefined,
  maxIssues: 100,
});
const issues = result.issues ?? { numErrors: 0, numWarnings: 0, numInfos: 0, messages: [] };
console.log(JSON.stringify({
  validator: 'gltf-validator 2.0.0-dev.3.10',
  file: path,
  numErrors: issues.numErrors,
  numWarnings: issues.numWarnings,
  numInfos: issues.numInfos,
  messages: issues.messages,
}, null, 2));
process.exit(issues.numErrors > 0 ? 1 : 0);
