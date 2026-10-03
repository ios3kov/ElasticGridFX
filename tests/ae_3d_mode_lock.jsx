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
// Execute create, then capture on separate host turns (automatic idle binding).
function fstrLockCreate(root, source) {
    if(app.project.file||app.project.numItems||app.project.dirty)throw Error('Requires empty project');
    app.open(new File(source));
    var c=app.project.item(1);
    if(!(c instanceof CompItem)||c.comment!=='FSTR_FINAL_cbe6da1'||c.numLayers!==1)throw Error('Owned source only');
    c.comment='FSTR_3D_LOCK';c.openInViewer();
}
function fstrLockCapture(root) {
    var p=app.project,c=p.activeItem;
    if(!(c instanceof CompItem)||c.comment!=='FSTR_3D_LOCK'||c.numLayers!==1)throw Error('Ownership');
    var l=c.layer(1),e=l.property('ADBE Effect Parade').property(1);
    if(e.property('__FSTR Plane Kind').expression.indexOf('native plane v2')<0)throw Error('Old binding not migrated');
    var keys=e.property('Grid Positions').numKeys;
    var corners=['Plane Top Left','Plane Top Right','Plane Bottom Right','Plane Bottom Left'];
    var values=[[80,60],[560,35],[590,410],[65,430]];
    for(var i=0;i<4;i++)e.property(corners[i]).setValue(values[i]);
    egfxWaveParam(e, 'Wave Amplitude').setValue(15);
    function snap(name){c.saveFrameToPng(0,new File(root+'/'+name+'.png'));}
    l.threeDLayer=true;l.transform.xRotation.setValue(-20);l.transform.yRotation.setValue(30);
    for(var d=0;d<3;d++){p.bitsPerChannel=[8,16,32][d];
        e.property('Deformation Plane').setValue(1);snap('text-'+p.bitsPerChannel+'-layer');
        e.property('Deformation Plane').setValue(2);snap('text-'+p.bitsPerChannel+'-corners');
    }
    p.bitsPerChannel=8;l.threeDLayer=false;
    if(e.property('__FSTR Plane Kind').value!==1||e.property('Deformation Plane').value!==2)throw Error('2D restore');
    snap('text-2d-corners');e.property('Deformation Plane').setValue(1);snap('text-2d-layer');
    e.property('Deformation Plane').setValue(2);l.threeDLayer=true;
    if(e.property('__FSTR Plane Kind').value!==2||e.property('Grid Positions').numKeys!==keys)throw Error('Transition or keys');
    for(var j=0;j<4;j++){var v=e.property(corners[j]).value;if(v[0]!==values[j][0]||v[1]!==values[j][1])throw Error('Corner lost');}
    var f=new File(root+'/mode-lock.aep');if(f.exists)throw Error('Existing output');p.save(f);
    l.selected=true;e.selected=true;
}
