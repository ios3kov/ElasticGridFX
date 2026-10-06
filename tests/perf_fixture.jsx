// Standalone fixtures accept legacy/new labels and flat/grouped host controls.
function egfxFixtureParam(root, name, depth) {
    depth = depth || 0;
    if (depth > 8) throw Error("Parameter nesting exceeds fixture limit");
    var aliases = {"Falloff":"Follow Shape", "Stretch Easing":"Smooth Stretch", "Easing Distance":"Smooth Width"};
    var direct = root.property(name);
    if (direct !== null) return direct;
    if (aliases[name]) { direct = root.property(aliases[name]); if (direct !== null) return direct; }
    if (root.numProperties > 128) throw Error("Parameter count exceeds fixture limit");
    for (var i = 1; i <= root.numProperties; i++) {
        var child = root.property(i);
        if (child !== null && child.numProperties > 0) {
            var found = egfxFixtureParam(child, name, depth + 1);
            if (found !== null) return found;
        }
    }
    return null;
}
// Creation returns to the host so its existing idle binding can finish.
// Finalization never installs expressions or accepts an unready plane.
function elasticGridPerfFixtureCreate(config) {
    var owned=null, result={run_id:config.run_id,status:"FAIL",stage:"guard"}, file=null;
    function q(s){return '"'+String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"')+'"';}
    app.exitCode=92;
    try {
        if(!/^[a-f0-9]{32}$/.test(config.run_id) ||
           !((config.width===1920 && config.height===1080)||(config.width===3840 && config.height===2160)) ||
           !(config.bit_depth===8||config.bit_depth===16||config.bit_depth===32) || config.fps!==30 || config.duration!==2 ||
           !(config.mode==="static"||config.mode==="animated") || !(config.output_precision===8||config.output_precision===16) ||
           !(config.plane_mode==="four-corners"||config.plane_mode==="layer"))
            throw new Error("Unsupported creation configuration");
        if(!(new Folder(config.folder)).exists)throw new Error("Missing workspace");
        var input=new File(config.folder+"/pattern.png");
        if(!input.exists || input.length<=0)throw new Error("Missing pattern");
        file=new File(config.folder+"/creation.json");
        if(file.exists || (new File(config.folder+"/capture.json")).exists || (new File(config.folder+"/EGFX_PERF.aep")).exists){file=null;throw new Error("Stale fixture");}
        if(app.project===null || app.project.file!==null || app.project.numItems!==0 || app.project.dirty!==false)
            throw new Error("Requires clean empty project");
        owned=app.project;owned.workingSpace="";owned.linearizeWorkingSpace=false;owned.bitsPerChannel=config.bit_depth;
        var footage=owned.importFile(new ImportOptions(input));
        var comp=owned.items.addComp("EGFX_PERF",config.width,config.height,1,config.duration,config.fps);
        comp.comment="__EGFX_PERF_"+config.run_id;comp.resolutionFactor=[1,1];
        var layer=comp.layers.add(footage);
        var fx=layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        if(fx===null || fx.matchName!=="com.elasticgrid.fx.warp")throw new Error("Effect unavailable");
        comp.openInViewer();layer.selected=false;
        result.status="CREATED";result.stage="await_binding";
    } catch(e) {
        if(owned!==null && app.project===owned){
            if(owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)!==false && app.project!==owned)app.newProject();
        }
    } finally {
        if(file!==null){
            file.encoding="UTF-8";
            if(!file.exists && file.open("w")){file.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"stage":'+q(result.stage)+'}');file.close();}
            else result.status="FAIL";
        }
        app.exitCode=result.status==="CREATED"?0:92;
    }
    return app.exitCode;
}

// Finalizes only the structurally verified scene created in the previous host turn.
function elasticGridPerfFixture(config) {
    var owned = null, footage = null, comp = null, initialBpc = null;
    var resultFile = null, suppressing = false, projectFile = null, saved = false;
    var result = {run_id:config.run_id,status:"FAIL",stage:"guard",ae_version:""};
    function q(s) { return '"' + String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n") + '"'; }
    function own() { if (owned === null || app.project !== owned) throw new Error("Project ownership changed"); }
    function param(fx,name) { var p=egfxFixtureParam(fx,name); if (p===null) throw new Error("Missing parameter"); return p; }
    function hasTemplate(list,name) { for (var i=0;i<list.length;i++) if (list[i]===name) return true; return false; }
    app.exitCode = 92;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw new Error("Invalid run identifier");
        if (!((config.width===1920 && config.height===1080) || (config.width===3840 && config.height===2160))) throw new Error("Unsupported fixture size");
        if (!(config.bit_depth===8 || config.bit_depth===16 || config.bit_depth===32)) throw new Error("Unsupported bit depth");
        if (!(config.fps===30 && config.duration===2)) throw new Error("Unsupported fixture timing");
        if (!(config.mode==="static" || config.mode==="animated")) throw new Error("Unsupported fixture mode");
        if (!(config.output_precision===8 || config.output_precision===16)) throw new Error("Unsupported output precision");
        if (!(config.plane_mode==="four-corners" || config.plane_mode==="layer")) throw new Error("Unsupported deformation plane");
        var folder=new Folder(config.folder);
        if (!folder.exists) throw new Error("Workspace missing");
        var input=new File(config.folder+"/pattern.png");
        if (!input.exists || input.length<=0) throw new Error("Pattern missing");
        projectFile=new File(config.folder+"/EGFX_PERF.aep");
        resultFile=new File(config.folder+"/capture.json");
        if (projectFile.exists || resultFile.exists) { resultFile=null; throw new Error("Stale fixture evidence"); }
        if (app.project===null || app.project.file!==null || app.project.numItems!==2)
            throw new Error("Requires owned unsaved fixture");
        var candidate=null;
        for(var ci=1;ci<=app.project.numItems;ci++){
            var item=app.project.item(ci);
            if(item.name==="EGFX_PERF" && item.layers)candidate=item;
        }
        if(candidate===null || candidate.comment!=="__EGFX_PERF_"+config.run_id || candidate.numLayers!==1 ||
           candidate.width!==config.width || candidate.height!==config.height || candidate.frameRate!==config.fps ||
           candidate.duration!==config.duration || candidate.resolutionFactor[0]!==1 || candidate.resolutionFactor[1]!==1)
            throw new Error("Fixture ownership/geometry differs");
        var layer=candidate.layer(1), parade=layer.property("ADBE Effect Parade");
        if(layer.source===null || layer.source.file===null || layer.source.file.fsName!==input.fsName || parade.numProperties!==1)
            throw new Error("Fixture source/effect differs");
        var fx=parade.property(1);
        if(fx.matchName!=="com.elasticgrid.fx.warp")throw new Error("Effect differs");
        // A UUID comment alone is insufficient: check the exact structure/source above.
        var hiddenNames=["__FSTR Probe TL","__FSTR Probe TR","__FSTR Probe BR","__FSTR Probe BL","__FSTR Plane Kind"];
        for(var hi=0;hi<5;hi++){
            var hidden=fx.property(hiddenNames[hi]);
            if(hidden===null || hidden.name!==hiddenNames[hi] || !hidden.expressionEnabled || hidden.expressionError!=="")
                throw new Error("Deferred plane binding is not ready");
        }
        if(fx.property('__FSTR Plane Kind').value!==1)throw new Error("Expected ready 2D footage plane");

        result.ae_version=app.version;
        app.beginSuppressDialogs(); suppressing=true;
        initialBpc=app.project.bitsPerChannel;
        owned=app.project;
        owned.workingSpace="";owned.linearizeWorkingSpace=false;
        if ((owned.workingSpace!=="" && owned.workingSpace!=="None") || owned.linearizeWorkingSpace!==false)
            throw new Error("Fixture color state was not applied");
        owned.bitsPerChannel=config.bit_depth;
        if (owned.bitsPerChannel!==config.bit_depth) throw new Error("Bit depth not applied");

        result.stage="fixture";
        comp=candidate;footage=layer.source;
        result.binding_ready=true;
        var planeOrdinal=config.plane_mode==="four-corners" ? 2 : 1;
        param(fx,"Deformation Plane").setValue(planeOrdinal);
        if (param(fx,"Deformation Plane").value!==planeOrdinal) throw new Error("Plane mode was not applied");
        var planeCorners=[];
        if (planeOrdinal===2) {
            var cornerNames=["Top Left","Top Right","Bottom Right","Bottom Left"];
            var cornerValues=[[0,0],[config.width-1,0],[config.width-1,config.height-1],[0,config.height-1]];
            for (var cornerIndex=0;cornerIndex<4;cornerIndex++) {
                var cornerProperty=param(fx,cornerNames[cornerIndex]);cornerProperty.setValue(cornerValues[cornerIndex]);
                var corner=cornerProperty.value;
                if (corner.length!==2 || corner[0]!==cornerValues[cornerIndex][0] || corner[1]!==cornerValues[cornerIndex][1])
                    throw new Error("Plane corners were not applied");
                planeCorners.push(corner[0]);planeCorners.push(corner[1]);
            }
        }
        param(fx,"Columns").setValue(8); param(fx,"Rows").setValue(8);
        param(fx,"Render Quality").setValue(2); param(fx,"__FSTR Edge Value").setValue(1);
        param(fx,"Wave Axis").setValue(1); param(fx,"Wave Frequency").setValue(1.3);
        param(fx,"Wave Phase").setValue(35.0); param(fx,"Stretch Easing").setValue(0.0);
        param(fx,"Wave Amplitude").setValue(10.0);
        param(fx,"Wave Speed").setValue(config.mode==="animated" ? 0.5 : 0.0);

        result.stage="render_queue";
        var rq=owned.renderQueue.items.add(comp);
        result.stage="render_template";
        if (!hasTemplate(rq.templates,"Best Settings")) throw new Error("Best Settings template unavailable");
        rq.applyTemplate("Best Settings");
        rq.timeSpanStart=0.0; rq.timeSpanDuration=config.duration; rq.skipFrames=0; rq.render=true;
        var om=rq.outputModule(1);
        result.stage="output_template";
        // Reuse the straight RGBA16 template already verified by the target
        // plane fixture. A template label alone is never precision evidence.
        var outputTemplate=config.output_precision===16 && hasTemplate(om.templates,"_HIDDEN X-Factor 16") ? "_HIDDEN X-Factor 16" :
            (hasTemplate(om.templates,"PNG Sequence") ? "PNG Sequence" : (hasTemplate(om.templates,"png") ? "png" : null));
        if (outputTemplate===null) throw new Error("PNG Sequence template unavailable");
        om.applyTemplate(outputTemplate);
        // OutputModule objects may be invalidated by settings changes; reacquire.
        om=rq.outputModule(1);
        result.stage="output_path";
        var outputFolder=new Folder(config.folder+"/fixture-output");
        if (outputFolder.exists || !outputFolder.create()) throw new Error("Fixture output directory collision/failure");
        om.file=new File(config.folder+"/fixture-output/frame_[#####].png");
        var settings=om.getSettings(GetSettingsFormat.STRING);
        result.stage="output_format";
        if (!settings || String(settings.Format)!=="PNG Sequence") throw new Error("Output format is not PNG Sequence");
        if (String(settings.Resize)!=="false" || String(settings.Crop)!=="false") throw new Error("Output geometry changed");
        if (config.output_precision===16 && (String(settings.Depth)!=="Trillions of Colors+" ||
            String(settings.Channels)!=="RGB + Alpha" || String(settings.Color)!=="Straight (Unmatted)"))
            throw new Error("Required straight RGBA16 output was not applied");
        if (config.output_precision===8 && String(settings.Depth)!=="Millions of Colors")
            throw new Error("Legacy RGB8 output was not applied");

        result.stage="save";
        owned.save(projectFile);
        if (!projectFile.exists || projectFile.length<=0 || owned.file===null || owned.file.fsName!==projectFile.fsName)
            throw new Error("Fixture project save failed");
        saved=true;
        result.status="PREPARED";
        result.stage="prepared";
        result.width=config.width; result.height=config.height; result.fps=config.fps;
        result.duration=config.duration; result.bit_depth=config.bit_depth; result.mode=config.mode;
        result.plane_mode=config.plane_mode;result.plane_corners=planeCorners;
        result.expected_render_path=planeOrdinal===2 ? "plane_region" : "legacy_cpu";
        result.composition="EGFX_PERF"; result.rqindex=1;
        result.render_template="Best Settings"; result.output_template=outputTemplate;
        result.output_format="PNG Sequence"; result.output_pattern="frame_[#####].png";
        result.output_precision=config.output_precision;result.output_depth=String(settings.Depth);
        result.output_channels=String(settings.Channels);result.output_color=String(settings.Color);
        result.working_space=String(owned.workingSpace);result.linearize=owned.linearizeWorkingSpace;
    } catch (error) {
        result.status="FAIL";
        result.error_number=typeof error.number==='number' ? error.number : null;
        result.error_line=typeof error.line==='number' ? error.line : null;
    } finally {
        if (owned!==null) {
            if (app.project!==owned) { result.status="FAIL"; result.stage="foreign_project"; }
            else {
                try {
                    // This project was empty/unsaved before ownership was acquired.
                    // Close only this owned synthetic/test project; never a user project.
                    owned.bitsPerChannel=initialBpc;
                    if (!owned.close(CloseOptions.DO_NOT_SAVE_CHANGES)) throw new Error("Owned project close failed");
                    app.newProject();
                    if (app.project===null) throw new Error("New empty project unavailable");
                } catch (cleanupError) { result.status="FAIL"; result.stage="cleanup"; }
            }
        }
        if (suppressing) {
            try { app.endSuppressDialogs(false); } catch (dialogError) { result.status="FAIL"; result.stage="dialogs"; }
        }
        if (resultFile!==null) {
            try {
                resultFile.encoding="UTF-8";
                if (resultFile.exists || !resultFile.open("w")) throw new Error("Result unavailable");
                resultFile.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"stage":'+q(result.stage)+
                    ',"ae_version":'+q(result.ae_version)+',"width":'+(result.width||0)+',"height":'+(result.height||0)+
                    ',"fps":'+(result.fps||0)+',"duration":'+(result.duration||0)+',"bit_depth":'+(result.bit_depth||0)+
                    ',"mode":'+q(result.mode||"")+
                    ',"plane_mode":'+q(result.plane_mode||"")+',"plane_corners":['+(result.plane_corners||[]).join(',')+']'+
                    ',"expected_render_path":'+q(result.expected_render_path||"")+',"composition":'+q(result.composition||"")+
                    ',"rqindex":'+(result.rqindex||0)+',"render_template":'+q(result.render_template||"")+
                    ',"output_template":'+q(result.output_template||"")+
                    ',"output_format":'+q(result.output_format||"")+
                    ',"output_pattern":'+q(result.output_pattern||"")+
                    ',"output_precision":'+(result.output_precision||0)+',"output_depth":'+q(result.output_depth||"")+
                    ',"output_channels":'+q(result.output_channels||"")+',"output_color":'+q(result.output_color||"")+
                    ',"working_space":'+q(result.working_space||"")+',"linearize":'+String(result.linearize===true)+
                    ',"saved":'+(saved?"true":"false")+',"binding_ready":'+(result.binding_ready===true?"true":"false")+
                    ',"error_number":'+(result.error_number==null?'null':result.error_number)+
                    ',"error_line":'+(result.error_line==null?'null':result.error_line)+'}');
                resultFile.close();
            } catch (writeError) { result.status="FAIL"; }
        }
        app.exitCode=result.status==="PREPARED" ? 0 : 92;
    }
    return app.exitCode;
}
