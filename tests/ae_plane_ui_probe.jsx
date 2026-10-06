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
// Test-owned visual acceptance scene. Never opens/closes user work.
function elasticGridPlaneUIProbe(config) {
    var owned=null;
    function check(v,m){if(!v)throw new Error(m);}
    try {
        check(/^[a-f0-9]{32}$/.test(config.run_id),"Invalid run id");
        var dir=new Folder(config.folder);check(dir.exists,"Missing owned directory");
        var output=new File(dir.fsName+"/plane-project.aep");check(!output.exists,"Stale project");
        check(app.project && app.project.file===null && app.project.numItems===0 && app.project.dirty===false,"Requires clean empty project");
        owned=app.project;
        owned.workingSpace="";owned.linearizeWorkingSpace=false;owned.bitsPerChannel=16;
        var comp=owned.items.addComp("__EGFX_UI_"+config.run_id,640,480,1,1,30);
        var layer=comp.layers.addSolid([0.2,0.4,0.6],"__EGFX_TEST_PLANE",640,480,1);
        var fx=layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        check(fx!==null,"Effect missing");
        var names=["Top Left","Top Right","Bottom Right","Bottom Left"];
        var values=[[0,0],[639,0],[639,479],[0,479]];
        for(var i=0;i<4;i++)fx.property(names[i]).setValue(values[i]);
        fx.property("Deformation Plane").setValue(1);
        egfxWaveParam(fx, "Wave Amplitude").setValue(0);
        layer.threeDLayer=true;
        layer.property("ADBE Transform Group").property("ADBE Rotate Y").setValue(35);
        layer.property("ADBE Transform Group").property("ADBE Scale").setValue([75,75,75]);
        owned.save(output);
        comp.openInViewer();layer.selected=true;fx.selected=true;
        return 0;
    } catch(e) {
        if(owned!==null && app.project===owned){owned.close(CloseOptions.DO_NOT_SAVE_CHANGES);app.newProject();}
        throw e;
    }
}

function elasticGridPlaneUIStage(config) {
    var expected=new File(config.folder+"/plane-project.aep");
    if(!app.project || !app.project.file || app.project.file.fsName!==expected.fsName)throw new Error("Foreign project");
    var comp=app.project.activeItem;
    if(!comp || comp.name!=="__EGFX_UI_"+config.run_id)throw new Error("Foreign comp");
    var layer=comp.layer("__EGFX_TEST_PLANE");
    var fx=layer.property("ADBE Effect Parade").property("com.elasticgrid.fx.warp");
    if(config.stage==="four")fx.property("Deformation Plane").setValue(2);
    else if(config.stage==="camera") {
        var parent=comp.layers.addNull();parent.name="__EGFX_PARENT";parent.threeDLayer=true;
        layer.parent=parent;
        parent.property("ADBE Transform Group").property("ADBE Rotate Z").setValue(15);
        var camera=comp.layers.addCamera("__EGFX_CAMERA",[320,240]);
        var position=camera.property("ADBE Transform Group").property("ADBE Position");
        var p=position.value;position.setValue([p[0]+80,p[1]-35,p[2]]);
    } else throw new Error("Unknown stage");
    for(var i=1;i<=comp.numLayers;i++)comp.layer(i).selected=false;
    layer.selected=true;fx.selected=true;comp.openInViewer();
}
