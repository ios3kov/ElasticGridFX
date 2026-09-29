// Internal Stage 9 fixture. Run only after a clean candidate is installed and
// live identity is independently checked. Never closes or edits user work.
function elasticGridPlaneSmoke(config) {
    var owned=null, suppressing=false, report=null;
    var status="FAIL", stage="guard", message="", frames=[];
    function q(s) {return '"'+String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n")+'"';}
    function check(v,s) {if(!v) throw new Error(s);}
    try {
        check(/^[a-f0-9]{32}$/.test(config.run_id),"Invalid run id");
        var folder=new Folder(config.folder);
        check(folder.exists,"Missing workspace");
        report=new File(folder.fsName+"/plane-smoke.json");
        check(!report.exists,"Refusing stale report");
        check(app.project!==null && app.project.file===null && app.project.numItems===0,"Requires empty unsaved project");
        var dirty=app.project.dirty;
        check(dirty===false || (typeof dirty==="undefined" && app.project.revision===1),"Unsafe project state");
        owned=app.project;
        app.beginSuppressDialogs();suppressing=true;
        stage="fixture";
        var input=new File(folder.fsName+"/pattern.png");check(input.exists,"Missing fixture");
        var footage=owned.importFile(new ImportOptions(input));
        var comp=owned.items.addComp("__EGFX_PLANE_"+config.run_id,128,96,1,1,30);
        var layer=comp.layers.add(footage);
        var fx=layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        check(fx!==null,"Effect unavailable");
        function corners(points) {
            var names=["Plane Top Left","Plane Top Right","Plane Bottom Right","Plane Bottom Left"];
            for(var i=0;i<4;++i) {
                var property=fx.property(names[i]);check(property!==null,"Missing "+names[i]);
                property.setValue(points[i]);
            }
        }
        function capture(name) {
            check(app.project===owned,"Project ownership changed");
            var file=new File(folder.fsName+"/"+name+".png");check(!file.exists,"Stale frame");
            comp.saveFrameToPng(0,file);check(file.exists && file.length>0,"Missing frame "+name);
            frames.push(name);
        }
        var fit=[[0,0],[127,0],[127,95],[0,95]];
        corners(fit);
        for(var d=0;d<3;++d) {
            var depth=[8,16,32][d];owned.bitsPerChannel=depth;stage="pixels-"+depth;
            comp.resolutionFactor=[1,1];fx.property("Wave Amplitude").setValue(0);
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-original");
            fx.property("Deformation Plane").setValue(2);capture("d"+depth+"-identity");
            fx.property("Wave Amplitude").setValue(8);
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-legacy-wave");
            fx.property("Deformation Plane").setValue(2);capture("d"+depth+"-plane-wave");
            corners([[10,5],[120,0],[127,85],[0,95]]);capture("d"+depth+"-skew-wave");
            corners([[0,0],[127,95],[127,0],[0,95]]);capture("d"+depth+"-invalid");
            corners(fit);fx.property("Wave Amplitude").setValue(0);
            comp.resolutionFactor=[2,2];capture("d"+depth+"-half-identity");
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-half-original");
        }
        stage="save";owned.bitsPerChannel=8;comp.resolutionFactor=[1,1];
        fx.property("Deformation Plane").setValue(2);
        corners([[10,5],[120,0],[127,85],[0,95]]);
        fx.property("Wave Amplitude").setValue(8);
        var file=new File(folder.fsName+"/plane-project.aep");check(!file.exists,"Stale project");
        owned.save(file);check(file.exists,"Project not saved");
        // Leave this test-owned saved project open for independent identity and
        // interaction/roundtrip checks. User work was never accepted as input.
        comp.openInViewer();layer.selected=true;
        status="CAPTURED";stage="complete";
    } catch(error) {message=String(error);}
    finally {
        if(suppressing) {try{app.endSuppressDialogs(false);}catch(e){status="FAIL";}}
        if(report!==null && !report.exists) {
            report.encoding="UTF-8";
            if(report.open("w")) {
                var names=[];for(var j=0;j<frames.length;++j) names.push(q(frames[j]));
                report.write('{"run_id":'+q(config.run_id)+',"status":'+q(status)+',"stage":'+q(stage)+
                    ',"message":'+q(message)+',"frames":['+names.join(",")+']}');
                report.close();
            }
        }
    }
}
