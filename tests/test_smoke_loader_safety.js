'use strict';
const fs=require('node:fs'),vm=require('node:vm'),path=require('node:path'),assert=require('node:assert/strict');
const source=fs.readFileSync(path.join(__dirname,'ae_runtime_smoke.jsx'),'utf8');

function env(options={}){
  const folder='/owned', run='a'.repeat(32), files={[folder+'/pattern.png']:'png'};
  const calls={close:0,newProject:0,dialogs:0};
  const fx={matchName:'com.elasticgrid.fx.warp',property(){return{};}};
  const layer={name:'',property(){return {numProperties:1,addProperty(){return options.noEffect?null:fx;},property(){return fx;}};}};
  const comp={name:'',numLayers:1,layers:{add(){return layer;}},layer(){return layer;}};
  const footage={name:''};
  const project=options.unsafe?{file:{fsName:'/user.aep'},numItems:1,dirty:false,revision:2}:{
    file:null,numItems:0,get dirty(){return options.unknownDirty?undefined:false;},
    get revision(){if(options.revisionThrows)throw Error('host unavailable');return Object.hasOwn(options,'revision')?options.revision:1;},
    importFile(){this.numItems++;this._footage=footage;return footage;},
    items:{addComp(name){comp.name=name;project.numItems++;project._comp=comp;return comp;}},
    item(i){return i===1?this._footage:this._comp;},
    close(){calls.close++;if(options.closeFail)return false;app.project=null;return true;}
  };
  const app={project,version:'mock',beginSuppressDialogs(){calls.dialogs++;},endSuppressDialogs(){calls.dialogs--;},
    newProject(){calls.newProject++;app.project={file:null,numItems:0,
      get dirty(){return options.freshUnknownDirty?undefined:false;},
      get revision(){return Object.hasOwn(options,'freshRevision')?options.freshRevision:1;}};return app.project;}};
  function File(n){this.fsName=n;Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,n)});Object.defineProperty(this,'length',{get:()=>files[n]?.length||0});this.open=()=>true;this.write=t=>files[n]=t;this.close=()=>{};}
  function Folder(n){this.exists=n===folder;} function ImportOptions(f){this.file=f;}
  const ctx={app,File,Folder,ImportOptions,CloseOptions:{DO_NOT_SAVE_CHANGES:0}};
  vm.runInNewContext(source,ctx,{timeout:1000});
  return {ctx,app,calls,files,config:{run_id:run,folder}};
}
{
  const e=env();
  assert.equal(e.ctx.elasticGridSmokeArm(e.config),0);
  assert.equal(JSON.parse(e.files['/owned/arm.json']).status,'ARMED');
  assert.equal(e.app.project.numItems,2);
  assert.equal(e.calls.dialogs,0);
  assert.equal(e.ctx.elasticGridSmokeDisarm(e.config),0);
  assert.equal(e.calls.close,1);
  assert.equal(e.calls.newProject,1);
  assert.equal(JSON.parse(e.files['/owned/disarm.json']).status,'CLEAN');
}
{
  const e=env({unknownDirty:true,revision:1,freshUnknownDirty:true,freshRevision:1});
  assert.equal(e.ctx.elasticGridSmokeArm(e.config),0);
  const arm=JSON.parse(e.files['/owned/arm.json']);
  assert.equal(arm.guard,'DIRTY_UNAVAILABLE');
  assert.equal(arm.project_revision,'1');
  assert.equal(e.ctx.elasticGridSmokeDisarm(e.config),0);
  const disarm=JSON.parse(e.files['/owned/disarm.json']);
  assert.equal(disarm.fresh_guard,'DIRTY_UNAVAILABLE');
  assert.equal(disarm.fresh_project_revision,'1');
}
{
  const e=env({freshUnknownDirty:true,freshRevision:2});
  assert.equal(e.ctx.elasticGridSmokeArm(e.config),0);
  assert.notEqual(e.ctx.elasticGridSmokeDisarm(e.config),0);
  const disarm=JSON.parse(e.files['/owned/disarm.json']);
  assert.equal(disarm.fresh_guard,'DIRTY_UNAVAILABLE');
  assert.equal(disarm.fresh_project_revision,'2');
  assert.equal(e.calls.close,1,'only the verified arm project may be closed');
  assert.equal(e.calls.newProject,1);
}
for (const options of [
  {unknownDirty:true,revision:0},{unknownDirty:true,revision:2},
  {unknownDirty:true,revisionThrows:true}
]) {
  const e=env(options);
  assert.notEqual(e.ctx.elasticGridSmokeArm(e.config),0);
  assert.equal(e.calls.close+e.calls.newProject,0);
}
{
  const e=env({unsafe:true});
  assert.notEqual(e.ctx.elasticGridSmokeArm(e.config),0);
  assert.equal(e.calls.close+e.calls.newProject,0);
}
{
  const e=env({noEffect:true});
  assert.notEqual(e.ctx.elasticGridSmokeArm(e.config),0);
}
console.log('PASS: smoke arm/live-identity cleanup ownership cases (mock only)');
