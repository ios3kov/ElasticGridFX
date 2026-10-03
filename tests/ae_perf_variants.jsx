// Resolve both legacy flat controls and controls inside native UI groups.
function egfxWaveParam(root, name) {
    var direct = root.property(name);
    if (direct !== null) return direct;
    for (var i = 1; i <= root.numProperties; i++) {
        var child = root.property(i);
        if (child !== null && child.numProperties > 0) {
            var found = egfxWaveParam(child, name);
            if (found !== null) return found;
        }
    }
    return null;
}
// Fresh owned phase variants; every pair uses the same saved AEP for both builds.
function egfxPerfVariants(config) {
    var p=app.project,owned=null,records=[],status='FAIL',stage='guard';
    if(!/^[a-f0-9]{32}$/.test(config.run_id)||!p||p.file||p.numItems!==0||p.dirty!==false)throw Error('Clean empty project required');
    if(config.variants.length!==6||!(new File(config.source)).exists)throw Error('Pinned fixture required');
    if(!(new Folder(config.folder)).exists||(new File(config.folder+'/variants.json')).exists)throw Error('Fresh evidence required');
    for(var vi=0;vi<config.variants.length;vi++){
        var variant=config.variants[vi];
        if(typeof variant.phase!=='number'||!isFinite(variant.phase)||variant.phase<0||variant.phase>360||variant.phase===35)throw Error('Fresh finite phase required');
        for(var prior=0;prior<vi;prior++)if(variant.phase===config.variants[prior].phase)throw Error('Distinct phases required');
    }
    app.exitCode=94;
    try {
        for(var n=0;n<config.variants.length;n++){
            var v=config.variants[n],dest=new File(v.project);
            if(dest.exists||!(new Folder(v.folder)).exists)throw Error('Fresh destination required');
            if(app.project.file||app.project.numItems!==0||app.project.dirty!==false)throw Error('Project changed');
            stage='open';app.open(new File(config.source));var opened=app.project;
            if(!opened||!opened.file||opened.file.fsName!==(new File(config.source)).fsName||opened.numItems!==2||opened.renderQueue.numItems!==1)throw Error('Opened scene differs');
            owned=opened;
            var c=null;for(var i=1;i<=owned.numItems;i++){var item=owned.item(i);if(item.name==='EGFX_PERF'&&item.layers)c=item;}
            if(!c||c.comment!==config.source_owner||c.numLayers!==1||c.width!==1920||c.height!==1080||c.frameRate!==30||c.duration!==2||c.resolutionFactor[0]!==1||c.resolutionFactor[1]!==1)throw Error('Owned geometry differs');
            var l=c.layer(1),parade=l.property('ADBE Effect Parade');
            if(!l.source||!l.source.file||l.source.file.fsName!==(new File(config.pattern)).fsName||parade.numProperties!==1)throw Error('Owned source/effect differs');
            var e=parade.property(1);if(e.matchName!=='com.elasticgrid.fx.warp')throw Error('Effect differs');
            var names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
            for(var h=0;h<5;h++){var stream=e.property(names[h]);if(!stream||stream.name!==names[h]||!stream.expressionEnabled||stream.expressionError!=='')throw Error('Binding unready');}
            if(e.property('__FSTR Plane Kind').value!==1||e.property('Deformation Plane').value!==2||e.property('Render Quality').value!==2||egfxWaveParam(e, 'Wave Phase').value!==35||owned.bitsPerChannel!==32||owned.linearizeWorkingSpace!==false||(owned.workingSpace!==''&&owned.workingSpace!=='None'))throw Error('Pinned render state differs');
            stage='phase';egfxWaveParam(e, 'Wave Phase').setValue(v.phase);var actual=egfxWaveParam(e, 'Wave Phase').value;
            if(Math.abs(actual-v.phase)>0.00001)throw Error('Phase not applied');
            c.comment='__EGFX_PERF_VARIANT_'+config.run_id+'_'+n;
            owned.renderQueue.item(1).outputModule(1).file=new File(v.folder+'/fixture-output/frame_[#####].png');
            stage='save';owned.save(dest);if(!dest.exists||dest.length<=0||!owned.file||owned.file.fsName!==dest.fsName)throw Error('Save failed');
            records.push('{"index":'+n+',"requested_phase":'+v.phase+',"actual_phase":'+actual+',"binding_ready":true}');
            if(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)===false)throw Error('Close failed');owned=null;app.newProject();
        }
        status='PREPARED';stage='prepared';
    } finally {
        if(owned!==null&&app.project===owned){if(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)!==false)app.newProject();}
        var report=new File(config.folder+'/variants.json');report.encoding='UTF-8';
        if(report.exists||!report.open('w'))status='FAIL';else{report.write('{"run_id":"'+config.run_id+'","status":"'+status+'","stage":"'+stage+'","variants":['+records.join(',')+']}');report.close();}
        app.exitCode=status==='PREPARED'?0:94;
    }
}
