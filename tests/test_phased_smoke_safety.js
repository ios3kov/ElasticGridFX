'use strict';
// Real JSX guard/cleanup flow under mocks; not native AE acceptance.
const fs=require('node:fs'),vm=require('node:vm'),path=require('node:path'),assert=require('node:assert/strict');
const source=['ae_runtime_smoke.jsx','ae_runtime_smoke_phased.jsx'].map(n=>fs.readFileSync(path.join(__dirname,n),'utf8')).join('\n');
function run(change={}) {
 const nonce='a'.repeat(32),calls={closed:0,newProject:0},files={};
 const fx={matchName:'com.elasticgrid.fx.warp'},parade={numProperties:1,property(){return fx;}};
 const layer={adjustmentLayer:true,property(){return parade;}};
 const comp={id:2,name:'__EGFX_'+nonce,numLayers:1,layer(){return layer;}};
 const chain={id:3,name:'__EGFX_CHAIN_'+nonce,numLayers:2,layer(){return layer;}};
 const items=[{id:1},comp,chain,{id:4},{id:5}];
 const project={file:null,numItems:5,bitsPerChannel:32,linearizeWorkingSpace:false,workingSpace:'',item(i){return items[i-1];},
   close(){calls.closed++;if(change.closeFail)return false;return true;}};
 const app={project,newProject(){calls.newProject++;this.project={file:null,numItems:0,dirty:false,revision:1};return this.project;},version:'25.6x101'};
 if(change.saved)project.file={};
 if(change.foreignItem)items[0].id=99;
 if(change.foreignComp)comp.name='foreign';
 if(change.foreignLayer)chain.numLayers=3;
 if(change.foreignEffect)fx.matchName='foreign';
 if(change.wrongDepth)project.bitsPerChannel=8;
 if(change.foreignProject)app.project=null;
 const config={folder:'/owned',run_id:change.nonce?'invalid':nonce,action:'cleanup',prepared:{run_id:nonce,status:'PREPARED',item_ids:[1,2,3,4,5],comp_id:2,chain_id:3}};
 const context={app,CloseOptions:{DO_NOT_SAVE_CHANGES:0},File:function(name){this.exists=false;this.open=()=>true;this.write=text=>{files[name]=JSON.parse(text);return true;};this.close=()=>true;}};
 vm.createContext(context);vm.runInContext(source,context);context.egfxPhasedCapture(config);
 return {calls,result:files['/owned/capture.json'],code:app.exitCode};
}
const good=run();assert.equal(good.result.status,'CAPTURED');assert.equal(good.result.fresh_guard,'CLEAN');assert.equal(good.calls.closed,1);assert.equal(good.calls.newProject,1);
for(const flag of ['nonce','saved','foreignItem','foreignComp','foreignLayer','foreignEffect','wrongDepth','foreignProject']) {
 const r=run({[flag]:true});assert.equal(r.result.status,'FAIL',flag);assert.equal(r.calls.closed,0,flag);assert.equal(r.calls.newProject,0,flag);
}
const failed=run({closeFail:true});assert.equal(failed.result.status,'FAIL');assert.equal(failed.calls.newProject,0);
console.log('PASS: phased fixture rejects foreign/saved/malformed state; only exact owned scene is closed (mock flow, not AE)');

const c={};vm.createContext(c);vm.runInContext(source,c);
function ready(change={}) {return c.egfxPhasedBindingReady({property(name){if(change.missing)return null;return {expressionEnabled:!change.disabled,expression:change.blank?'':'owned expression',expressionError:change.error?'failed':'',value:change.kind===undefined?1:change.kind};}});}
assert.equal(ready(),true);for(const change of [{missing:true},{disabled:true},{blank:true},{error:true},{kind:0},{kind:4},{kind:NaN}])assert.equal(ready(change),false);
console.log('PASS: pending/invalid binding is rejected before any frame render (mock prerequisite, not native proof)');
