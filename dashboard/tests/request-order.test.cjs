const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { join } = require('node:path');
const { runInNewContext } = require('node:vm');
const html = readFileSync(join(__dirname, '../templates/index.html'), 'utf8');
const source = html.slice(html.indexOf('function app()'), html.lastIndexOf('</script>'));
function harness() {
  const pending = [];
  const context = {
    URLSearchParams, console: { error() {} },
    document: { body: { style: {} }, activeElement: null },
    fetch(url) { return new Promise((resolve, reject) => pending.push({ url, resolve, reject })); }
  };
  runInNewContext(source, context);
  const app = context.app(); app.$nextTick = fn => fn(); app.$refs = {};
  const resolve = (request, body) => request.resolve({ ok: true, json: async () => body });
  return { app, pending, resolve, context };
}

test('newer category results survive an older request finishing last', async () => {
  const { app, pending, resolve } = harness();
  const old = app.fetchNodes();
  app.activeTab = 'generated'; const latest = app.fetchNodes();
  assert.match(pending[1].url, /type=generated/);
  resolve(pending[1], { nodes: [{ id: 'generated-result' }], total_count: 1 }); await latest;
  resolve(pending[0], { nodes: [{ id: 'observed-result' }], total_count: 99 }); await old;
  assert.equal(app.nodes[0].id, 'generated-result'); assert.equal(app.totalCount, 1); assert.equal(app.loading, false);
});
test('an older failed list request cannot clear a newer result or its loading state', async () => {
  const { app, pending, resolve } = harness();
  const old = app.fetchNodes(); const latest = app.fetchNodes();
  pending[0].reject(new Error('old request failed')); await old;
  assert.equal(app.loading, true); assert.equal(app.listError, '');
  resolve(pending[1], { nodes: [{ id: 'latest' }], total_count: 1 }); await latest;
  assert.equal(app.nodes[0].id, 'latest'); assert.equal(app.loading, false);
});
test('older record details cannot replace the selected record details', async () => {
  const { app, pending, resolve } = harness();
  const first = app.openNode({ id: 'first' }); const second = app.openNode({ id: 'second' });
  for (const request of pending.slice(3)) resolve(request, { children: [{ id: 'second-child' }], falsifiers: [], diffs: [] });
  await second;
  for (const request of pending.slice(0, 3)) resolve(request, { children: [{ id: 'first-child' }], falsifiers: [], diffs: [] });
  await first;
  assert.equal(app.selectedNode.id, 'second'); assert.equal(app.selectedChildren[0].id, 'second-child');
});
test('closing a dialog invalidates its in-flight details and restores scrolling', async () => {
  const { app, pending, resolve, context } = harness();
  const loading = app.openNode({ id: 'first' }); app.closeNode();
  for (const request of pending) resolve(request, { children: [{ id: 'late-child' }], falsifiers: [], diffs: [] });
  await loading;
  assert.equal(app.selectedNode, null); assert.equal(app.selectedChildren.length, 0);
  assert.equal(app.detailLoading, false); assert.equal(context.document.body.style.overflow, '');
});
