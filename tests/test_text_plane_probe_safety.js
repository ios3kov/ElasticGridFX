'use strict';
const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const source=fs.readFileSync(__dirname+'/ae_text_plane_probe.jsx','utf8');
for(const project of [null,{file:null,numItems:1,dirty:true},{file:{fsName:'/user.aep'},numItems:1,dirty:false}]) {
  const ctx={app:{project},Folder:function(n){this.fsName=n;this.exists=true;},
    File:function(n){this.fsName=n;this.exists=false;}};
  vm.runInNewContext(source,ctx);
  const config={run_id:'a'.repeat(32),folder:'/owned'};
  assert.throws(()=>ctx.elasticGridTextPlaneProbe(config),/Requires clean empty project/);
  assert.throws(()=>ctx.elasticGridTextPlaneToggle(config),/Foreign project/);
  assert.equal(ctx.app.project,project);
}
console.log('text plane probe foreign-project guards PASS');
