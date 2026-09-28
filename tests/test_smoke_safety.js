'use strict';
// Executes the real JSX with host/file calls replaced by counters. NOT real AE QA.
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
const assert = require('node:assert/strict');
const source = fs.readFileSync(process.env.EG_SMOKE_JSX || path.join(__dirname, 'ae_runtime_smoke.jsx'), 'utf8');
function run(options = {}) {
    const folder = '/test-owned';
    const config = {run_id:'a'.repeat(32), folder};
    const files = {[folder+'/pattern.png']: 'fixture'};
    if (options.staleResult) files[folder+'/capture.json'] = 'old evidence';
    if (options.staleFrame) files[folder+'/bypass.png'] = 'old frame';
    const calls = {modified:0, removed:0, frames:[], dialogs:0, closed:0};
    const properties = {};
    const fx = {matchName:'com.elasticgrid.fx.warp', enabled:true,
        property(name) {
            if (options.missingParameter && name === 'Wave Amplitude') return null;
            return properties[name] ||= {value:0, setValue(v) {this.value=v;}};
        }};
    const footage = {remove() {calls.removed++; if (options.cleanupError) throw Error('cleanup');}};
    const layer = {source:footage, property() {return {addProperty() {return fx;}};}};
    const comp = {resolutionFactor:[1,1], layers:{add() {return layer;},addSolid() {return layer;}},
        remove() {calls.removed++; if (options.cleanupError) throw Error('cleanup');},
        saveFrameToPng(time,file) {
            calls.frames.push({name:file.fsName.split('/').pop(),time,enabled:fx.enabled,
                amplitude:properties['Wave Amplitude'].value,speed:properties['Wave Speed'].value});
            if (!options.noOutput) files[file.fsName] = 'mock PNG';
            if (options.foreignProject) app.project=options.foreignProject;
        }};
    const project = options.noProject ? null : {
        file:options.saved ? {fsName:'/user/work.aep'} : null,
        numItems:options.occupied ? 5 : 0,
        get dirty() {if(options.dirtyThrows) throw Error('host unavailable'); return options.unknownDirty ? undefined : !!options.dirty;},
        _bpc:16, get bitsPerChannel() {return this._bpc;}, set bitsPerChannel(v) {calls.modified++; this._bpc=v;},
        importFile() {calls.modified++; return footage;},
        items:{addComp() {calls.modified++; return comp;}},
        close() {calls.closed++;}
    };
    const app = {project,version:'test-fixture-not-AE', effects:[{matchName:fx.matchName}],
        beginSuppressDialogs() {calls.dialogs++;},endSuppressDialogs() {calls.dialogs--;}};
    function File(name) {
        this.fsName=name;
        Object.defineProperty(this,'exists',{get:()=>Object.hasOwn(files,name)});
        Object.defineProperty(this,'length',{get:()=>files[name]?.length || 0});
        this.open=()=>!options.writeFailure;
        this.write=text=>{files[name]=text;}; this.close=()=>{};
        this.remove=()=>{calls.removed++; delete files[name];};
    }
    function Folder(name) {this.exists=name===folder;}
    function ImportOptions(file) {this.file=file;}
    const suffix = source.includes('function elasticGridSmoke') ? '\nelasticGridSmoke('+JSON.stringify(config)+');' : '';
    vm.runInNewContext(source+suffix, {app,File,Folder,ImportOptions}, {timeout:1000});
    let capture=null;
    try {capture=JSON.parse(files[folder+'/capture.json']);} catch {}
    return {app,calls,files,capture};
}
for (const options of [{saved:true},{occupied:true},{dirty:true},{dirtyThrows:true},{unknownDirty:true},{noProject:true}]) {
    const {app,calls}=run(options);
    assert.notEqual(app.exitCode,0,'unsafe project must refuse');
    assert.equal(calls.modified+calls.removed+calls.closed,0,'no user project side effects');
    assert.equal(calls.dialogs,0);
}
{
    const {app,calls,capture}=run();
    assert.equal(app.exitCode,0);
    assert.equal(calls.frames.length,7,'must capture seven states, not a single nonempty PNG');
    assert.equal(capture.status,'CAPTURED','JSX cannot declare image assertions PASS');
    assert.equal(capture.loaded_build_id,null,'do not invent observed identity');
    assert.deepEqual(calls.frames.map(f=>f.name), ['bypass.png','identity.png','static_a.png','static_b.png','animated_a.png','animated_b.png','reset.png']);
    assert.equal(calls.frames[0].enabled,false);
    assert.equal(calls.frames[1].enabled,true);
    assert.equal(calls.frames[2].amplitude,10);
    assert.equal(calls.frames[2].speed,0);
    assert.equal(calls.frames[4].speed,0.5);
    assert.equal(calls.frames[6].amplitude,0);
    assert.equal(calls.removed,2); assert.equal(calls.closed,0); assert.equal(calls.dialogs,0);
}
for (const options of [{missingParameter:true},{noOutput:true},{cleanupError:true},{writeFailure:true},{staleFrame:true}]) {
    const {app,capture}=run(options);
    assert.notEqual(app.exitCode,0);
    assert.notEqual(capture?.status,'PASS');
}
{
    const {app,files,calls}=run({staleResult:true});
    assert.notEqual(app.exitCode,0); assert.equal(files['/test-owned/capture.json'],'old evidence');
    assert.equal(calls.modified+calls.removed+calls.closed,0);
}
{
    const foreign={bitsPerChannel:8};
    const {app,calls,capture}=run({foreignProject:foreign});
    assert.equal(app.project,foreign); assert.equal(foreign.bitsPerChannel,8);
    assert.equal(calls.removed+calls.closed,0); assert.equal(capture.stage,'foreign_project');
    assert.notEqual(app.exitCode,0);
}
console.log('PASS: 14 smoke capture/ownership/error cases (mock control flow, not AE execution)');
