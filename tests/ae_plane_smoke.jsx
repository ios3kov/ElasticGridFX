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
// Internal Stage 9 fixture. Run only after a clean candidate is installed and
// live identity is independently checked. Never closes or edits user work.
function elasticGridPlaneSmoke(config) {
    var owned=null, suppressing=false, report=null;
    var status="FAIL", stage="guard", message="", frames=[], colorBefore=null, colorApplied=null;
    function q(s) {return '"'+String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n")+'"';}
    function check(v,s) {if(!v) throw new Error(s);}
    function findComp(project,name) {
        for(var i=1;i<=project.numItems;++i) {
            var item=project.item(i);
            if(item && item.name===name && item.layers) return item;
        }
        return null;
    }
    function findEffect(layer) {
        var parade=layer.property("ADBE Effect Parade");
        if(!parade) return null;
        for(var i=1;i<=parade.numProperties;++i) {
            var candidate=parade.property(i);
            if(candidate && candidate.matchName==="com.elasticgrid.fx.warp") return candidate;
        }
        return null;
    }
    function safeResetOwned() {
        try {
            if(owned!==null && app.project===owned) {
                owned.close(CloseOptions.DO_NOT_SAVE_CHANGES);
                app.newProject();
            }
        } catch(e) {}
    }
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
        stage="color_context";
        colorBefore={space:String(owned.workingSpace),linearize:owned.linearizeWorkingSpace===true};
        // Only the owned empty fixture project; never application preferences.
        owned.workingSpace="";owned.linearizeWorkingSpace=false;
        check((owned.workingSpace==="" || owned.workingSpace==="None") && owned.linearizeWorkingSpace===false,
              "Fixture color state was not applied");
        colorApplied={space:String(owned.workingSpace),linearize:owned.linearizeWorkingSpace===true};
        stage="fixture";
        var input=new File(folder.fsName+"/pattern.png");check(input.exists,"Missing fixture");
        var footage=owned.importFile(new ImportOptions(input));
        var compName="__EGFX_PLANE_"+config.run_id;
        var comp=owned.items.addComp(compName,128,96,1,1,30);
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
            var path=folder.fsName+"/"+name+".png";
            var file=new File(path);check(!file.exists,"Stale frame");
            // AE 25.6 saveFrameToPng maps native white to ~0.1 at 32 bpc.
            // Render at the actual project depth; export straight 16-bit RGBA.
            // Never compensate pixels, lower thresholds or change project depth.
            check(owned.renderQueue.numItems===0,"Unexpected render queue items");
            var queue=owned.renderQueue.items.add(comp);
            try {
                var output=queue.outputModule(1);
                output.applyTemplate("_HIDDEN X-Factor 16");
                output=queue.outputModule(1);
                var settings=output.getSettings(GetSettingsFormat.STRING);
                check(settings.Format==="PNG Sequence" && settings.Channels==="RGB + Alpha" &&
                      settings.Depth==="Trillions of Colors+" && settings.Color==="Straight (Unmatted)" &&
                      settings.Resize==="false" && settings.Crop==="false","Unsafe PNG output template");
                var sequence=folder.fsName+"/"+name+"-00000.png";
                check(!(new File(sequence)).exists,"Stale sequence frame");
                output.file=new File(folder.fsName+"/"+name+"-[#####].png");
                queue.setSettings({"Quality":"Best","Resolution":comp.resolutionFactor[0]===2?"Half":"Full",
                                   "Color Depth":"Current Settings","Effects":"Current Settings"});
                var renderSettings=queue.getSettings(GetSettingsFormat.STRING);
                check(renderSettings.Resolution===(comp.resolutionFactor[0]===2?"Half":"Full") &&
                      renderSettings["Color Depth"]==="Current Settings","Render settings not applied");
                queue.timeSpanStart=0;queue.timeSpanDuration=comp.frameDuration;
                check(app.project===owned && owned.renderQueue.numItems===1,"Render ownership changed");
                owned.renderQueue.render();
                check(queue.status===RQItemStatus.DONE,"Render queue did not complete");
                var rendered=new File(sequence);
                check(rendered.exists && rendered.length>0,"Missing queue frame "+name);
                check(rendered.rename(name+".png"),"Cannot name queue frame");
            } finally {
                if(app.project===owned) queue.remove();
            }
            // Refresh ExtendScript File metadata after AE publishes the PNG.
            file=new File(path);
            // The filesystem entry/length may not immediately be
            // visible to ExtendScript. Match the proven Stage 7 smoke behavior:
            // bounded wait only, never retry the render or kill/restart AE.
            for(var attempt=0;(!file.exists || file.length<=0) && attempt<50;++attempt) {
                if(typeof $==="undefined" || typeof $.sleep!=="function") break;
                $.sleep(100);
                check(app.project===owned,"Project ownership changed");
                file=new File(path);
            }
            check(file.exists && file.length>0,"Missing frame "+name);
            frames.push(name);
        }
        var fit=[[0,0],[127,0],[127,95],[0,95]];
        var skew=[[10,5],[120,0],[127,85],[0,95]];
        corners(fit);
        for(var d=0;d<3;++d) {
            var depth=[8,16,32][d];owned.bitsPerChannel=depth;stage="pixels_"+depth;
            comp.resolutionFactor=[1,1];egfxWaveParam(fx, "Wave Amplitude").setValue(0);
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-original");
            fx.property("Deformation Plane").setValue(2);capture("d"+depth+"-identity");
            corners(skew);capture("d"+depth+"-skew-identity");corners(fit);
            // Diagnostic host reference, not an equality oracle: Adobe's
            // default source boundary convention can differ by one pixel.
            fx.enabled=false;
            var nativePin=layer.property("ADBE Effect Parade").addProperty("ADBE Corner Pin");
            check(nativePin!==null,"Native Corner Pin unavailable");
            nativePin.property(1).setValue(skew[0]);nativePin.property(2).setValue(skew[1]);
            nativePin.property(3).setValue(skew[3]);nativePin.property(4).setValue(skew[2]);
            capture("d"+depth+"-native-skew");
            nativePin.remove();fx=findEffect(layer);check(fx!==null,"Effect lost after reference capture");fx.enabled=true;
            egfxWaveParam(fx, "Wave Amplitude").setValue(8);
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-legacy-wave");
            fx.property("Deformation Plane").setValue(2);capture("d"+depth+"-plane-wave");
            corners(skew);capture("d"+depth+"-skew-wave");
            corners([[0,0],[127,95],[127,0],[0,95]]);capture("d"+depth+"-invalid");
            corners(fit);egfxWaveParam(fx, "Wave Amplitude").setValue(0);
            comp.resolutionFactor=[2,2];capture("d"+depth+"-half-identity");
            fx.property("Deformation Plane").setValue(1);capture("d"+depth+"-half-original");
        }

        // Roundtrip a non-trivial plane state through a real AEP before camera checks.
        stage="roundtrip_save";owned.bitsPerChannel=8;comp.resolutionFactor=[1,1];
        fx.property("Deformation Plane").setValue(2);corners(skew);
        egfxWaveParam(fx, "Wave Amplitude").setValue(8);
        capture("roundtrip-before");
        var projectFile=new File(folder.fsName+"/plane-project.aep");check(!projectFile.exists,"Stale project");
        owned.save(projectFile);check(projectFile.exists && owned.file!==null,"Project not saved");
        check(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES),"Could not close owned project");
        owned=app.open(projectFile);check(owned!==null && owned.file!==null && owned.file.fsName===projectFile.fsName,"Project reopen failed");
        comp=findComp(owned,compName);check(comp!==null,"Roundtrip composition missing");
        layer=comp.layer(1);check(layer!==null,"Roundtrip layer missing");
        fx=findEffect(layer);check(fx!==null,"Roundtrip effect missing");
        check(fx.property("Deformation Plane").value===2,"Plane mode did not roundtrip");
        check(Math.abs(egfxWaveParam(fx, "Wave Amplitude").value-8)<0.001,"Wave amplitude did not roundtrip");
        capture("roundtrip-after");

        // 3D/camera acceptance: independently exercise the Default Camera
        // fallback before any camera layer exists, then layer position, scale,
        // rotation, parenting, active-camera movement and camera switching.
        // Overlay/drag remains a separate real-UI gate.
        stage="three_d";
        layer.threeDLayer=true;
        var transform=layer.property("ADBE Transform Group");check(transform!==null,"Missing 3D transform");
        // The test owns this composition. Prove there is no explicit camera
        // layer by topology, not by CompItem.activeCamera: AE 25.6 on the target
        // host exposed a non-null camera-like value here despite no camera layer.
        check(comp.numLayers===1 && comp.layer(1)===layer,"Unexpected pre-camera layers");
        capture("3d-no-camera");

        var camera=comp.layers.addCamera("__EGFX_CAMERA_A_"+config.run_id,[64,48]);
        check(camera!==null && comp.activeCamera!==null && comp.activeCamera.index===camera.index,"Active camera unavailable");
        capture("3d-base");

        var position=transform.property("ADBE Position");check(position!==null,"Missing 3D position");
        var pv=position.value;position.setValue([pv[0]+14,pv[1]-9,pv[2]]);capture("3d-position");

        var scale=transform.property("ADBE Scale");check(scale!==null,"Missing 3D scale");
        scale.setValue([86,112,100]);capture("3d-scale");

        var yrot=transform.property("ADBE Rotate Y");check(yrot!==null,"Missing Y rotation");
        yrot.setValue(28);capture("3d-layer-rotate");

        var cameraTransform=camera.property("ADBE Transform Group");
        var cameraPosition=cameraTransform.property("ADBE Position");check(cameraPosition!==null,"Missing camera position");
        var cv=cameraPosition.value;
        cameraPosition.setValue([cv[0]+24,cv[1]-10,cv[2]]);capture("3d-camera-move");

        var parent=comp.layers.addNull();check(parent!==null,"Could not add parent");
        parent.name="__EGFX_PARENT_"+config.run_id;parent.threeDLayer=true;layer.parent=parent;
        var parentY=parent.property("ADBE Transform Group").property("ADBE Rotate Y");check(parentY!==null,"Missing parent rotation");
        parentY.setValue(-18);capture("3d-parent");

        var cameraB=comp.layers.addCamera("__EGFX_CAMERA_B_"+config.run_id,[64,48]);check(cameraB!==null,"Second camera unavailable");
        var cameraBPosition=cameraB.property("ADBE Transform Group").property("ADBE Position");check(cameraBPosition!==null,"Missing second camera position");
        var cvb=cameraBPosition.value;cameraBPosition.setValue([cvb[0]-32,cvb[1]+14,cvb[2]]);
        check(comp.activeCamera!==null && comp.activeCamera.index===cameraB.index,"Camera switch did not become active");
        capture("3d-camera-switch");

        // Leave this exact test-owned saved project open for live-image identity.
        // The dedicated cleanup phase closes only this path after diagnosis.
        comp.openInViewer();layer.selected=true;
        status="CAPTURED";stage="complete";
    } catch(error) {
        message=String(error);
        safeResetOwned();
    } finally {
        if(suppressing) {try{app.endSuppressDialogs(false);}catch(e){status="FAIL";}}
        if(report!==null && !report.exists) {
            report.encoding="UTF-8";
            if(report.open("w")) {
                var names=[];for(var j=0;j<frames.length;++j) names.push(q(frames[j]));
                function colorJSON(value) {return value===null?"null":'{"working_space":'+q(value.space)+',"linearize":'+String(value.linearize)+'}';}
                report.write('{"capture_method":"render-queue-straight-rgba16","run_id":'+q(config.run_id)+',"status":'+q(status)+',"stage":'+q(stage)+
                    ',"color_before":'+colorJSON(colorBefore)+',"color_applied":'+colorJSON(colorApplied)+
                    ',"message":'+q(message)+',"frames":['+names.join(",")+']}');
                report.close();
            }
        }
    }
    return status==="CAPTURED" ? 0 : 1;
}

function elasticGridPlaneSmokeCleanup(config) {
    var report=null,status="FAIL",stage="guard",message="";
    function q(s) {return '"'+String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n")+'"';}
    function check(v,s) {if(!v) throw new Error(s);}
    try {
        check(/^[a-f0-9]{32}$/.test(config.run_id),"Invalid run id");
        var folder=new Folder(config.folder);check(folder.exists,"Missing workspace");
        report=new File(folder.fsName+"/plane-cleanup.json");check(!report.exists,"Refusing stale cleanup report");
        var expected=new File(folder.fsName+"/plane-project.aep");check(expected.exists,"Owned project file missing");
        check(app.project!==null && app.project.file!==null && app.project.file.fsName===expected.fsName,
              "Current project is not the owned plane fixture");
        stage="close";
        check(app.project.close(CloseOptions.DO_NOT_SAVE_CHANGES),"Could not close owned plane fixture");
        var fresh=app.newProject();check(fresh!==null,"Could not create fresh project");
        var dirty=fresh.dirty;
        check(fresh.file===null && fresh.numItems===0 &&
              (dirty===false || (typeof dirty==="undefined" && fresh.revision===1)),
              "Fresh project guard failed");
        status="CLEAN";stage="complete";
    } catch(error) {message=String(error);}
    finally {
        if(report!==null && !report.exists && report.open("w")) {
            report.encoding="UTF-8";
            report.write('{"run_id":'+q(config.run_id)+',"status":'+q(status)+',"stage":'+q(stage)+',"message":'+q(message)+'}');
            report.close();
        }
    }
    return status==="CLEAN" ? 0 : 1;
}
