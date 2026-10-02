'use strict';
// Control-flow only: real AE creation/rendering must be verified separately.
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict'),path=require('node:path');
const source=fs.readFileSync(path.join(__dirname,'ae_perf_variants.jsx'),'utf8');
function run(options={}){
 const files={'/source/EGFX_PERF.aep':'aep','/source/pattern.png':'png'},calls={open:0,save:0,close:0,phase:0,newProject:0},config={run_id:'a'.repeat(32),folder:'/owned',source:'/source/EGFX_PERF.aep',source_owner:'__EGFX_PERF_source',pattern:'/source/pattern.png',variants:Array.from({length:6},(_,i)=>({folder:'/owned/'+i,project:'/owned/'+i+'/EGFX_PERF.aep',phase:46.37+11.37*i}))};
 if(options.stale)files[config.variants[0].project]='old';
 if(options.staleReport)files['/owned/variants.json']='previous evidence';
 if(options.badPhase!==undefined)config.variants[0].phase=options.badPhase;
 if(options.duplicatePhase)config.variants[1].phase=config.variants[0].phase;
 function File(name){this.fsName=name;Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,name)});Object.defineProperty(this,'length',{get:()=>files[name]?.length||0});this.open=()=>true;this.write=s=>files[name]=s;this.close=()=>{};}
 function Folder(name){this.exists=name==='/owned'||/^\/owned\/[0-5]$/.test(name);}
 const names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
 function opened(){const phase={value:35,setValue(v){calls.phase++;this.value=v;}};const fx={matchName:'com.elasticgrid.fx.warp',property(n){if(typeof n==='number')return{name:names[n-24],expressionEnabled:!options.unready,expressionError:options.expressionError?'error':'',value:options.wrongKind?0:1};return n==='Wave Phase'?phase:{value:2};}};const layer={source:{file:new File(options.wrongSource?'/foreign.png':config.pattern)},property(){return{numProperties:1,property(){return fx;}};}};const comp={name:'EGFX_PERF',comment:config.source_owner,numLayers:1,width:options.geometry?3840:1920,height:1080,frameRate:30,duration:2,resolutionFactor:[1,1],layers:{},layer(){return layer;}};
  const p={file:new File(options.foreignOpened?'/user.aep':config.source),numItems:2,bitsPerChannel:options.depth?8:32,linearizeWorkingSpace:false,workingSpace:'',item(i){return i===1?comp:{};},renderQueue:{numItems:1,item(){return{outputModule(){return{};}};}},save(file){calls.save++;files[file.fsName]='saved';this.file=file;},close(){calls.close++;if(options.closeFail)return false;app.project=null;return true;}};return p;}
 function empty(){return{file:null,numItems:0,dirty:false};}
 const app={project:empty(),open(){calls.open++;app.project=opened();},newProject(){calls.newProject++;app.project=empty();}};
 if(options.saved)app.project.file=new File('/user.aep');if(options.dirty)app.project.dirty=true;if(options.occupied)app.project.numItems=2;
 const context={app,File,Folder,CloseOptions:{DO_NOT_SAVE_CHANGES:0}};vm.createContext(context);vm.runInContext(source,context);let error;try{context.egfxPerfVariants(config);}catch(e){error=e;}
 let report;try{report=JSON.parse(files['/owned/variants.json']);}catch{}
 return{calls,error,report,files};
}
const pass=run();assert.equal(pass.error,undefined);assert.equal(pass.report.status,'PREPARED');assert.equal(pass.report.variants.length,6);assert.equal(pass.calls.open,6);assert.equal(pass.calls.save,6);assert.equal(pass.calls.phase,6);assert.equal(pass.calls.close,6);
for(const options of [{saved:true},{dirty:true},{occupied:true},{stale:true},{staleReport:true},{badPhase:NaN},{badPhase:Infinity},{badPhase:'46.37'},{badPhase:-1},{badPhase:361},{badPhase:35},{duplicatePhase:true}]){const r=run(options);assert.ok(r.error);assert.equal(r.calls.open+r.calls.save+r.calls.close+r.calls.phase,0,'initial/old evidence/phase guard');}
assert.equal(run({staleReport:true}).files['/owned/variants.json'],'previous evidence');
const foreign=run({foreignOpened:true});assert.ok(foreign.error);assert.equal(foreign.calls.save+foreign.calls.close+foreign.calls.phase,0,'foreign opened project untouched');
for(const options of [{geometry:true},{wrongSource:true},{depth:true},{unready:true},{expressionError:true},{wrongKind:true}]){const r=run(options);assert.ok(r.error);assert.equal(r.calls.save+r.calls.phase,0,'unproven source must not be changed');assert.equal(r.report.status,'FAIL');}
const close=run({closeFail:true});assert.ok(close.error);assert.equal(close.calls.newProject,0);assert.equal(close.report.status,'FAIL');
console.log('PASS: fresh phase fixture source/geometry/binding and foreign-project guards (mock only)');
