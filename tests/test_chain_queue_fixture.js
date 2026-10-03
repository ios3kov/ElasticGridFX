'use strict';
// Host/file mocks verify refusal and cleanup; this is not AE pixel evidence.
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict'),path=require('node:path');
const source=fs.readFileSync(path.join(__dirname,'ae_chain_queue_fixture.jsx'),'utf8');
function run(options={}){
 const files={'/owned/pattern.png':'pattern'},folders=new Set(['/owned']),items=[],calls={mutations:0,render:0,remove:0,close:0,newProject:0,dialogs:0};
 const hidden=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
 function effect(name){const params={};return {matchName:name,property(n){if(hidden.includes(n))return{name:n,expressionEnabled:!options.unready,expressionError:options.bindingError?'error':'',value:options.wrongKind?0:1};return params[n]||={value:0,setValue(v){calls.mutations++;this.value=v;}};}};}
 function layer(src){const effects=[];const parade={get numProperties(){return effects.length;},addProperty(name){const fx=effect(name);effects.push(fx);return fx;},property(i){return effects[i-1];}};return{source:src,property(){return parade;}};}
 function comp(name){const layers=[];const c={name,width:319,height:241,duration:2,frameRate:30,frameDuration:1/30,resolutionFactor:[1,1],openInViewer(){},get numLayers(){return layers.length;},layer(i){return layers[i-1];}};c.layers={add(src){const l=layer(src);layers.unshift(l);return l;},addSolid(){const folder={name:'Solids'},src={name:'Solid'};items.push(folder,src);const l=layer(src);layers.unshift(l);return l;}};return c;}
 function File(name){this.fsName=name;Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,name)});Object.defineProperty(this,'length',{get:()=>files[name]?.length||0});this.open=()=>true;this.write=text=>files[name]=text;this.close=()=>{};this.copy=destination=>{files[destination]=files[name];return true;};}
 function Folder(name){this.fsName=name;Object.defineProperty(this,'exists',{get:()=>folders.has(name)});this.create=()=>{folders.add(name);return true;};this.getFiles=()=>Object.keys(files).filter(p=>p.startsWith(name+'/frame_')).map(p=>new File(p));}
 let active;
 const queue={numItems:0,items:{add(c){queue.numItems++;const output={applyTemplate(){},getSettings(){return{Format:'PNG Sequence',Channels:options.noAlpha?'RGB':'RGB + Alpha',Depth:options.lowDepth?'Millions of Colors':'Trillions of Colors+',Color:options.matted?'Premultiplied (Matted)':'Straight (Unmatted)',Resize:'false',Crop:'false'};}};active={status:'DONE',applyTemplate(){},outputModule(){return output;},setSettings(s){this.settings=s;},getSettings(){return this.settings;},remove(){calls.remove++;queue.numItems--;},output};return active;}},render(){calls.render++;if(options.foreignDuringRender)app.project={file:{},numItems:1};if(!options.noOutput)files[active.output.file.fsName.replace('[#####]','00008')]='PNG';}};
 const project={file:options.saved?{}:null,dirty:!!options.dirty,get numItems(){return items.length;},renderQueue:queue,
 importFile(input){const item={file:input.file};items.push(item);return item;},items:{addComp(name){const c=comp(name);items.push(c);return c;}},item(i){return items[i-1];},close(){calls.close++;if(options.closeFail)return false;app.project=null;return true;}};
 const app={project,beginSuppressDialogs(){calls.dialogs++;},endSuppressDialogs(){calls.dialogs--;},newProject(){calls.newProject++;app.project={file:null,numItems:0,dirty:false};}};
 const config={run_id:'a'.repeat(32),folder:'/owned'};
 const context={app,File,Folder,ImportOptions:function(f){this.file=f;},CloseOptions:{DO_NOT_SAVE_CHANGES:0},GetSettingsFormat:{STRING:1},RQItemStatus:{DONE:'DONE'}};
 vm.createContext(context);vm.runInContext(source,context);let error;
 try{context.egfxCreateChainQueue(config);if(options.foreignBetween)app.project={file:{},numItems:5,renderQueue:queue};if(options.extraItem)items.push({name:'foreign'});if(options.wrongSource)items[1].layer(1).source={file:new File('/foreign.png')};context.egfxCaptureChainQueue(config);}catch(e){error=e;}
 let report;try{report=JSON.parse(files['/owned/chain-queue.json']);}catch{}
 return{calls,error,report,app};
}
const valid=run();assert.equal(valid.error,undefined);assert.equal(valid.report.status,'CAPTURED');assert.equal(valid.report.frames.length,10);assert.equal(valid.calls.render,10);assert.equal(valid.calls.remove,10);assert.equal(valid.calls.close,1);assert.equal(valid.calls.newProject,1);assert.equal(valid.calls.dialogs,0);
for(const options of [{saved:true},{dirty:true},{foreignBetween:true},{extraItem:true},{wrongSource:true},{unready:true},{bindingError:true},{wrongKind:true}]){const r=run(options);assert.notEqual(r.report?.status,'CAPTURED');assert.equal(r.calls.render+r.calls.close+r.calls.newProject+r.calls.mutations,0,'unproven scene must remain untouched');}
for(const options of [{noAlpha:true},{lowDepth:true},{matted:true}]){const r=run(options);assert.equal(r.report.status,'FAIL');assert.equal(r.calls.render,0);assert.equal(r.calls.remove,1);assert.equal(r.report.status,'FAIL');}
const failed=run({noOutput:true});assert.equal(failed.report.status,'FAIL');assert.equal(failed.calls.remove,1);assert.equal(failed.report.status,'FAIL');
const foreign=run({foreignDuringRender:true});assert.equal(foreign.report.status,'FAIL');assert.equal(foreign.calls.close+foreign.calls.newProject+foreign.calls.remove,0,'foreign project untouched after render');
const close=run({closeFail:true});assert.equal(close.report.status,'FAIL');assert.equal(close.calls.newProject,0);
console.log('PASS: chain queue owned structure, readiness, depth/alpha and cleanup guards (mock only)');
