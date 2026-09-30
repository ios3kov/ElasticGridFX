'use strict';
// Executes the production fixture's capture function against an owned queue mock.
// This checks safety/control flow, not AE rendering or color correctness.
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(require('node:path').join(__dirname,'ae_plane_smoke.jsx'),'utf8');
const capture=source.slice(source.indexOf('        function capture(name)'),source.indexOf('        var fit='));
assert.ok(capture.includes('owned.renderQueue.render()'));
function run(options={}) {
 const files=new Set(),frames=[],calls={render:0,remove:0};
 const settings={Format:'PNG Sequence',Channels:'RGB + Alpha',Depth:'Trillions of Colors+',
  Color:'Straight (Unmatted)',Resize:'false',Crop:'false',...options.settings};
 let renderSettings={};
 const output={applyTemplate(name){assert.equal(name,'_HIDDEN X-Factor 16');},getSettings(){return settings;}};
 const item={outputModule(){return output;},setSettings(s){renderSettings=s;},
  getSettings(){return renderSettings;},status:'DONE',remove(){calls.remove++;queue.numItems--;}};
 const queue={numItems:options.foreign?1:0,items:{add(){queue.numItems++;return item;}},
  render(){calls.render++;if(options.renderFailure)throw Error('render failed');files.add('/owned/frame-00000.png');}};
 const owned={renderQueue:queue};const app={project:owned};
 function File(path){this.path=path;Object.defineProperty(this,'exists',{get:()=>files.has(path)});
  this.length=42;this.rename=name=>{files.delete(path);files.add('/owned/'+name);return true;};}
 const context={File,app,owned,folder:{fsName:'/owned'},frames,comp:{resolutionFactor:options.half?[2,2]:[1,1],frameDuration:1/30},
  GetSettingsFormat:{STRING:1},RQItemStatus:{DONE:'DONE'},check(v,m){if(!v)throw Error(m);}};
 vm.runInNewContext(capture,context);
 let error;try{context.capture('frame');}catch(e){error=e;}
 return {error,calls,frames,renderSettings,files};
}
for(const half of [false,true]){
 const r=run({half});assert.equal(r.error,undefined);assert.equal(r.calls.render,1);assert.equal(r.calls.remove,1);
 assert.equal(r.renderSettings.Resolution,half?'Half':'Full');assert.equal(r.renderSettings['Color Depth'],'Current Settings');
 assert.deepEqual(r.frames,['frame']);assert.ok(r.files.has('/owned/frame.png'));
}
for(const settings of [{Channels:'RGB'},{Depth:'Millions of Colors+'},{Color:'Premultiplied (Matted)'},{Format:'QuickTime'},{Resize:'true'},{Crop:'true'}]){
 const r=run({settings});assert.ok(r.error);assert.equal(r.calls.render,0);assert.equal(r.calls.remove,1);assert.equal(r.frames.length,0);
}
const foreign=run({foreign:true});assert.ok(foreign.error);assert.equal(foreign.calls.render+foreign.calls.remove,0);
const failure=run({renderFailure:true});assert.ok(failure.error);assert.equal(failure.calls.remove,1);assert.equal(failure.frames.length,0);
console.log('PASS: owned queue capture format/depth/alpha/resolution guards and failure cleanup (mock)');
