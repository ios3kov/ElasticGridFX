'use strict';
// Safety/control-flow checks for Stage 9 JSX. Mock only; never executes After Effects.
const fs=require('node:fs'),vm=require('node:vm'),path=require('node:path'),assert=require('node:assert/strict');
const source=fs.readFileSync(path.join(__dirname,'ae_plane_smoke.jsx'),'utf8');

function load(options={}) {
  const folder='/owned', run='a'.repeat(32), files={[folder+'/pattern.png']:'png',[folder+'/plane-project.aep']:'aep'};
  const calls={close:0,newProject:0,dialogs:0,imported:0};
  const project=options.project || {file:null,numItems:0,dirty:false,revision:1};
  if (!project.close) project.close=function(){calls.close++;return true;};
  const app={project,
    beginSuppressDialogs(){calls.dialogs++;},endSuppressDialogs(){calls.dialogs--;},
    newProject(){calls.newProject++;this.project={file:null,numItems:0,dirty:false,revision:1};return this.project;},
    open(){throw Error('must not open in guard tests');}
  };
  function File(n){
    this.fsName=n;
    Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,n)});
    Object.defineProperty(this,'length',{get:()=>files[n]?.length||0});
    this.open=()=>true;this.write=t=>{files[n]=t;};this.close=()=>{};
  }
  function Folder(n){this.fsName=n;this.exists=n===folder;}
  function ImportOptions(f){this.file=f;}
  const ctx={app,File,Folder,ImportOptions,CloseOptions:{DO_NOT_SAVE_CHANGES:0}};
  vm.runInNewContext(source,ctx,{timeout:1000});
  return {ctx,app,calls,files,config:{run_id:run,folder}};
}

{
  const unsafe={file:{fsName:'/user/work.aep'},numItems:4,dirty:false,revision:8,
    close(){throw Error('foreign project must not close');}};
  const e=load({project:unsafe});
  assert.notEqual(e.ctx.elasticGridPlaneSmoke(e.config),0);
  assert.equal(e.app.project,unsafe);
  assert.equal(e.calls.close+e.calls.newProject+e.calls.dialogs,0);
  const report=JSON.parse(e.files['/owned/plane-smoke.json']);
  assert.equal(report.status,'FAIL');
  assert.equal(report.stage,'guard');
}
{
  const foreign={file:{fsName:'/user/foreign.aep'},numItems:1,dirty:false,revision:2,
    close(){throw Error('foreign project must not close');}};
  const e=load({project:foreign});
  assert.notEqual(e.ctx.elasticGridPlaneSmokeCleanup(e.config),0);
  assert.equal(e.app.project,foreign);
  assert.equal(e.calls.close+e.calls.newProject,0);
  const report=JSON.parse(e.files['/owned/plane-cleanup.json']);
  assert.equal(report.status,'FAIL');
  assert.equal(report.stage,'guard');
}
assert.ok(source.includes('app.project.file===null && app.project.numItems===0'));
assert.ok(source.includes('app.project.file.fsName===expected.fsName'));
assert.ok(source.includes('CloseOptions.DO_NOT_SAVE_CHANGES'));
assert.ok(source.includes('file=new File(path);'));
assert.ok(source.includes('attempt<50'));
assert.ok(source.includes('$.sleep(100)'));
assert.ok(source.indexOf('capture("3d-no-camera")') < source.indexOf('addCamera("__EGFX_CAMERA_A_'));
assert.ok(source.includes('comp.numLayers===1 && comp.layer(1)===layer'));
assert.ok(!source.includes('Unexpected camera before camera-layer creation'));
assert.ok(!source.includes('cameraB.enabled=false;camera.enabled=false'));
assert.ok(source.includes('never retry the render or kill/restart AE'));
assert.ok(!source.includes('app.quit('));
assert.ok(!source.includes('system.callSystem'));
console.log('PASS: Stage 9 plane JSX refuses foreign project mutation (mock only)');
