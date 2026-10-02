'use strict';
// Guard/readback mocks are not a preview timing or display oracle.
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict'),path=require('node:path');
const source=fs.readFileSync(path.join(__dirname,'ae_preview_fixture.jsx'),'utf8');
function run(options={}){
 const files={},calls={open:0,parameter:0,close:0,newProject:0,viewer:0};
 const modes={FP_OFF:1,FP_ADAPTIVE_RESOLUTION:2,FP_DRAFT:3,FP_FAST_DRAFT:4,FP_WIREFRAME:5};
 const config={run_id:'a'.repeat(32),folder:'/owned',record:'state',action:options.action||'read',source:'/source.aep',source_owner:'source',pattern:'/pattern.png',phase:options.phase??131.37,previous_fast:'FP_ADAPTIVE_RESOLUTION'};
 const hidden=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];const params={'Render Quality':{value:options.lowQuality?1:2},'Deformation Plane':{value:2},'Wave Phase':{value:35,setValue(v){calls.parameter++;this.value=v;}},Columns:{value:8},Rows:{value:8},'Wave Speed':{value:.5}};
 const fx={matchName:'com.elasticgrid.fx.warp',property(n){if(typeof n==='number')return{name:hidden[n-24],expressionEnabled:!options.unready,expressionError:options.expressionError?'error':'',value:options.wrongKind?0:1};return params[n];}};
 const layer={source:{file:{fsName:options.wrongSource?'/foreign.png':config.pattern}},property(){return{numProperties:1,property(){return fx;}};}};
 const viewerOptions={fastPreview:options.adaptive?2:1};const viewer={type:options.layerViewer?'layer':'comp',activeViewIndex:0,views:[{options:viewerOptions}]};
 const comp={name:'EGFX_PERF',comment:config.action==='open'?config.source_owner:'__EGFX_PREVIEW_'+config.run_id,numLayers:1,width:options.geometry?3840:1920,height:1080,duration:2,frameRate:30,resolutionFactor:options.half?[2,2]:[1,1],workAreaStart:0,workAreaDuration:2,layers:{},layer(){return layer;},openInViewer(){calls.viewer++;return viewer;}};
 const project={file:{fsName:options.foreign?'/user.aep':config.source},numItems:2,bitsPerChannel:options.depth?8:32,workingSpace:'',linearizeWorkingSpace:false,item(i){return i===1?comp:{};},close(){calls.close++;if(options.closeFail)return false;app.project=null;return true;}};
 const app={project,open(){calls.open++;app.project=project;},memoryInUse:12345,newProject(){calls.newProject++;app.project={file:null,numItems:0,dirty:false};}};
 if(config.action==='open')app.project={file:null,numItems:0,dirty:!!options.dirty};
 function File(name){this.fsName=name;Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,name)});this.open=()=>true;this.write=s=>files[name]=s;this.close=()=>{};}
 function Folder(){this.exists=true;}
 const context={app,File,Folder,FastPreviewType:modes,ViewerType:{VIEWER_COMPOSITION:'comp'},CloseOptions:{DO_NOT_SAVE_CHANGES:0}};
 vm.createContext(context);vm.runInContext(source,context);let error;try{context.egfxPreviewFixture(config);}catch(e){error=e;}
 let report;try{report=JSON.parse(files['/owned/state.json']);}catch{}
 return{calls,error,report,viewerOptions,comp};
}
for(const action of ['open','read','invalidate','cleanup']){const r=run({action});assert.equal(r.error,undefined);assert.equal(r.report.full_final,true);assert.equal(r.report.binding_ready,true);assert.equal(r.report.scope,'setup/readback only; UI proves preview completion');}
for(const options of [{foreign:true},{geometry:true},{wrongSource:true},{depth:true},{lowQuality:true},{unready:true},{expressionError:true},{wrongKind:true}]){const r=run({...options,action:'invalidate'});assert.ok(r.error);assert.equal(r.calls.parameter+r.calls.close,0,'unproven scene untouched');}
for(const options of [{half:true},{adaptive:true},{phase:0},{phase:999},{phase:NaN},{phase:Infinity},{phase:'131.37'},{layerViewer:true}]){const r=run({...options,action:'invalidate'});assert.ok(r.error);assert.equal(r.calls.parameter,0);}
const invalidation=run({action:'invalidate'});assert.ok(Number.isFinite(invalidation.report.invalidation_start_utc_ms));assert.ok(Number.isFinite(invalidation.report.invalidation_end_utc_ms));
assert.equal(run({action:'read'}).report.invalidation_start_utc_ms,null,'readback is not an invalidation event');
const dirty=run({action:'open',dirty:true});assert.ok(dirty.error);assert.equal(dirty.calls.open,0);
const open=run({action:'open',adaptive:true});assert.equal(open.report.previous_fast,'FP_ADAPTIVE_RESOLUTION');assert.equal(open.viewerOptions.fastPreview,1);
const cleanup=run({action:'cleanup'});assert.equal(cleanup.calls.close,1);assert.equal(cleanup.calls.newProject,1);assert.equal(cleanup.viewerOptions.fastPreview,2);
const failed=run({action:'cleanup',closeFail:true});assert.ok(failed.error);assert.equal(failed.calls.newProject,0);
console.log('PASS: ordinary preview ownership/full-quality/invalidation/restore guards (mock only)');
