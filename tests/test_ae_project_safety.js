'use strict';
// Control-flow regression for the actual JSX. This is NOT an After Effects host test.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const source = fs.readFileSync(path.join(__dirname, 'ae_project_roundtrip.jsx'), 'utf8');

function run(project, options = {}) {
  const calls = { close: 0, newProject: 0, remove: 0, create: 0, restoreDialogs: 0, paths: [], completions: [] };
  const app = {
    project,
    beginSuppressDialogs() {},
    endSuppressDialogs() { calls.restoreDialogs++; },
    newProject() { calls.newProject++; app.project = { bitsPerChannel: 8 }; return app.project; },
  };
  if (project) {
    project.close = () => {
      calls.close++;
      if (options.closeFails) return false;
      app.project = null;
      return true;
    };
    project.items = { addComp() {
      if (options.foreignProject) app.project = options.foreignProject;
      throw new Error('intentional fixture creation failure');
    } };
  }
  function File(name) {
    this.fsName = String(name);
    this.name = path.basename(this.fsName);
    this.open = () => true;
    this.write = value => { calls.completions.push(JSON.parse(value)); return true; };
    this.close = () => true;
    this.rename = () => true;
    this.exists = Boolean(options.preexistingPayload);
    this.length = 10;
    this.remove = () => { calls.remove++; return true; };
    calls.paths.push(this.fsName);
  }
  function Folder(name) {
    this.fsName = String(name);
    this.exists = Boolean(options.workspaceExists);
    this.create = () => { calls.create++; this.exists = true; return true; };
  }
  Folder.temp = { fsName: '/mock-temp' };
  vm.runInNewContext(source, { app, File, Folder, ELASTICGRID_ROUNDTRIP_CONFIG: options.config, CloseOptions: { DO_NOT_SAVE_CHANGES: 0 } }, { timeout: 1000 });
  return { calls, app };
}
const guardedCases = [
  ['saved project', { file: { fsName: '/user/work.aep' }, numItems: 1, dirty: false }],
  ['unsaved work', { file: null, numItems: 1, dirty: true }],
  ['dirty empty project', { file: null, numItems: 0, dirty: true }],
  ['dirty getter failure', { file: null, numItems: 0, get dirty() { throw new Error('host error'); } }],
  ['missing project', null],
  ['unknown dirty state', { file: null, numItems: 0 }],
];
for (const [name, project] of guardedCases) {
  const { calls, app } = run(project);
  assert.equal(app.exitCode, 40, name);
  assert.equal(calls.close, 0, `${name}: must not close user project`);
  assert.equal(calls.newProject, 0, `${name}: must not replace user project`);
  assert.equal(calls.remove, 0, `${name}: must not delete files`);
  assert.equal(calls.create, 0, `${name}: must not create test workspace`);
  assert.equal(calls.restoreDialogs, 1, `${name}: dialog suppression must be restored`);
}
const empty = () => ({ file: null, numItems: 0, dirty: false, bitsPerChannel: 16 });
for (const options of [{ workspaceExists: true }, { preexistingPayload: true }]) {
  const { calls, app } = run(empty(), options);
  assert.equal(app.exitCode, 40);
  assert.equal(calls.close + calls.newProject + calls.remove, 0, 'workspace collision must be non-destructive');
  assert.equal(app.project.bitsPerChannel, 16, 'workspace refusal must preserve bit depth');
}
{
  const { calls, app } = run(empty());
  assert.equal(app.exitCode, 41);
  assert.equal(calls.close, 1, 'failed test may close only its owned empty project');
  assert.equal(calls.newProject, 1);
  assert.equal(app.project.bitsPerChannel, 16, 'restore bit depth on owned cleanup');
  assert.equal(calls.remove, 0, 'retain unique workspace evidence');
  assert.ok(calls.paths.every(p => p.startsWith('/mock-temp/ElasticGridFX-roundtrip-')));
}
{
  const project = empty();
  const { calls, app } = run(project, { closeFails: true });
  assert.equal(app.exitCode, 41);
  assert.equal(calls.close, 1);
  assert.equal(calls.newProject, 0, 'failed close must not replace current project');
  assert.equal(app.project, project);
}
{
  const foreign = { file: null, numItems: 10, dirty: true, bitsPerChannel: 8 };
  const { calls, app } = run(empty(), { foreignProject: foreign });
  assert.equal(app.exitCode, 41);
  assert.equal(app.project, foreign);
  assert.equal(calls.close + calls.newProject + calls.remove, 0, 'context change must not touch foreign project');
  assert.equal(foreign.bitsPerChannel, 8);
}
for (const options of [{}, { closeFails: true }]) {
  const config = { run_id: 'a'.repeat(32), build_id: 'EGFX-' + 'b'.repeat(24),
    folder: '/mock-temp/owned/evidence', result_file: '/mock-temp/owned/completion.json' };
  const { calls, app } = run(empty(), { ...options, config });
  assert.equal(calls.completions.length, 1);
  assert.deepEqual(calls.completions[0], { schema: 1, run_id: config.run_id,
    build_id: config.build_id, status: 'FAIL', exit_code: app.exitCode });
  assert.equal(app.exitCode, 41);
}
console.log('PASS: 13 roundtrip ownership/refusal/cleanup cases (mock control flow; not AE verification)');

// Execute the fixture's real compatibility lookup against flat and grouped AE models.
const lookupSource = source.slice(0, source.indexOf('(function ()'));
const lookupContext = {};
vm.runInNewContext(lookupSource, lookupContext);
const value = { name: 'Wave Amplitude', numKeys: 2 };
function group(children) {
  return { numProperties: children.length, property(key) {
    return typeof key === 'number' ? children[key - 1] : children.find(c => c.name === key) || null;
  } };
}
for (const model of [group([value]), group([{ ...group([value]), name: 'Wave Animation' }])]) {
  assert.equal(lookupContext.egfxWaveParam(model, 'Wave Amplitude'), value);
  assert.equal(lookupContext.egfxWaveParam(model, 'Wave Speed'), null);
  assert.equal(value.numKeys, 2);
}
console.log('PASS: flat/grouped Wave parameter lookup preserves the original parameter object');

// File.exists is a construction-time snapshot in this mock. A delayed export
// must be observed through a fresh File and a missing/empty export must time out.
for (const [readyAt, length, expected] of [[2, 100, true], [0, 100, true], [2, 0, false], [Infinity, 100, false]]) {
  let ticks = 0, reads = 0;
  lookupContext.File = function (name) {
    assert.equal(name, '/owned/frame.png'); reads++;
    this.exists = ticks >= readyAt;
    this.length = this.exists ? length : 0;
  };
  lookupContext.$ = { sleep(ms) { assert.equal(ms, 100); ticks++; } };
  const file = lookupContext.egfxWaitForPng('/owned/frame.png');
  assert.equal(file !== null, expected);
  if (expected) {
    assert.equal(ticks, readyAt);
    assert.equal(reads, readyAt + 1);
  } else {
    assert.equal(ticks, 50);
    assert.equal(reads, 50);
  }
}
console.log('PASS: delayed PNG export, fresh File observation and bounded missing/empty timeout');
