'use strict';
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(__dirname+'/ae_text_plane_probe.jsx','utf8');
for(const project of [null,{file:null,numItems:1,dirty:true},{file:{fsName:'/user.aep'},numItems:1,dirty:false}]) {
  const ctx={app:{project},Folder:function(n){this.fsName=n;this.exists=true;},
    File:function(n){this.fsName=n;this.exists=false;}};
  vm.runInNewContext(source,ctx);
  const config={run_id:'a'.repeat(32),folder:'/owned',three_d:true};
  assert.throws(()=>ctx.elasticGridTextPlaneProbe(config),/Requires clean empty project/);
  assert.throws(()=>ctx.elasticGridTextPlaneToggle(config),/Foreign project/);
  assert.equal(ctx.app.project,project);
}
console.log('text plane probe foreign-project guards PASS');

function fixture() {
  let mutations=0;
  const fx={selected:false};
  const text={property:()=>({})};
  const effects={property:()=>fx};
  const layer={locked:false,property:n=>n==='ADBE Text Properties'?text:effects};
  Object.defineProperty(layer,'threeDLayer',{set:()=>mutations++});
  const comp={name:'__EGFX_TEXT_'+'a'.repeat(32),comment:'EGFX_TEXT_PROBE_V1:'+'a'.repeat(32),
    numLayers:1,width:640,height:480,layer:()=>layer};
  const ctx={app:{project:{file:{fsName:'/owned/text-plane.aep'},activeItem:comp}},
    File:function(n){this.fsName=n;}};
  vm.runInNewContext(source,ctx);
  return {ctx,comp,layer,fx,text,effects,mutations:()=>mutations};
}
const config={run_id:'a'.repeat(32),folder:'/owned',three_d:true};
for(const change of [
  f=>f.comp.comment='', f=>f.comp.numLayers=2, f=>f.comp.width=1920,
  f=>f.comp.layer=()=>null, f=>f.layer.property=()=>null,
  f=>f.effects.property=()=>null, f=>f.layer.locked=true,
  f=>f.text.property=()=>null,
]) {
  const f=fixture();change(f);
  assert.throws(()=>f.ctx.elasticGridTextPlaneToggle(config),/Foreign (comp|layer)/);
  assert.equal(f.mutations(),0);assert.equal(f.fx.selected,false);
}
const valid=fixture();valid.ctx.elasticGridTextPlaneToggle(config);
assert.equal(valid.mutations(),1);assert.equal(valid.fx.selected,true);
assert.throws(()=>valid.ctx.elasticGridTextPlaneToggle({...config,three_d:'false'}),/Invalid toggle config/);
assert.equal(valid.mutations(),1);
console.log('text plane probe structural ownership and positive control PASS');

// Fixture layout must not inherit a user's very large last-used leading.
{
  let savedDocument;
  const property={setValue(){}};
  const doc={leading:386,autoLeading:true};
  const text={property:()=>({value:doc,setValue:d=>{savedDocument={...d};}})};
  const effect={property:()=>property};
  const layer={property:n=>n==='ADBE Text Properties'?text:
    n==='ADBE Effect Parade'?{addProperty:()=>effect}:{property:()=>property}};
  const comp={layers:{addText:()=>layer},openInViewer(){}};
  const ctx={app:{project:{file:null,numItems:0,dirty:false,
    items:{addComp:()=>comp},save(){}}},
    ParagraphJustification:{CENTER_JUSTIFY:1},
    Folder:function(n){this.fsName=n;this.exists=true;},
    File:function(n){this.fsName=n;this.exists=false;}};
  vm.runInNewContext(source,ctx);
  ctx.elasticGridTextPlaneProbe({run_id:'a'.repeat(32),folder:'/owned'});
  assert.equal(savedDocument.leading,110);
  assert.equal(savedDocument.autoLeading,false);
}
console.log('text plane fixture explicit leading PASS');
