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
// Opens only a copied historical test-owned AEP; preserves original evidence.
function elasticGridPlaneMigrationProbe(config) {
    var owned=null,status="FAIL",message="",clean=false;
    function check(v,m){if(!v)throw new Error(m);}
    function q(s){return '"'+String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n")+'"';}
    function validate(project) {
        var comp=null;
        for(var i=1;i<=project.numItems;i++)if(project.item(i).name===config.comp_name)comp=project.item(i);
        check(comp!==null,"Historical comp missing");
        var fx=comp.layer(1).property("ADBE Effect Parade").property("com.elasticgrid.fx.warp");
        check(fx!==null,"Historical effect missing");
        check(fx.property("Deformation Plane").value===2,"Plane mode changed");
        check(Math.abs(egfxWaveParam(fx, "Wave Amplitude").value-8)<0.001,"Wave changed");
        var names=["Top Left","Top Right","Bottom Right","Bottom Left"];
        var expected=[[10,5],[120,0],[127,85],[0,95]];
        for(var n=0;n<4;n++) {
            var p=fx.property(names[n]);check(p!==null,"Missing corner");
            check(p.numKeys===0,"Unexpected historical keyframes");
            var value=p.value;
            check(Math.abs(value[0]-expected[n][0])<0.001 && Math.abs(value[1]-expected[n][1])<0.001,"Corner changed");
        }
        check(fx.property("Columns").value===4 && fx.property("Rows").value===4,"Grid dimensions changed");
        check(fx.property("Render Quality").value===2,"Quality changed");
    }
    var dir=new Folder(config.folder),report=new File(dir.fsName+"/migration.json");
    check(/^[a-f0-9]{32}$/.test(config.run_id) && dir.exists && !report.exists,"Invalid/stale run");
    try {
        check(app.project && app.project.file===null && app.project.numItems===0 && app.project.dirty===false,"Requires clean empty project");
        var copy=new File(dir.fsName+"/legacy-copy.aep");check(copy.exists,"Missing copied fixture");
        owned=app.project;check(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES),"Empty project close failed");owned=null;
        owned=app.open(copy);check(owned && owned.file && owned.file.fsName===copy.fsName,"Copied project open failed");
        validate(owned);
        var migrated=new File(dir.fsName+"/migrated.aep");check(!migrated.exists,"Stale migrated project");
        owned.save(migrated);check(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES),"Owned close failed");owned=null;
        owned=app.open(migrated);check(owned && owned.file && owned.file.fsName===migrated.fsName,"Reopen failed");
        validate(owned);status="PASS";
    }catch(e){message=String(e);}
    finally{
        if(owned!==null && app.project===owned){owned.close(CloseOptions.DO_NOT_SAVE_CHANGES);app.newProject();}
        clean=app.project && app.project.file===null && app.project.numItems===0;
        if(!clean)status="FAIL";
        report.encoding="UTF-8";check(report.open("w"),"Cannot write report");
        report.write('{"status":'+q(status)+',"run_id":'+q(config.run_id)+',"cleanup_clean":'+String(clean)+',"scope":"historical static values; no legacy keyframes in fixture","message":'+q(message)+'}');report.close();
    }
    return status==="PASS"?0:1;
}
