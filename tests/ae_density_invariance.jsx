// Load ae_runtime_smoke.jsx for its strict empty-project ownership guard.
// One kind/depth per test-owned session. Loaded Build ID is a separate gate.
function fstrDensityInvariance(config) {
    if (!config || typeof config.run_id !== 'string' || !/^[a-f0-9]{32}$/.test(config.run_id)) throw Error('Invalid run id');
    if (config.depth !== 8 && config.depth !== 16 && config.depth !== 32) throw Error('Invalid depth');
    if (config.kind !== 'text' && config.kind !== 'checker_precomp') throw Error('Invalid kind');
    if (typeof elasticGridCurrentProjectState !== 'function' || typeof elasticGridHasTestProjectOwnership !== 'function') throw Error('Load project guard');
    var state=elasticGridCurrentProjectState();
    if (!elasticGridHasTestProjectOwnership(state.guard,state.project_revision)) throw Error('Requires clean empty unsaved project');
    var root=new Folder(config.folder);
    if (!root.exists || root.name!=='EGFX-density-'+config.run_id || root.getFiles().length!==0) throw Error('Fresh run folder required');
    var p=app.project;
    if (p.renderQueue.numItems!==0) throw Error('Existing queue');
    p.bitsPerChannel=config.depth; p.workingSpace=''; p.linearizeWorkingSpace=false;
    var c=p.items.addComp('__EGFX_DENSITY_'+config.run_id,640,480,1,2,25),l;
    if (config.kind==='text') { l=c.layers.addText('FSTR DENSITY'); }
    else {
        l=c.layers.addSolid([0.3,0.6,0.9],'Density checker',640,480,1,2);
        l.property('ADBE Effect Parade').addProperty('ADBE Checkerboard');
        c.layers.precompose([l.index],'Density source',true); l=c.layer(1);
    }
    var e=l.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    if (!e) throw Error('Effect unavailable');
    var grid=e.property('Grid Positions'),cols=e.property('Columns'),rows=e.property('Rows');
    if (cols.canVaryOverTime || rows.canVaryOverTime || !grid.canVaryOverTime) throw Error('Animation flags');
    grid.addKey(0); grid.addKey(1);
    // Real changing frames without trying to fabricate AE's CUSTOM_VALUE bytes.
    // Nonuniform Grid Positions animation is additionally tested via Rust/FFI
    // and must be exercised in a manually authored native fixture.
    e.property('Wave Amplitude').setValue(15); e.property('Wave Speed').setValue(1);
    c.openInViewer();
    function verifyKeys() {
        if (grid.numKeys!==2 || grid.keyTime(1)!==0 || grid.keyTime(2)!==1 || cols.numKeys!==0 || rows.numKeys!==0)
            throw Error('Count change mutated key topology');
    }
    function capture(label,time) {
        var item=p.renderQueue.items.add(c);
        try {
            var output=item.outputModule(1); output.applyTemplate('_HIDDEN X-Factor 16'); output=item.outputModule(1);
            var s=output.getSettings(GetSettingsFormat.STRING);
            if (s.Format!=='PNG Sequence'||s.Depth!=='Trillions of Colors+'||s.Color!=='Straight (Unmatted)'||s.Resize!=='false'||s.Crop!=='false') throw Error('Output contract');
            output.file=new File(root.fsName+'/'+label+'-[#####].png');
            item.setSettings({'Quality':'Best','Resolution':'Full','Color Depth':'Current Settings','Effects':'Current Settings'});
            item.timeSpanStart=time; item.timeSpanDuration=c.frameDuration; p.renderQueue.render();
            if (item.status!==RQItemStatus.DONE) throw Error('Capture failed '+label);
        } finally { item.remove(); }
    }
    // Run capture in a LATER host turn, after automatic plane initialization.
    // This function only prepares state; it never pretends that setup is PASS.
    function run() {
        if (app.project!==p || p.activeItem!==c || p.renderQueue.numItems!==0) throw Error('Owned context changed');
        var counts=[[1,1],[9,2],[50,50],[4,4]],times=[0,0.48,1];
        for(var ti=0;ti<times.length;ti++) {
            c.time=times[ti]; cols.setValue(4);rows.setValue(4);verifyKeys();capture('t'+ti+'-baseline',times[ti]);
            for(var ci=0;ci<counts.length;ci++) {
                cols.setValue(counts[ci][0]);rows.setValue(counts[ci][1]);verifyKeys();capture('t'+ti+'-c'+ci,times[ti]);
            }
        }
        var target=new File(root.fsName+'/density.aep'); if(target.exists)throw Error('Existing project');p.save(target);
        return 'CAPTURED_NOT_FULL_ACCEPTANCE: compare decoded pairs; verify loaded identity and real Grid Positions animation separately';
    }
    return run;
}
