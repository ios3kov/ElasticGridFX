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
    const cornerPoints = Array.from({length:4},()=>({value:null,setValue(v){this.value=v;}}));
    const corner = {matchName:'ADBE Corner Pin', property(index){return cornerPoints[index-1] || null;}};
    const parade = {addProperty(name) {
        if (name === 'com.elasticgrid.fx.warp') return fx;
        if (name === 'ADBE Corner Pin') return options.cornerUnavailable ? null : corner;
        return null;
    }};
    const footage = {remove() {calls.removed++; if (options.cleanupError) throw Error('cleanup');}};
    const layer = {source:footage, property() {return parade;}};
    const adjustment = {adjustmentLayer:false, property(){return parade;}};
    const comp = {resolutionFactor:[1,1], layers:{add() {return layer;},addSolid() {return adjustment;}},
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
        get dirty() {
            if(options.dirtyThrows) throw Error('host unavailable');
            if(Object.hasOwn(options,'dirtyValue')) return options.dirtyValue;
            return options.unknownDirty ? undefined : !!options.dirty;
        },
        get revision() {
            if(options.revisionThrows) throw Error('host unavailable');
            return Object.hasOwn(options,'revision') ? options.revision : 1;
        },
        _bpc:16, get bitsPerChannel() {return this._bpc;}, set bitsPerChannel(v) {calls.modified++; this._bpc=v;},
        importFile() {calls.modified++; return footage;},
        items:{addComp() {calls.modified++; return comp;}},
        close() {calls.closed++;}
    };
    const app = {project,version:'test-fixture-not-AE', effects:[{matchName:fx.matchName}],
        beginSuppressDialogs() {calls.dialogs++;},endSuppressDialogs() {calls.dialogs--;}};
    function File(name) {
        this.fsName=name;
        const snapshotExists=Object.hasOwn(files,name), snapshotLength=files[name]?.length || 0;
        Object.defineProperty(this,'exists',{get:()=>options.cachedMetadata ? snapshotExists : Object.hasOwn(files,name)});
        Object.defineProperty(this,'length',{get:()=>options.cachedMetadata ? snapshotLength : files[name]?.length || 0});
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
{
    const {app,calls,capture}=run({cachedMetadata:true});
    assert.equal(app.exitCode,0);
    assert.equal(calls.frames.length,10);
    assert.equal(capture.status,'CAPTURED');
}
for (const options of [
    {saved:true},{occupied:true},{dirty:true},{dirtyThrows:true},
    {unknownDirty:true,revision:0},{unknownDirty:true,revision:2},
    {unknownDirty:true,revision:1.5},{unknownDirty:true,revisionThrows:true},
    {dirtyValue:null,revision:1},{dirtyValue:'false',revision:1},{noProject:true}
]) {
    const {app,calls}=run(options);
    assert.notEqual(app.exitCode,0,'unsafe project must refuse');
    assert.equal(calls.modified+calls.removed+calls.closed,0,'no user project side effects');
    assert.equal(calls.dialogs,0);
}
{
    const {app,calls,capture}=run();
    assert.equal(app.exitCode,0);
    assert.equal(calls.frames.length,10,'must capture direct states plus the Adjustment Layer / Corner Pin chain');
    assert.equal(capture.status,'CAPTURED','JSX cannot declare image assertions PASS');
    assert.equal(capture.loaded_build_id,null,'do not invent observed identity');
    assert.equal(capture.guard,'CLEAN');
    assert.equal(capture.project_revision,'1');
    assert.deepEqual(calls.frames.map(f=>f.name), ['bypass.png','identity.png','static_a.png','static_b.png','animated_a.png','animated_b.png','reset.png','chain_before_corner.png','chain_corner_identity.png','chain_corner_moved.png']);
    assert.equal(calls.frames[0].enabled,false);
    assert.equal(calls.frames[1].enabled,true);
    assert.equal(calls.frames[2].amplitude,10);
    assert.equal(calls.frames[2].speed,0);
    assert.equal(calls.frames[4].speed,0.5);
    assert.equal(calls.frames[6].amplitude,0);
    assert.equal(calls.removed,3); assert.equal(calls.closed,0); assert.equal(calls.dialogs,0);
}
{
    const {app,calls,capture}=run({unknownDirty:true,revision:1});
    assert.equal(app.exitCode,0,'only a pristine revision-1 project may replace an unavailable dirty attribute');
    assert.equal(calls.frames.length,10);
    assert.equal(capture.status,'CAPTURED');
    assert.equal(capture.guard,'DIRTY_UNAVAILABLE');
    assert.equal(capture.project_revision,'1');
}
// Publication is checked externally after JSX returns, not via cached File data.
assert.equal(run({noOutput:true}).capture.status,'CAPTURED');
for (const options of [{missingParameter:true},{cornerUnavailable:true},{cleanupError:true},{writeFailure:true},{staleFrame:true}]) {
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
console.log('PASS: 21 smoke capture/ownership/error cases (mock control flow, not AE execution)');
