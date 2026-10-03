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
// Owned-only native acceptance. config.root must be a fresh output directory.
// Creation and capture are separate host turns so the documented idle hook runs.
function fstrCreateFinal(config) {
    var p=app.project;
    if(!p||p.file||p.numItems!==0||p.dirty)throw Error('Requires clean empty project');
    var c=p.items.addComp('FSTR Stretch Acceptance',640,480,1,2,30);
    c.comment=config.owner;
    var l=c.layers.addText('GRID\rPLANE');l.name='Native text';
    var t=l.property('ADBE Text Properties').property('ADBE Text Document'),d=t.value;
    d.fontSize=100;d.autoLeading=false;d.leading=110;d.justification=ParagraphJustification.CENTER_JUSTIFY;t.setValue(d);
    l.transform.position.setValue([320,200]);l.threeDLayer=true;
    l.transform.xRotation.setValue(-20);l.transform.yRotation.setValue(30);
    var e=l.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    e.property('Grid Positions').addKey(0);e.property('Grid Positions').addKey(1);
    c.openInViewer();l.selected=false;
}
function fstrCaptureFinal(config) {
    var p=app.project,c=p.activeItem;
    if(!(c instanceof CompItem)||c.comment!==config.owner||c.numLayers!==1||p.renderQueue.numItems!==0)throw Error('Ownership');
    var l=c.layer(1),e=l.property('ADBE Effect Parade').property(1);
    var hidden=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL','__FSTR Plane Kind'];
    for(var i=0;i<hidden.length;i++)if(!e.property(hidden[i]).expressionEnabled||e.property(hidden[i]).expressionError!=='')throw Error('Automatic binding');
    if(e.property('__FSTR Plane Kind').value!==2||e.property('Grid Positions').numKeys!==2)throw Error('Kind/keyframes');
    p.workingSpace='';p.linearizeWorkingSpace=false;
    var root=new Folder(config.root);if(!root.exists)throw Error('Missing fresh output directory');
    function capture(folder,name) {
        var q=p.renderQueue.items.add(c);
        try {
            var o=q.outputModule(1);o.applyTemplate('_HIDDEN X-Factor 16');o=q.outputModule(1);
            var s=o.getSettings(GetSettingsFormat.STRING);
            if(s.Format!=='PNG Sequence'||s.Depth!=='Trillions of Colors+'||s.Color!=='Straight (Unmatted)'||s.Resize!=='false'||s.Crop!=='false')throw Error('Output contract');
            o.file=new File(folder+'/'+name+'-[#####].png');
            q.setSettings({'Quality':'Best','Resolution':'Full','Color Depth':'Current Settings','Effects':'Current Settings'});
            q.timeSpanStart=0;q.timeSpanDuration=c.frameDuration;p.renderQueue.render();
            if(q.status!==RQItemStatus.DONE)throw Error('Render failed '+name);
            var f=new File(folder+'/'+name+'-00000.png');if(!f.exists||!f.rename(name+'.png'))throw Error('Missing output '+name);
        } finally {q.remove();}
    }
    var full=[[0,0],[640,0],[640,480],[0,480]],custom=[[30,50],[590,15],[620,425],[65,460]],records=[];
    app.beginSuppressDialogs();
    try {
        for(var di=0;di<3;di++) {
            var depth=[8,16,32][di],folder=config.root+'/d'+depth,dir=new Folder(folder);
            if(dir.exists||!dir.create())throw Error('Stale directory');p.bitsPerChannel=depth;
            e.property(1).setValue(1);egfxWaveParam(e, 'Wave Amplitude').setValue(0);e.enabled=false;capture(folder,'original');
            e.enabled=true;capture(folder,'neutral');egfxWaveParam(e, 'Wave Amplitude').setValue(15);capture(folder,'layer-wave');
            for(var j=0;j<4;j++)e.property(j+2).setValue(full[j]);e.property(1).setValue(2);capture(folder,'corners-wave');
            for(var j=0;j<4;j++)e.property(j+2).setValue(custom[j]);egfxWaveParam(e, 'Wave Amplitude').setValue(0);capture(folder,'custom-neutral');
            egfxWaveParam(e, 'Wave Amplitude').setValue(15);capture(folder,'custom-wave');
            var points=[];for(var j=0;j<4;j++)points.push(e.property(['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL'][j]).value);
            var r=new File(folder+'/quad.txt');if(!r.open('w'))throw Error('Report');r.write(points.toSource());r.close();
            records.push('d'+depth+' captured');
        }
        p.bitsPerChannel=8;e.property(1).setValue(1);egfxWaveParam(e, 'Wave Amplitude').setValue(0);
        l.threeDLayer=false;if(e.property('__FSTR Plane Kind').value!==1)throw Error('2D transition');
        l.threeDLayer=true;if(e.property('__FSTR Plane Kind').value!==2)throw Error('3D transition');
        if(e.property('Grid Positions').numKeys!==2)throw Error('Lost keys');
        var target=new File(config.root+'/acceptance.aep');if(target.exists)throw Error('Existing project');p.save(target);
        return records.join('; ')+'; transitions and 2 keys PASS';
    } finally {app.endSuppressDialogs(false);}
}
