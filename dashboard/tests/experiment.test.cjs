const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { join } = require('node:path');
const { runInNewContext } = require('node:vm');
const html = readFileSync(join(__dirname, '../templates/exp001.html'), 'utf8');
const script = html.slice(html.indexOf('function app()'), html.lastIndexOf('</script>'));
function harness(fetch) { const ctx = { fetch }; runInNewContext(script, ctx); return ctx.app(); }
test('experiment loads all three types and uses the observation payload contract', async () => {
  const urls = [];
  const app = harness(async url => { urls.push(url); return { ok: true, json: async () => ({ nodes: [{ id: url }], total_count: 1 }) }; });
  await app.init(); assert.equal(app.witnessRecords.length, 3); assert.equal(app.recordsError, '');
  for (const type of ['observed', 'inferred', 'generated']) assert.ok(urls.some(u => u.includes('type=' + type)));
  assert.equal(app.summarizePayload({ measurement: { quantity: 'affinity', value: { numeric: -7, unit: 'kcal/mol' } } }, 'Observed'), 'affinity: -7 kcal/mol');
  assert.ok(!html.includes('innerHTML')); assert.ok(html.includes('x-text="(n.labels || []).join'));
});
test('failed or incomplete experiment coverage clears records and reports an error', async () => {
  for (const response of [{ ok: false, status: 500 }, { ok: true, json: async () => ({ nodes: [], total_count: 101 }) }]) {
    const app = harness(async () => response); await app.init(); assert.equal(app.witnessRecords.length, 0); assert.ok(app.recordsError); assert.equal(app.recordsLoading, false);
  }
});
