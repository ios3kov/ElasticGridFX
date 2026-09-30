'use strict';
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(__dirname+'/ae_binding_probe.jsx','utf8');
const config={projectPath:'/owned.aep',runId:'a'.repeat(32)};
function fixture(){
 function CompItem(){}
 const names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL'];
 const points=names.map(name=>({name,numKeys:0,expressionEnabled:true,expression:'// FSTR research plane v1\n[1,2]',expressionError:'',value:[1,2]}));
 const builtin={matchName:'ADBE Effect Built In Params'};
 const kind={name:'__FSTR Plane Kind',numKeys:0,expressionEnabled:true,expressionError:'',value:1};
 const fx={matchName:'com.elasticgrid.fx.warp',numProperties:29,property(i){return i===29?builtin:i===28?kind:points[i-24];}};
 const effects={numProperties:2,property(i){return i===2?fx:{property(){return {expression:''};}};}};
 const layer={name:'__EGFX_TEST_TEXT',property(n){return n==='ADBE Text Properties'?{}:effects;}};
 const comp=Object.assign(new CompItem(),{name:'__EGFX_TEXT_'+config.runId,comment:'EGFX_TEXT_PROBE_V1:'+config.runId,numLayers:1,width:640,height:480,layer(){return layer;}});
 const ctx={CompItem,app:{project:{file:{fsName:config.projectPath},activeItem:comp}}};
 vm.runInNewContext('Array.prototype.toSource=function(){return JSON.stringify(this)};'+source,ctx);
 return {ctx,fx,builtin,points,kind};
}
const f=fixture();assert.match(f.ctx.EGFXBindingProbe(config,true),/^PASS/);
for(const alter of [f=>f.fx.numProperties=28,f=>f.builtin.matchName='foreign',f=>f.points[0].expressionError='bad',f=>f.points[1].numKeys=1,f=>f.kind.value=2]){
 const f=fixture();alter(f);assert.throws(()=>f.ctx.EGFXBindingProbe(config,true));
}
console.log('binding schema includes native Compositing Options; negative controls PASS');
