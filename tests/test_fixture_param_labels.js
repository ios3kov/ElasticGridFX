'use strict';
// Execute each standalone fixture's actual resolver; no AE runtime claim.
const assert=require('node:assert/strict'),fs=require('node:fs'),vm=require('node:vm'),path=require('node:path');
function group(names,children=[]) {
  const values=[...Object.values(names),...children];
  return {numProperties:values.length,property(key) { return typeof key==='number' ? values[key-1]||null : names[key]||null; }};
}
for(const name of ['ae_runtime_smoke.jsx','perf_fixture.jsx','ae_chain_queue_fixture.jsx']) {
  const source=fs.readFileSync(path.join(__dirname,name),'utf8');
  const start=source.indexOf('function egfxFixtureParam('),end=source.indexOf('\n}\n',start)+3, context={};
  vm.runInNewContext(source.slice(start,end),context);
  const lookup=context.egfxFixtureParam,value={};
  for(const [legacy,current] of [['Falloff','Follow Shape'],['Stretch Easing','Smooth Stretch'],['Easing Distance','Smooth Width'],['Wave Amplitude','Wave Amplitude']]) {
    assert.equal(lookup(group({[legacy]:value}),legacy),value);
    assert.equal(lookup(group({[current]:value}),legacy),value);
    assert.equal(lookup(group({},[group({[current]:value})]),legacy),value);
    assert.equal(lookup(group({}),legacy),null);
  }
  assert.throws(()=>lookup({numProperties:129,property(){return null;}},'Wave Speed'),/count/);
  let nested=group({});for(let i=0;i<10;i++)nested=group({},[nested]);
  assert.throws(()=>lookup(nested,'Wave Speed'),/nesting/);
}
console.log('PASS: legacy/new labels and flat/grouped parameters in all three real fixtures; bounded invalid models rejected');
