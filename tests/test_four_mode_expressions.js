'use strict';
// Execute the actual embedded expression sources; no replacement math oracle.
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const root=__dirname+'/../host-rust/src/';
const point=fs.readFileSync(root+'binding_point_v3.jsx','utf8');
const kind=fs.readFileSync(root+'binding_kind_v3.jsx','utf8');
for(const mode of [1,2,3,4])for(const threeD of [false,true])for(const text of [false,true])for(const time of [0,1]){
 const rect={left:text?-19:0,top:text?7:0,width:319+time*22,height:241+time*12};
 let rectCalls=0,compCalls=0;
 const layer={transform:{position:{value:threeD?[80,45,4]:[80,45]}},
  sourceRectAtTime(t,ext){assert.equal(t,time);assert.equal(ext,false);rectCalls++;return rect;},
  toComp(p){compCalls++;return [80+p[0]*.5,45+p[1]*.75,4];}};
 if(text)layer.text={sourceText:{value:'FSTR'}};
 const ctx={thisLayer:layer,time,thisProperty:{propertyGroup(n){assert.equal(n,1);return i=>{assert.equal(i,1);return {value:mode};};}}};
 const resultKind=vm.runInNewContext(kind,ctx);
 assert.equal(resultKind,threeD?(text?2:3):(mode===3?4:1));
 for(const [right,bottom] of [[false,false],[true,false],[true,true],[false,true]]){
  const result=vm.runInNewContext(point.replace('@RIGHT@',right?'+r.width':'').replace('@BOTTOM@',bottom?'+r.height':''),ctx);
  const x=rect.left+(right?rect.width:0),y=rect.top+(bottom?rect.height:0);
  assert.deepEqual(Array.from(result),!threeD&&mode===3?[x,y]:[80+x*.5,45+y*.75]);
 }
 assert.equal(rectCalls,4);assert.equal(compCalls,!threeD&&mode===3?0:4);
}
console.log('PASS: embedded v3 expressions, 32 mode/layer/text/time combinations, four corners; old-mode transforms unchanged, 2D Layer bounds local');
