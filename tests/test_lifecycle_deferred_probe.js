'use strict';
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(__dirname+'/ae_lifecycle_deferred_probe.jsx','utf8');
const config={projectPath:'/owned/test.aep',runId:'a'.repeat(32)};
function fixture(){
 let changes=0;
 const effect={matchName:'com.elasticgrid.fx.warp',remove(){effects.numProperties--;changes++;}};
 const effects={numProperties:1,property(){return effect;},addProperty(){this.numProperties++;changes++;}};
 const layer={name:'__EGFX_TEST_TEXT',locked:false,property(n){return n==='ADBE Text Properties'?{}:effects;}};
 function CompItem(){}
 const comp=Object.assign(new CompItem(),{name:'__EGFX_TEXT_'+config.runId,comment:'EGFX_TEXT_PROBE_V1:'+config.runId,width:640,height:480,numLayers:1,layer(){return layer;}});
 const project={file:{fsName:config.projectPath},activeItem:comp,dirty:false,save(){throw Error('Must not save');}};
 const ctx={app:{project},CompItem};vm.runInNewContext(source,ctx);
 return {ctx,project,comp,layer,effects,effect,changes:()=>changes};
}
for(const alter of [f=>f.project.file=null,f=>f.project.dirty=true,f=>f.comp.comment='',f=>f.comp.numLayers=2,f=>f.layer.locked=true,f=>f.effect.matchName='foreign',f=>f.effects.numProperties=2]){
 const f=fixture();alter(f);assert.throws(()=>f.ctx.EGFXLifecycleDeferredProbe(config,'add'));assert.equal(f.changes(),0);
}
const f=fixture();assert.equal(f.ctx.EGFXLifecycleDeferredProbe(config,'add'),'add PASS');
assert.equal(f.effects.numProperties,2);assert.equal(f.ctx.EGFXLifecycleDeferredProbe(config,'remove'),'remove PASS');
assert.equal(f.effects.numProperties,1);assert.equal(f.changes(),2);
assert.throws(()=>f.ctx.EGFXLifecycleDeferredProbe(config,'remove'));assert.equal(f.changes(),2);
console.log('deferred lifecycle ownership, separate cleanup, no-save guards PASS');
