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

const p4=fs.readFileSync(root+'binding_point_v4.jsx','utf8');
const k4=fs.readFileSync(root+'binding_kind_v4.jsx','utf8');
for(const mode of [1,2,3,4])for(const three of [false,true])for(const text of [false,true]){
 const rect={left:-19,top:7,width:319,height:241};
 const layer={transform:{position:{value:three?[80,45,4]:[80,45]}},
  sourceRectAtTime(){return rect;},toComp(p){return [80+p[0]*.5,45+p[1]*.75];},
  fromComp(p){return [(p[0]-80)*2,(p[1]-45)/.75];},
  fromCompToSurface(p){return [(p[0]-80)*2,(p[1]-45)/.75];}};
 if(text)layer.text={sourceText:{value:'FSTR'}};
 const ctx={thisLayer:layer,thisComp:{width:640,height:480},time:0,
  thisProperty:{propertyGroup(){return ()=>({value:mode});}}};
 assert.equal(vm.runInNewContext(k4,ctx),mode===1?(three?(text?7:6):5):(three?(text?2:3):(mode===3?4:1)));
 for(const [right,bottom] of [[false,false],[true,false],[true,true],[false,true]]){
  const source=p4.replace('@RIGHT@',right?'+r.width':'').replace('@BOTTOM@',bottom?'+r.height':'')
   .replace('@COMPX@',right?'thisComp.width':'0').replace('@COMPY@',bottom?'thisComp.height':'0');
  const result=Array.from(vm.runInNewContext(source,ctx));
  const comp=[right?640:0,bottom?480:0],local=[rect.left+(right?rect.width:0),rect.top+(bottom?rect.height:0)];
  const expected=mode===1?(three&&text?comp:layer.fromComp(comp)):(!three&&mode===3?local:layer.toComp(local));
  assert.deepEqual(result,expected);
 }
}
console.log('PASS: v4 Comp spans full 640x480 frame through inverse layer transform; Layer/Flat/Perspective retain bounds');
