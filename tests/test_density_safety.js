'use strict';
const assert=require('node:assert/strict'), fs=require('node:fs'), vm=require('node:vm'), path=require('node:path');
const source=fs.readFileSync(path.join(__dirname,'ae_density_invariance.jsx'),'utf8');
let touches=0;
const c={app:{},Folder:function(){touches++;throw Error('Unexpected filesystem access');},
    elasticGridCurrentProjectState:()=>({guard:'OCCUPIED',project_revision:'2'}),
    elasticGridHasTestProjectOwnership:()=>false};
vm.createContext(c);vm.runInContext(source,c);
const config={run_id:'a'.repeat(32),depth:8,kind:'text',folder:'/tmp/not-owned'};
for (const value of [null,{}, {...config,run_id:'../escape'},{...config,depth:24},{...config,kind:'unknown'}])
    assert.throws(()=>c.fstrDensityInvariance(value));
assert.throws(()=>c.fstrDensityInvariance(config),/clean empty unsaved/);
assert.equal(touches,0);
for(const token of ['app.quit(','.purge(','beginSuppressDialogs(','DO_NOT_SAVE_CHANGES','scheduleTask('])
    assert.ok(!source.includes(token),token);
assert.ok(source.includes('CAPTURED_NOT_FULL_ACCEPTANCE'));
console.log('PASS density fixture loading/argument/ownership safety (NOT AE)');
