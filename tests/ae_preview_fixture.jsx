// Ordinary GUI preview setup/readback only; completion is observed through UI.
function egfxPreviewFixture(config) {
    app.exitCode=95;
    if(!/^[a-f0-9]{32}$/.test(config.run_id))throw Error('Invalid owner');
    var report=new File(config.folder+'/'+config.record+'.json');
    if(!/^[a-z0-9-]{1,64}$/.test(config.record)||report.exists||!(new Folder(config.folder)).exists)throw Error('Fresh evidence required');
    if(config.action==='open'){
        var p=app.project;
        if(!p||p.file||p.numItems!==0||p.dirty!==false)throw Error('Clean empty project required');
        app.open(new File(config.source));
    }
    var p=app.project;
    if(!p||!p.file||p.file.fsName!==(new File(config.source)).fsName||p.numItems!==2)throw Error('Owned scene required');
    var c=null;for(var i=1;i<=p.numItems;i++){var item=p.item(i);if(item.name==='EGFX_PERF'&&item.layers)c=item;}
    var owner='__EGFX_PREVIEW_'+config.run_id;
    if(!c||c.comment!==(config.action==='open'?config.source_owner:owner)||c.numLayers!==1||c.width!==1920||c.height!==1080||c.duration!==2||c.frameRate!==30)throw Error('Owned geometry differs');
    var l=c.layer(1),parade=l.property('ADBE Effect Parade');
    if(!l.source||!l.source.file||l.source.file.fsName!==(new File(config.pattern)).fsName||parade.numProperties!==1)throw Error('Owned source differs');
    var e=parade.property(1);if(e.matchName!=='com.elasticgrid.fx.warp')throw Error('Effect differs');
    var names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
    for(var h=0;h<5;h++){var stream=e.property(24+h);if(!stream||stream.name!==names[h]||!stream.expressionEnabled||stream.expressionError!=='')throw Error('Binding unready');}
    if(e.property(28).value!==1||p.bitsPerChannel!==32||(p.workingSpace!==''&&p.workingSpace!=='None')||p.linearizeWorkingSpace!==false||e.property('Render Quality').value!==2||e.property('Deformation Plane').value!==2)throw Error('Pinned quality differs');
    if(config.action!=='open'&&config.action!=='read'&&config.action!=='invalidate'&&config.action!=='cleanup')throw Error('Unsupported action');
    var v=c.openInViewer();if(!v||v.type!==ViewerType.VIEWER_COMPOSITION)throw Error('Composition viewer required');
    var options=v.views[v.activeViewIndex].options;
    var fastNames=['FP_OFF','FP_ADAPTIVE_RESOLUTION','FP_DRAFT','FP_FAST_DRAFT','FP_WIREFRAME'],previousFast=null;
    for(var n=0;n<fastNames.length;n++)if(options.fastPreview===FastPreviewType[fastNames[n]])previousFast=fastNames[n];
    if(previousFast===null)throw Error('Unknown Fast Preview');
    if(config.action==='open'){
        c.comment=owner;c.resolutionFactor=[1,1];c.workAreaStart=0;c.workAreaDuration=2;c.time=0;
        options.fastPreview=FastPreviewType.FP_OFF;l.selected=true;e.selected=true;
    }
    var invalidationStart=null,invalidationEnd=null;
    if(config.action==='invalidate'){
        if(typeof config.phase!=='number'||!isFinite(config.phase)||config.phase<120||config.phase>350||c.resolutionFactor[0]!==1||c.resolutionFactor[1]!==1||options.fastPreview!==FastPreviewType.FP_OFF)throw Error('Invalidation contract');
        invalidationStart=(new Date()).getTime();
        e.property('Wave Phase').setValue(config.phase);
        if(Math.abs(e.property('Wave Phase').value-config.phase)>0.00001)throw Error('Phase not applied');
        c.time=0;
        invalidationEnd=(new Date()).getTime();
    }
    var ready=options.fastPreview===FastPreviewType.FP_OFF&&c.resolutionFactor[0]===1&&c.resolutionFactor[1]===1;
    var body='{"run_id":"'+config.run_id+'","action":"'+config.action+'","binding_ready":true,"full_final":'+ready+',"previous_fast":"'+previousFast+'","bit_depth":'+p.bitsPerChannel+',"width":'+c.width+',"height":'+c.height+',"fps":'+c.frameRate+',"duration":'+c.duration+',"work_area_start":'+c.workAreaStart+',"work_area_duration":'+c.workAreaDuration+',"phase":'+e.property('Wave Phase').value+',"columns":'+e.property('Columns').value+',"rows":'+e.property('Rows').value+',"wave_speed":'+e.property('Wave Speed').value+',"memory_in_use":'+app.memoryInUse+',"scope":"setup/readback only; UI proves preview completion"}';
    // Wall-clock bounds bracket the parameter operation, not rendering/display.
    body=body.substr(0,body.length-1)+',"invalidation_start_utc_ms":'+invalidationStart+',"invalidation_end_utc_ms":'+invalidationEnd+'}';
    if(config.action==='cleanup'){
        if(!/^(FP_OFF|FP_ADAPTIVE_RESOLUTION|FP_DRAFT|FP_FAST_DRAFT|FP_WIREFRAME)$/.test(config.previous_fast))throw Error('Restore mode missing');
        options.fastPreview=FastPreviewType[config.previous_fast];
        if(options.fastPreview!==FastPreviewType[config.previous_fast])throw Error('Restore failed');
        if(p.close(CloseOptions.DO_NOT_SAVE_CHANGES)===false)throw Error('Close failed');app.newProject();
    }
    report=new File(config.folder+'/'+config.record+'.json');report.encoding='UTF-8';
    if(!report.open('w'))throw Error('Report unavailable');report.write(body);report.close();
    app.exitCode=0;
}
