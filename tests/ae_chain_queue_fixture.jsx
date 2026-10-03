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
// Owned-only queue capture; separate host turns prove native idle binding.
// This complements the retained historical saveFrameToPng smoke evidence.
function egfxCreateChainQueue(config) {
    var p=app.project;
    if(!/^[a-f0-9]{32}$/.test(config.run_id)||!p||p.file||p.numItems!==0||p.dirty!==false)throw Error('Requires clean empty project');
    var root=new Folder(config.folder),input=new File(config.folder+'/pattern.png');
    if(!root.exists||!input.exists||input.length<=0||(new File(config.folder+'/chain-queue.json')).exists)throw Error('Fresh fixture required');
    p.workingSpace='';p.linearizeWorkingSpace=false;p.bitsPerChannel=32;
    var footage=p.importFile(new ImportOptions(input));
    function comp(name){var c=p.items.addComp(name,319,241,1,2,30);c.comment='__EGFX_CHAIN_QUEUE_'+config.run_id;c.resolutionFactor=[1,1];return c;}
    var direct=comp('EGFX_QUEUE_DIRECT'),layer=direct.layers.add(footage);
    layer.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    var chain=comp('EGFX_QUEUE_CHAIN');chain.layers.add(footage);
    var adjustment=chain.layers.addSolid([0,0,0],'EGFX_QUEUE_ADJUSTMENT',319,241,1,2);adjustment.adjustmentLayer=true;
    adjustment.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    chain.openInViewer();adjustment.selected=false;
    app.exitCode=0;
}
function egfxCaptureChainQueue(config) {
    var owned=null,records=[],status='FAIL',suppressed=false,bindingReady=false,stage='guard',errorNumber=null,errorLine=null;
    var report=new File(config.folder+'/chain-queue.json');
    if(report.exists)throw Error('Stale evidence');
    function check(v,message){if(!v)throw Error(message);}
    function own(){check(owned!==null&&app.project===owned,'Foreign project');}
    function comp(name){for(var i=1;i<=app.project.numItems;i++){var item=app.project.item(i);if(item.name===name&&item.layers)return item;}return null;}
    function verified(c,n){check(c&&c.comment==='__EGFX_CHAIN_QUEUE_'+config.run_id&&c.numLayers===n&&c.width===319&&c.height===241&&c.duration===2&&c.frameRate===30&&c.resolutionFactor[0]===1&&c.resolutionFactor[1]===1,'Structure differs');}
    function effect(l){var parade=l.property('ADBE Effect Parade');check(parade&&parade.numProperties===1,'Effect count differs');var e=parade.property(1);check(e.matchName==='com.elasticgrid.fx.warp','Effect differs');
        var names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
        for(var i=0;i<5;i++){var h=e.property(names[i]);check(h&&h.name===names[i]&&h.expressionEnabled&&h.expressionError==='','Binding unready');}
        check(e.property('__FSTR Plane Kind').value===1,'Plane kind differs');return e;}
    function defaults(e){e.property('Columns').setValue(4);e.property('Rows').setValue(4);e.property('Render Quality').setValue(2);e.property('Edge Behavior').setValue(1);egfxWaveParam(e, 'Wave Axis').setValue(1);egfxWaveParam(e, 'Wave Frequency').setValue(1.3);egfxWaveParam(e, 'Wave Phase').setValue(35);e.property('Stretch Easing').setValue(0);egfxWaveParam(e, 'Wave Speed').setValue(0);egfxWaveParam(e, 'Wave Amplitude').setValue(0);}
    function frame(c,name,time){
        stage='frame_'+name;own();check(owned.renderQueue.numItems===0,'Unexpected queue');
        var folder=new Folder(config.folder+'/'+name);check(!folder.exists&&folder.create(),'Stale frame directory');
        var q=owned.renderQueue.items.add(c);
        try {
            q.applyTemplate('Best Settings');var o=q.outputModule(1);o.applyTemplate('_HIDDEN X-Factor 16');o=q.outputModule(1);
            var s=o.getSettings(GetSettingsFormat.STRING);
            check(s.Format==='PNG Sequence'&&s.Channels==='RGB + Alpha'&&s.Depth==='Trillions of Colors+'&&s.Color==='Straight (Unmatted)'&&s.Resize==='false'&&s.Crop==='false','Output contract');
            o.file=new File(folder.fsName+'/frame_[#####].png');
            q.setSettings({'Quality':'Best','Resolution':'Full','Color Depth':'Current Settings','Effects':'Current Settings'});
            var rs=q.getSettings(GetSettingsFormat.STRING);check(rs.Resolution==='Full'&&rs['Color Depth']==='Current Settings','Render contract');
            q.timeSpanStart=time;q.timeSpanDuration=c.frameDuration;q.skipFrames=0;q.render=true;
            var actual=q.timeSpanStart;own();owned.renderQueue.render();own();check(q.status===RQItemStatus.DONE,'Queue failed');
            var files=folder.getFiles('frame_*.png');check(files.length===1&&files[0] instanceof File&&files[0].length>0,'Exactly one frame required');
            var source=files[0].fsName,destination=config.folder+'/'+name+'.png';check(!(new File(destination)).exists&&files[0].copy(destination),'Copy failed');
            records.push({name:name,requested_time:time,queue_time:actual,project_bit_depth:owned.bitsPerChannel,output_bit_depth:16});
        } finally {if(app.project===owned)q.remove();}
    }
    app.exitCode=93;
    try {
        check(/^[a-f0-9]{32}$/.test(config.run_id)&&app.project&&!app.project.file&&app.project.numItems===5&&app.project.renderQueue.numItems===0,'Owned scene required');
        var direct=comp('EGFX_QUEUE_DIRECT'),chain=comp('EGFX_QUEUE_CHAIN');verified(direct,1);verified(chain,2);
        var layer=direct.layer(1),adjustment=chain.layer(1),source=chain.layer(2);
        var input=new File(config.folder+'/pattern.png');check(layer.source&&layer.source.file&&layer.source.file.fsName===input.fsName&&source.source&&source.source.file&&source.source.file.fsName===input.fsName&&adjustment.adjustmentLayer===true,'Owned sources required');
        check(app.project.bitsPerChannel===32&&(app.project.workingSpace===''||app.project.workingSpace==='None')&&app.project.linearizeWorkingSpace===false,'Color/depth differs');
        var fx=effect(layer),chainFx=effect(adjustment);owned=app.project;bindingReady=true;app.beginSuppressDialogs();suppressed=true;
        defaults(fx);fx.enabled=false;frame(direct,'bypass',0.25);fx.enabled=true;frame(direct,'identity',0.25);
        egfxWaveParam(fx, 'Wave Amplitude').setValue(10);frame(direct,'static_a',0.25);frame(direct,'static_b',0.75);
        egfxWaveParam(fx, 'Wave Speed').setValue(0.5);frame(direct,'animated_a',0.125);frame(direct,'animated_b',0.625);
        egfxWaveParam(fx, 'Wave Amplitude').setValue(0);frame(direct,'reset',0.25);
        defaults(chainFx);egfxWaveParam(chainFx, 'Wave Amplitude').setValue(10);frame(chain,'chain_before_corner',0.25);
        var corner=adjustment.property('ADBE Effect Parade').addProperty('ADBE Corner Pin');check(corner&&corner.matchName==='ADBE Corner Pin','Corner unavailable');frame(chain,'chain_corner_identity',0.25);
        corner.property(1).setValue([20,15]);corner.property(2).setValue([299,25]);corner.property(3).setValue([10,220]);corner.property(4).setValue([309,225]);frame(chain,'chain_corner_moved',0.25);
        status='CAPTURED';stage='captured';
    } catch(error) {
        status='FAIL';errorNumber=typeof error.number==='number'?error.number:null;errorLine=typeof error.line==='number'?error.line:null;
    } finally {
        if(owned!==null){if(app.project!==owned)status='FAIL';else if(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)===false)status='FAIL';else app.newProject();}
        if(suppressed)app.endSuppressDialogs(false);
        var parts=[];for(var ri=0;ri<records.length;ri++){var r=records[ri];parts.push('{"name":"'+r.name+'","requested_time":'+r.requested_time+',"queue_time":'+r.queue_time+',"project_bit_depth":'+r.project_bit_depth+',"output_bit_depth":16}');}
        report=new File(config.folder+'/chain-queue.json');report.encoding='UTF-8';
        if(report.open('w')){report.write('{"run_id":"'+config.run_id+'","status":"'+status+'","frames":['+parts.join(',')+'],"binding_ready":'+bindingReady+',"stage":"'+stage+'","error_number":'+errorNumber+',"error_line":'+errorLine+',"scope":"owned queue fixture; pixel assertions external"}');report.close();}else status='FAIL';
        app.exitCode=status==='CAPTURED'?0:93;
    }
}
