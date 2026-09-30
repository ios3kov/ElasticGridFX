// Load ae_runtime_smoke.jsx first for the existing strict project-ownership guard.
// Run ONE case per clean host session to test first-after-launch; repeat in a
// separate owned session for later-instance acceptance. No auto-run or install.
function elasticGridFirstApplicationFixture(config) {
    if (!config || typeof config.run_id !== 'string' || !/^[a-f0-9]{32}$/.test(config.run_id))
        throw Error('Invalid run id');
    if (config.depth !== 8 && config.depth !== 16 && config.depth !== 32)
        throw Error('Invalid depth');
    if (config.kind !== 'solid' && config.kind !== 'text' && config.kind !== 'checker_precomp')
        throw Error('Invalid layer kind');
    if (typeof config.three_d !== 'boolean') throw Error('Explicit 3D setting required');
    if (typeof elasticGridCurrentProjectState !== 'function' ||
        typeof elasticGridHasTestProjectOwnership !== 'function') throw Error('Load project guard first');
    var state=elasticGridCurrentProjectState();
    if (!elasticGridHasTestProjectOwnership(state.guard,state.project_revision))
        throw Error('Requires a clean, empty, unsaved test project');
    var root=new Folder(config.folder);
    if (!root.exists || root.name !== 'EGFX-first-'+config.run_id || root.getFiles().length !== 0)
        throw Error('Fresh uniquely named empty run folder required');
    var p=app.project;
    if (p.renderQueue.numItems !== 0) throw Error('Existing render queue');
    function record(stage) {
        var file=new File(root.fsName+'/'+stage+'.txt');
        if (file.exists || !file.open('w')) throw Error('Report unavailable');
        try {file.write('run_id='+config.run_id+'\nstage='+stage+'\nae='+app.version+
            '\ndepth='+config.depth+'\nkind='+config.kind+'\nthree_d='+config.three_d+'\n');}
        finally {file.close();}
    }
    p.bitsPerChannel=config.depth; p.workingSpace=''; p.linearizeWorkingSpace=false;
    var name='__EGFX_FIRST_'+config.run_id;
    var comp=p.items.addComp(name,640,480,1.0,1.0,25.0), layer;
    if (config.kind==='text') {
        layer=comp.layers.addText('FSTR first application');
    } else {
        layer=comp.layers.addSolid([0.3,0.6,0.9],name+'_source',640,480,1.0,1.0);
        if (config.kind==='checker_precomp') {
            layer.property('ADBE Effect Parade').addProperty('ADBE Checkerboard');
            comp.layers.precompose([layer.index],name+'_precomp',true);
            layer=comp.layer(1);
        }
    }
    layer.threeDLayer=config.three_d;
    function capture(label) {
        var item=p.renderQueue.items.add(comp);
        try {
            var output=item.outputModule(1); output.applyTemplate('_HIDDEN X-Factor 16');
            output=item.outputModule(1);
            var settings=output.getSettings(GetSettingsFormat.STRING);
            if (settings.Format!=='PNG Sequence' || settings.Depth!=='Trillions of Colors+' ||
                settings.Color!=='Straight (Unmatted)' || settings.Resize!=='false' || settings.Crop!=='false')
                throw Error('PNG output contract unavailable');
            output.file=new File(root.fsName+'/'+label+'-[#####].png');
            item.setSettings({'Quality':'Best','Resolution':'Full','Color Depth':'Current Settings','Effects':'Current Settings'});
            item.timeSpanStart=0; item.timeSpanDuration=comp.frameDuration;
            p.renderQueue.render();
            if (item.status!==RQItemStatus.DONE) throw Error('Render failed: '+label);
            // A successful queue status is not by itself first-application PASS.
        } finally {item.remove();}
    }
    record('started'); capture('bypass'); comp.openInViewer();
    record('before-add');
    // Deliberately no suppression, scheduled delay or binding repair here.
    // Capture immediately in this script turn, before deferred idle can run.
    var effect=layer.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    if (!effect || effect.matchName!=='com.elasticgrid.fx.warp') throw Error('Effect unavailable');
    record('after-add'); capture('first-neutral');
    record('first-frame-captured');
    p.save(new File(root.fsName+'/first-application.aep'));
    // Caller must verify actual loaded identity, no initial dialog, decoded
    // bypass/neutral pixel equality and subsequent deformation AFTER binding.
    // Retain failure/timeout evidence and the owned project; never close AE here.
    return 'CAPTURED_NOT_FULL_ACCEPTANCE';
}
