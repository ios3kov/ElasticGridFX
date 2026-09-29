'use strict';
const fs=require('node:fs'),vm=require('node:vm'),path=require('node:path'),assert=require('node:assert/strict');
for(const name of ['ae_plane_ui_probe.jsx','ae_plane_migration_probe.jsx']) {
  const writes={};let closed=0;
  const foreign={file:{fsName:'/user/work.aep'},numItems:2,dirty:true,close(){closed++;throw Error('foreign close');}};
  const app={project:foreign};
  function File(p){this.fsName=p;this.exists=false;this.open=()=>true;this.write=s=>{writes[p]=s;};this.close=()=>{};}
  function Folder(p){this.fsName=p;this.exists=true;}
  const ctx={app,File,Folder};vm.runInNewContext(fs.readFileSync(path.join(__dirname,name),'utf8'),ctx);
  const config={run_id:'a'.repeat(32),folder:'/owned'};
  if(name==='ae_plane_ui_probe.jsx') {
    assert.throws(()=>ctx.elasticGridPlaneUIProbe(config),/Requires clean empty/);
    assert.throws(()=>ctx.elasticGridPlaneUIStage(config),/Foreign project/);
  } else {
    assert.equal(ctx.elasticGridPlaneMigrationProbe(config),1);
    assert.equal(JSON.parse(writes['/owned/migration.json']).status,'FAIL');
  }
  assert.equal(app.project,foreign);assert.equal(closed,0);
}
console.log('PASS: visual/migration probes refuse foreign projects (mock only)');
