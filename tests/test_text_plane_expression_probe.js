'use strict';
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const code=fs.readFileSync(__dirname+'/ae_text_plane_expression_probe.jsx','utf8');
function setup(fail=false) {
  const names=['Plane Top Left','Plane Top Right','Plane Bottom Right','Plane Bottom Left'];
  const props=names.map((_,i)=>({canSetExpression:true,expression:'',numKeys:0,value:[i,i+1],
    get expressionError(){return fail?'simulated evaluation failure':'';},setValue(v){this.value=v;}}));
  const rotation={value:11,setValue(v){this.value=v;}};
  const fx={property:n=>props[names.indexOf(n)]};
  const layer={threeDLayer:true,transform:{yRotation:rotation},property:()=>({property:()=>fx})};
  const ctx={elasticGridTextPlaneToggle:()=>{},app:{project:{activeItem:{layer:()=>layer}}}};
  vm.runInNewContext(code,ctx);
  return {ctx,props,rotation,layer};
}
for(const fail of [false,true]) {
  const f=setup(fail);
  if(fail) assert.throws(()=>f.ctx.elasticGridTextPlaneExpressionProbe({}),/simulated/);
  else assert.equal(f.ctx.elasticGridTextPlaneExpressionProbe({}).length,4);
  assert.equal(f.layer.threeDLayer,true);
  assert.equal(f.rotation.value,11);
  f.props.forEach((p,i)=>{assert.equal(p.expression,'');assert.deepEqual(p.value,[i,i+1]);});
}
const invalid=setup();invalid.props[0].numKeys=1;
assert.throws(()=>invalid.ctx.elasticGridTextPlaneExpressionProbe({}),/Unexpected/);
assert.equal(invalid.rotation.value,11);
console.log('expression probe normal/error restoration and keyed-state refusal PASS');
