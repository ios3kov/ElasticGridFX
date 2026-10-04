// Loaded after ae_runtime_smoke.jsx. Separate prepare/set/export turns allow native
// deferred initialization to finish before captures. No scheduled tasks or
// host objects are retained between calls; the coordinator passes checked IDs.
function egfxPhasedWrite(config, name, data) {
    var f = new File(config.folder + "/" + name);
    if (f.exists) throw Error("Old phase result");
    f.encoding = "UTF-8";
    if (!f.open("w")) throw Error("Phase result unavailable");
    if (!f.write(data) || !f.close()) throw Error("Phase result write failed");
}
function egfxPhasedIds(project) {
    var ids = [];
    for (var i = 1; i <= project.numItems; i++) ids.push(project.item(i).id);
    return ids;
}
function egfxPhasedPrepare(config) {
    app.exitCode = 90;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw Error("Invalid nonce");
        var state = elasticGridCurrentProjectState();
        if (!elasticGridHasTestProjectOwnership(state.guard, state.project_revision))
            throw Error("Requires clean empty project");
        var owned = app.project, input = new File(config.folder + "/pattern.png");
        if (!input.exists || new File(config.folder + "/prepared.json").exists)
            throw Error("Workspace unavailable");
        owned.bitsPerChannel = 32; owned.workingSpace = ""; owned.linearizeWorkingSpace = false;
        var footage = owned.importFile(new ImportOptions(input));
        var comp = owned.items.addComp("__EGFX_" + config.run_id, 319, 241, 1, 2, 30);
        var chain = owned.items.addComp("__EGFX_CHAIN_" + config.run_id, 319, 241, 1, 2, 30);
        comp.layers.add(footage); chain.layers.add(footage);
        var adjustment = chain.layers.addSolid([0,0,0], "__EGFX_ADJUSTMENT__", 319, 241, 1, 2);
        adjustment.adjustmentLayer = true;
        for (var i = 0; i < 2; i++) {
            var layer = i === 0 ? comp.layer(1) : adjustment;
            var fx = layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
            if (!fx || fx.matchName !== "com.elasticgrid.fx.warp") throw Error("Effect unavailable");
            egfxFixtureParam(fx,"Columns").setValue(4); egfxFixtureParam(fx,"Rows").setValue(4);
            egfxFixtureParam(fx,"Render Quality").setValue(2); egfxFixtureParam(fx,"Edge Behavior").setValue(1);
            egfxFixtureParam(fx,"Wave Axis").setValue(1); egfxFixtureParam(fx,"Wave Frequency").setValue(1.3);
            egfxFixtureParam(fx,"Wave Phase").setValue(35); egfxFixtureParam(fx,"Wave Speed").setValue(0);
            egfxFixtureParam(fx,"Wave Amplitude").setValue(0); egfxFixtureParam(fx,"Stretch Easing").setValue(0);
        }
        if (app.project !== owned || owned.file !== null) throw Error("Project changed");
        egfxPhasedWrite(config,"prepared.json",'{"schema":1,"run_id":"'+config.run_id+
            '","status":"PREPARED","item_ids":['+egfxPhasedIds(owned).join(',')+
            '],"comp_id":'+comp.id+',"chain_id":'+chain.id+'}');
        app.exitCode = 0;
    } catch(e) {
        egfxPhasedWrite(config,"prepared.json",'{"run_id":"'+config.run_id+'","status":"FAIL"}');
    }
}
// Each setter turn is followed by an export-only turn. Never infer export
// completion from transport success, and never let a failed guard close work.
function egfxPhasedBindingReady(fx) {
    var names=["__FSTR Probe TL","__FSTR Probe TR","__FSTR Probe BR","__FSTR Probe BL","__FSTR Plane Kind"];
    for(var i=0;i<names.length;i++) {
        var p=fx.property(names[i]);
        if(!p || !p.expressionEnabled || !p.expression || p.expressionError) return false;
    }
    // Point .value is not a reliable scripting oracle in this host. Read only
    // the dimensionless discriminator; pixels remain the independent gate.
    var kind=fx.property(names[4]).value;
    return kind===1 || kind===2 || kind===3;
}
function egfxPhasedCapture(config) {
    var owned=null, comp=null, chain=null, status="FAIL", fresh=null, stage="guard";
    var names=["bypass","identity","static_a","static_b","animated_a","animated_b","reset",
        "chain_before_corner","chain_corner_identity","chain_corner_moved"];
    var times=[.25,.25,.25,.75,.125,.625,.25,.25,.25,.25], index=-1;
    var action=config.action, name=config.frame_name, output="capture.json";
    app.exitCode=90;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw Error("Invalid nonce");
        for(var n=0;n<names.length;n++)if(names[n]===name)index=n;
        if(action!=="cleanup" && (index<0 || (action!=="set" && action!=="export")))throw Error("Invalid step");
        if(action!=="cleanup")output=name+"-"+action+".json";
        var p=app.project,s=config.prepared;
        if(!s || s.run_id!==config.run_id || s.status!=="PREPARED" || p===null ||
            p.file!==null || p.numItems!==s.item_ids.length)throw Error("Ownership unavailable");
        var ids=egfxPhasedIds(p);
        for(var i=0;i<ids.length;i++) {
            if(ids[i]!==s.item_ids[i])throw Error("Foreign item");
            if(p.item(i+1).id===s.comp_id)comp=p.item(i+1);
            if(p.item(i+1).id===s.chain_id)chain=p.item(i+1);
        }
        if(!comp || !chain || comp.name!=="__EGFX_"+config.run_id ||
            chain.name!=="__EGFX_CHAIN_"+config.run_id || comp.numLayers!==1 || chain.numLayers!==2 ||
            !chain.layer(1).adjustmentLayer || p.bitsPerChannel!==32 || p.linearizeWorkingSpace!==false ||
            (p.workingSpace!=="" && p.workingSpace!=="None"))throw Error("Fixture changed");
        var effects=comp.layer(1).property("ADBE Effect Parade"),parade=chain.layer(1).property("ADBE Effect Parade");
        if(effects.numProperties!==1 || effects.property(1).matchName!=="com.elasticgrid.fx.warp" ||
            (parade.numProperties!==1 && parade.numProperties!==2) ||
            parade.property(1).matchName!=="com.elasticgrid.fx.warp" ||
            (parade.numProperties===2 && parade.property(2).matchName!=="ADBE Corner Pin"))throw Error("Effect changed");
        owned=p;
        if(action==="cleanup") {status="CAPTURED";stage="cleanup";}
        else {
            var fx=index<7?effects.property(1):parade.property(1), target=index<7?comp:chain;
            var amp=index===0 || index===1 || index===6?0:10, speed=index===4 || index===5?.5:0;
            if(!egfxPhasedBindingReady(fx))throw Error("Deferred binding not ready; no render attempted");
            stage=name+"_"+action;
            if(action==="set") {
                target.openInViewer();
                fx.enabled=index!==0;
                egfxFixtureParam(fx,"Wave Amplitude").setValue(amp);
                egfxFixtureParam(fx,"Wave Speed").setValue(speed);
                if(index>=8) {
                    var corner=parade.numProperties===2?parade.property(2):parade.addProperty("ADBE Corner Pin");
                    if(!corner || corner.matchName!=="ADBE Corner Pin")throw Error("Corner Pin unavailable");
                    var points=index===8?[[0,0],[319,0],[0,241],[319,241]]:[[20,15],[299,25],[10,220],[309,225]];
                    for(i=0;i<4;i++)corner.property(i+1).setValue(points[i]);
                }
                status="READY";
            } else {
                if(fx.enabled!==(index!==0) || egfxFixtureParam(fx,"Wave Amplitude").value!==amp ||
                    egfxFixtureParam(fx,"Wave Speed").value!==speed)throw Error("Step state changed");
                var path=config.folder+"/"+name+".png",file=new File(path);
                if(file.exists)throw Error("Old frame");
                target.saveFrameToPng(times[index],file);
                for(var attempt=0;attempt<50;attempt++) {
                    if(app.project!==owned)throw Error("Foreign project");
                    file=new File(path);if(file.exists && file.length>0)break;$.sleep(100);
                }
                if(!file.exists || file.length<=0)throw Error("Frame missing");
                status="EXPORTED";
            }
        }
    } catch(e) {status="FAIL";}
    finally {
        if(action==="cleanup" || status==="FAIL") {
            if(owned!==null && app.project===owned) {
                try {
                    if(owned.file!==null || egfxPhasedIds(owned).join(',')!==config.prepared.item_ids.join(',') ||
                        comp.numLayers!==1 || chain.numLayers!==2)throw Error("Cleanup changed");
                    if(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)===false)throw Error("Close failed");
                    app.newProject();fresh=elasticGridCurrentProjectState();
                    if(!elasticGridHasTestProjectOwnership(fresh.guard,fresh.project_revision))throw Error("Not clean");
                }catch(e){status="FAIL";stage="cleanup";}
            }else if(status==="CAPTURED"){status="FAIL";stage="foreign_project";}
        }
        egfxPhasedWrite(config,output,'{"run_id":"'+config.run_id+'","status":"'+status+
            '","stage":"'+stage+'","ae_version":"'+elasticGridAppVersion()+
            '","project_bpc":32,"fixture_color":"unmanaged","fresh_guard":"'+(fresh?fresh.guard:"NOT_CHECKED")+'","fresh_project_revision":"'+
            (fresh?fresh.project_revision:"NOT_CHECKED")+'"}');
        app.exitCode=status==="FAIL"?90:0;
    }
}
