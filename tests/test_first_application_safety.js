// Mock safety checks, not an AE run.
'use strict';
const assert=require('node:assert/strict');
const fs=require('node:fs');
const vm=require('node:vm');
const path=require('node:path');
const code=fs.readFileSync(path.join(__dirname,'ae_first_application.jsx'),'utf8');
let touched=0;
const context={
    Folder:function(){touched++;throw Error('Unexpected folder access');},
    app:{},
    elasticGridCurrentProjectState:()=>({guard:'OCCUPIED',project_revision:'2'}),
    elasticGridHasTestProjectOwnership:()=>false,
};
vm.createContext(context); vm.runInContext(code,context);
assert.equal(touched,0,'loading the script must do nothing');
const config={run_id:'a'.repeat(32),depth:8,kind:'solid',three_d:false,folder:'/tmp/unused'};
for (const bad of [null,{}, {...config,run_id:'../escape'}, {...config,depth:24},
                   {...config,kind:'unknown'}, {...config,three_d:undefined}]) {
    assert.throws(()=>context.elasticGridFirstApplicationFixture(bad));
}
assert.throws(()=>context.elasticGridFirstApplicationFixture(config),/clean, empty, unsaved/);
assert.equal(touched,0,'unsafe project must be rejected before file access or mutation');
context.elasticGridHasTestProjectOwnership=undefined;
assert.throws(()=>context.elasticGridFirstApplicationFixture(config),/Load project guard first/);
console.log('PASS first-application fixture argument/project guards (mock only)');
