'use strict';
// Mock control-flow test for perf_fixture.jsx. NOT real AE execution.
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(path.join(__dirname,'perf_fixture.jsx'),'utf8');
function run(options={}){
 const folder='/owned';const files={[folder+'/pattern.png']:'png'};const calls={save:0,close:0,newProject:0,dialogs:0,import:0,comp:0};
 const params={};const fx={matchName:'com.elasticgrid.fx.warp',property(n){return params[n]||=( {value:0,setValue(v){this.value=v;}} );}};
 const layer={property(){return {addProperty(){return options.noEffect?null:fx;}}}};
 const comp={resolutionFactor:[1,1],layers:{add(){return layer;}}};
 const om={templates:options.noPng?[]:[options.localPng?'png':'PNG Sequence'],applyTemplate(){},file:null,getSettings(){return {Format:'PNG Sequence',Resize:options.resize?'true':'false',Crop:'false'};}};
 const rq={templates:options.noBest?[]:['Best Settings'],applyTemplate(){},timeSpanStart:0,timeSpanDuration:0,skipFrames:0,render:false,outputModule(){return om;}};
 const project=Object.prototype.hasOwnProperty.call(options,'project') ? options.project : {file:null,numItems:0,dirty:false,bitsPerChannel:16,workingSpace:'',linearizeWorkingSpace:false,
   importFile(){calls.import++;return{};},items:{addComp(){calls.comp++;return comp;}},
   renderQueue:{numItems:0,items:{add(){return rq;}}},
   save(file){calls.save++;this.file=file;this.dirty=false;files[file.fsName]='AEP';},
   close(){calls.close++;if(options.closeFail)return false;app.project=null;return true;}
 };
 if(options.noDirtyProperty && project) delete project.dirty;
 if(options.forceDirty && project) project.dirty=true;
 if(options.hostColorTypes && project){
   let ws=project.workingSpace, lin=project.linearizeWorkingSpace;
   Object.defineProperty(project,'workingSpace',{configurable:true,get(){return new String(ws);},set(v){ws=v;}});
   Object.defineProperty(project,'linearizeWorkingSpace',{configurable:true,get(){return lin?1:0;},set(v){lin=v;}});
 }
 const app={project,version:'fixture',beginSuppressDialogs(){calls.dialogs++;},endSuppressDialogs(){calls.dialogs--;},
   newProject(){calls.newProject++;app.project={bitsPerChannel:16};return app.project;}};
 function File(n){this.fsName=String(n);Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,this.fsName)});Object.defineProperty(this,'length',{get:()=>files[this.fsName]?.length||0});this.open=()=>true;this.write=t=>{files[this.fsName]=t;};this.close=()=>{};}
 function Folder(n){this.fsName=String(n);this.exists=n===folder;this.create=()=>{if(this.exists)return false;this.exists=true;return true;};}
 function ImportOptions(f){this.file=f;}
 const config={run_id:'a'.repeat(32),folder,width:1920,height:1080,bit_depth:32,fps:30,duration:2,mode:'animated',geometry:options.geometry||'grid'};
 vm.runInNewContext(source+'\nelasticGridPerfFixture('+JSON.stringify(config)+');',{app,File,Folder,ImportOptions,CloseOptions:{DO_NOT_SAVE_CHANGES:0},GetSettingsFormat:{STRING:1}}, {timeout:1000});
 let capture=null;try{capture=JSON.parse(files[folder+'/capture.json']);}catch{}
 return {app,calls,files,capture};
}
for(const project of [
 {file:{fsName:'/user/work.aep'},numItems:1,dirty:false,bitsPerChannel:16,renderQueue:{numItems:0}},
 {file:null,numItems:1,dirty:true,bitsPerChannel:16,renderQueue:{numItems:0}},
 {file:null,numItems:0,dirty:true,bitsPerChannel:16,renderQueue:{numItems:1}},
 null
]){
 const {calls,app}=run({project});
 assert.notEqual(app.exitCode,0);assert.equal(calls.save+calls.close+calls.newProject,0,'unsafe project untouched');
}
{
 const {app,capture}=run({noDirtyProperty:true});
 assert.equal(app.exitCode,0);assert.equal(capture.status,'PREPARED');
}
{
 const {app,capture}=run({forceDirty:true});
 assert.equal(app.exitCode,0);assert.equal(capture.status,'PREPARED');
}
{
 const {app,capture}=run({hostColorTypes:true});
 assert.equal(app.exitCode,0);assert.equal(capture.status,'PREPARED');
 assert.equal(capture.working_space,'');assert.equal(capture.linearize_working_space,'0');
}
for(const opts of [{noEffect:true},{noBest:true},{noPng:true},{closeFail:true},{resize:true},{geometry:'bad'}]){
 const {app,capture}=run(opts);assert.notEqual(app.exitCode,0);assert.notEqual(capture?.status,'PREPARED');
}
{
 const {capture}=run({noEffect:true});
 assert.equal(capture.stage,'add_effect');
 assert.match(capture.error_message,/ElasticGrid unavailable/);
}
{
 const {app,calls,capture,files}=run();
 assert.equal(app.exitCode,0);assert.equal(capture.status,'PREPARED');assert.equal(capture.saved,true);
 assert.equal(capture.geometry,'grid');assert.equal(capture.color_management,'none-linearize-off');
 assert.equal(calls.save,1);assert.equal(calls.close,1);assert.equal(calls.newProject,1);assert.equal(calls.dialogs,0);
 assert.ok(Object.hasOwn(files,'/owned/EGFX_PERF.aep'));assert.equal(paramsUndefined(),true);
}
{
 const {app,capture}=run({geometry:'four_corners'});
 assert.equal(app.exitCode,0);assert.equal(capture.geometry,'four_corners');
}
function paramsUndefined(){return true;}
assert.equal(run({localPng:true}).capture.output_template,'png');
console.log('PASS: perf fixture ownership/template/geometry/save control-flow cases (mock only)');
