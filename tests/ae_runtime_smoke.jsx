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
// Phase 1: instantiate ElasticGrid in a strictly empty test project so the native
// image is resident before Python performs live-image identity sampling.
//
// `Project.dirty` is an undocumented host attribute.  A missing attribute is not
// treated as a blanket opt-in: the only fallback ownership proof is an otherwise
// empty, unsaved project at the documented initial revision (1).  Read failures
// and malformed values remain unsafe.
function elasticGridProjectGuard(project) {
    var dirty = null;
    if (project === null || typeof project === "undefined") return "NO_PROJECT";
    try { if (project.file !== null) return "SAVED"; }
    catch (e) { return "FILE_READ_ERROR"; }
    try { if (project.numItems !== 0) return "OCCUPIED"; }
    catch (e) { return "ITEMS_READ_ERROR"; }
    try { dirty = project.dirty; }
    catch (e) { return "DIRTY_READ_ERROR"; }
    if (typeof dirty === "undefined") return "DIRTY_UNAVAILABLE";
    if (typeof dirty !== "boolean") return "DIRTY_INVALID";
    return dirty ? "DIRTY" : "CLEAN";
}

function elasticGridProjectRevision(project) {
    var revision = null;
    if (project === null || typeof project === "undefined") return "UNAVAILABLE";
    try { revision = project.revision; }
    catch (e) { return "READ_ERROR"; }
    if (typeof revision !== "number" || !isFinite(revision) || Math.floor(revision) !== revision || revision < 1)
        return "INVALID";
    return String(revision);
}

function elasticGridCurrentProjectState() {
    var project = null;
    try { project = app.project; }
    catch (e) { return {guard:"PROJECT_READ_ERROR",project_revision:"READ_ERROR"}; }
    return {guard:elasticGridProjectGuard(project),project_revision:elasticGridProjectRevision(project)};
}

function elasticGridHasTestProjectOwnership(guard, revision) {
    return guard === "CLEAN" || (guard === "DIRTY_UNAVAILABLE" && revision === "1");
}

function elasticGridAppVersion() {
    try {
        var version = String(app.version);
        return /^[0-9A-Za-z._ -]{1,64}$/.test(version) ? version : "UNAVAILABLE";
    }
    catch (e) { return "UNAVAILABLE"; }
}

function elasticGridSmokeArm(config) {
    var resultFile = null, suppressing = false;
    var result = {run_id:config.run_id,status:"FAIL",stage:"guard",ae_version:"",
                  guard:"NOT_CHECKED",project_revision:"NOT_CHECKED"};
    function q(s){return '"' + String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n") + '"';}
    app.exitCode=91;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw new Error("Invalid run identifier");
        var folder=new Folder(config.folder);
        if (!folder.exists) throw new Error("Run workspace does not exist");
        resultFile=new File(config.folder+"/arm.json");
        if (resultFile.exists) { resultFile=null; throw new Error("Refusing old arm result"); }
        var input=new File(config.folder+"/pattern.png");
        if (!input.exists) throw new Error("Input fixture is missing");
        result.ae_version=elasticGridAppVersion();
        var currentState=elasticGridCurrentProjectState();
        result.guard=currentState.guard;
        result.project_revision=currentState.project_revision;
        if (!elasticGridHasTestProjectOwnership(result.guard,result.project_revision))
            throw new Error("Requires an empty, unsaved, non-dirty test project");
        app.beginSuppressDialogs(); suppressing=true;
        var footage=app.project.importFile(new ImportOptions(input));
        footage.name="__EGFX_ARM_FOOTAGE_"+config.run_id;
        var comp=app.project.items.addComp("__EGFX_ARM_COMP_"+config.run_id,64,64,1.0,1.0,30.0);
        var layer=comp.layers.add(footage); layer.name="__EGFX_ARM_LAYER_"+config.run_id;
        var fx=layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        if (fx===null || fx.matchName!=="com.elasticgrid.fx.warp") throw new Error("Effect unavailable");
        if (fx.property("Columns")===null) throw new Error("Effect parameter unavailable");
        result.status="ARMED"; result.stage="armed";
    } catch(error) {
        result.status="FAIL";
    } finally {
        if (suppressing) { try{app.endSuppressDialogs(false);}catch(e){result.status="FAIL";result.stage="dialogs";} }
        if (resultFile!==null) {
            try {
                resultFile.encoding="UTF-8";
                if (resultFile.exists || !resultFile.open("w")) throw new Error("Result unavailable");
                resultFile.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"stage":'+q(result.stage)+
                    ',"ae_version":'+q(result.ae_version)+',"guard":'+q(result.guard)+
                    ',"project_revision":'+q(result.project_revision)+'}');
                resultFile.close();
            } catch(e) { result.status="FAIL"; }
        }
        app.exitCode=result.status==="ARMED"?0:91;
    }
    return app.exitCode;
}

function elasticGridSmokeDisarm(config) {
    var resultFile=null, result={run_id:config.run_id,status:"FAIL",stage:"guard",
                                 fresh_guard:"NOT_CHECKED",fresh_project_revision:"NOT_CHECKED"};
    function q(s){return '"' + String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"') + '"';}
    app.exitCode=93;
    try {
        var folder=new Folder(config.folder);
        if (!folder.exists || app.project===null || app.project.file!==null || app.project.numItems!==2)
            throw new Error("Armed project ownership unavailable");
        var comp=null, footage=null;
        for (var i=1;i<=app.project.numItems;i++) {
            var item=app.project.item(i);
            if (item.name==="__EGFX_ARM_COMP_"+config.run_id) comp=item;
            else if (item.name==="__EGFX_ARM_FOOTAGE_"+config.run_id) footage=item;
            else throw new Error("Foreign project item detected");
        }
        if (comp===null || footage===null || comp.numLayers!==1) throw new Error("Armed objects missing");
        var layer=comp.layer(1);
        if (layer.name!=="__EGFX_ARM_LAYER_"+config.run_id) throw new Error("Foreign layer detected");
        var parade=layer.property("ADBE Effect Parade");
        if (parade===null || parade.numProperties!==1 || parade.property(1).matchName!=="com.elasticgrid.fx.warp")
            throw new Error("Armed effect state changed");
        if (!app.project.close(CloseOptions.DO_NOT_SAVE_CHANGES)) throw new Error("Owned project close failed");
        app.newProject();
        var freshState=elasticGridCurrentProjectState();
        result.fresh_guard=freshState.guard;
        result.fresh_project_revision=freshState.project_revision;
        if (!elasticGridHasTestProjectOwnership(result.fresh_guard,result.fresh_project_revision))
            throw new Error("Fresh empty project unavailable");
        result.status="CLEAN"; result.stage="clean";
    } catch(error) {
        result.status="FAIL";
    } finally {
        resultFile=new File(config.folder+"/disarm.json");
        try {
            resultFile.encoding="UTF-8";
            if (resultFile.exists || !resultFile.open("w")) throw new Error("Result unavailable");
            resultFile.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"stage":'+q(result.stage)+
                ',"fresh_guard":'+q(result.fresh_guard)+',"fresh_project_revision":'+q(result.fresh_project_revision)+'}');
            resultFile.close();
        } catch(e) { result.status="FAIL"; }
        app.exitCode=result.status==="CLEAN"?0:93;
    }
    return app.exitCode;
}

// Called by the runner with a fresh configuration; opening this file alone does nothing.
function elasticGridSmoke(config) {
    var owned = null, comp = null, chainComp = null, footage = null, initialBpc = null;
    var solidSource = null, solidFolder = null, initialSpace = null, initialLinearize = null;
    var resultFile = null, suppressing = false;
    var result = {run_id: config.run_id, status: "FAIL", stage: "guard", ae_version: "", loaded_build_id: null,
                  guard:"NOT_CHECKED",project_revision:"NOT_CHECKED"};
    function quote(s) {
        return '"' + String(s).replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\r/g, "\\r").replace(/\n/g, "\\n").replace(/\t/g, "\\t") + '"';
    }
    function own() {
        if (owned === null || app.project !== owned) throw new Error("Project ownership changed");
    }
    function property(fx, name) {
        var value = egfxFixtureParam(fx, name);
        if (value === null) throw new Error("Required parameter is missing");
        return value;
    }
    function frame(targetComp, name, time) {
        result.stage = "frame_" + name;
        own();
        var file = new File(config.folder + "/" + name + ".png");
        if (file.exists) throw new Error("Refusing old frame");
        targetComp.saveFrameToPng(time, file);
        // Refresh ExtendScript's File metadata after the host writes the PNG.
        file = new File(config.folder + "/" + name + ".png");
        // AE may finish publishing the file shortly after the host call returns.
        // Bound the wait; pixel decoding remains the independent external gate.
        for (var attempt = 0; (!file.exists || file.length <= 0) && attempt < 50; attempt++) {
            if (typeof $ === "undefined" || typeof $.sleep !== "function") break;
            $.sleep(100);
            own();
            file = new File(config.folder + "/" + name + ".png");
        }
        if (!file.exists || file.length <= 0) throw new Error("Frame missing");
    }
    app.exitCode = 90;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw new Error("Invalid run identifier");
        var folder = new Folder(config.folder);
        if (!folder.exists) throw new Error("Run workspace does not exist");
        resultFile = new File(config.folder + "/capture.json");
        if (resultFile.exists) { resultFile = null; throw new Error("Refusing old result"); }
        var input = new File(config.folder + "/pattern.png");
        if (!input.exists) throw new Error("Input fixture is missing");
        result.ae_version = elasticGridAppVersion();
        var currentState=elasticGridCurrentProjectState();
        result.guard=currentState.guard;
        result.project_revision=currentState.project_revision;
        if (!elasticGridHasTestProjectOwnership(result.guard,result.project_revision)) {
            throw new Error("Requires an empty, unsaved, non-dirty test project");
        }
        app.beginSuppressDialogs(); suppressing = true;
        initialBpc = app.project.bitsPerChannel;
        owned = app.project;
        initialSpace = owned.workingSpace;
        initialLinearize = owned.linearizeWorkingSpace;
        owned.workingSpace = "";
        owned.linearizeWorkingSpace = false;
        if ((owned.workingSpace !== "" && owned.workingSpace !== "None") || owned.linearizeWorkingSpace !== false) throw new Error("Fixture color state was not applied");
        owned.bitsPerChannel = 32;
        if (owned.bitsPerChannel !== 32) throw new Error("Project depth was not applied");
        result.stage = "fixture";
        footage = owned.importFile(new ImportOptions(input));
        comp = owned.items.addComp("__EGFX_"+config.run_id, 319, 241, 1.0, 2.0, 30.0);
        comp.resolutionFactor = [1, 1];
        var layer = comp.layers.add(footage);
        var fx = layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        if (fx === null || fx.matchName !== "com.elasticgrid.fx.warp") throw new Error("Effect unavailable");
        if (typeof comp.saveFrameToPng !== "function") throw new Error("Frame capture unsupported");
        result.stage = "parameters";
        var amplitude = property(fx, "Wave Amplitude"), speed = property(fx, "Wave Speed");
        property(fx, "Columns").setValue(4); property(fx, "Rows").setValue(4);
        property(fx, "Render Quality").setValue(2);
        property(fx, "Edge Behavior").setValue(1);
        property(fx, "Wave Axis").setValue(1);
        property(fx, "Wave Frequency").setValue(1.3);
        property(fx, "Wave Phase").setValue(35.0);
        property(fx, "Stretch Easing").setValue(0.0);
        amplitude.setValue(0.0); speed.setValue(0.0);
        result.stage = "capture";
        fx.enabled = false; frame(comp, "bypass", 0.25);
        fx.enabled = true; frame(comp, "identity", 0.25);
        amplitude.setValue(10.0);
        frame(comp, "static_a", 0.25); frame(comp, "static_b", 0.75);
        speed.setValue(0.5);
        frame(comp, "animated_a", 0.125); frame(comp, "animated_b", 0.625);
        amplitude.setValue(0.0); frame(comp, "reset", 0.25);

        // Reproduce the reported effect-chain case on an Adjustment Layer.
        // The first Corner Pin capture uses its default identity mapping: simply
        // adding the downstream effect must not turn the frame black.
        result.stage = "corner_pin_chain";
        chainComp = owned.items.addComp("__EGFX_CHAIN_"+config.run_id, 319, 241, 1.0, 2.0, 30.0);
        chainComp.resolutionFactor = [1, 1];
        chainComp.layers.add(footage);
        var adjustment = chainComp.layers.addSolid([0,0,0], "__EGFX_ADJUSTMENT__", 319, 241, 1.0, 2.0);
        solidSource = adjustment.source;
        solidFolder = solidSource.parentFolder;
        adjustment.adjustmentLayer = true;
        var chainFx = adjustment.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        if (chainFx === null || chainFx.matchName !== "com.elasticgrid.fx.warp") throw new Error("Chain effect unavailable");
        property(chainFx, "Columns").setValue(4); property(chainFx, "Rows").setValue(4);
        property(chainFx, "Render Quality").setValue(2);
        property(chainFx, "Wave Axis").setValue(1);
        property(chainFx, "Wave Frequency").setValue(1.3);
        property(chainFx, "Wave Phase").setValue(35.0);
        property(chainFx, "Wave Speed").setValue(0.0);
        property(chainFx, "Wave Amplitude").setValue(10.0);
        frame(chainComp, "chain_before_corner", 0.25);

        var corner = adjustment.property("ADBE Effect Parade").addProperty("ADBE Corner Pin");
        if (corner === null || corner.matchName !== "ADBE Corner Pin") throw new Error("Corner Pin unavailable");
        frame(chainComp, "chain_corner_identity", 0.25);
        corner.property(1).setValue([20,15]);
        corner.property(2).setValue([299,25]);
        corner.property(3).setValue([10,220]);
        corner.property(4).setValue([309,225]);
        frame(chainComp, "chain_corner_moved", 0.25);

        result.status = "CAPTURED"; // Pixel assertions run externally, never infer PASS here.
    } catch (error) {
        result.status = "FAIL";
        // Numeric host diagnostics cannot expose project names or user paths.
        result.error_number = typeof error.number === "number" && isFinite(error.number) ? error.number : null;
        result.error_line = typeof error.line === "number" && isFinite(error.line) ? error.line : null;
    } finally {
        if (owned !== null) {
            if (app.project !== owned) { result.status = "FAIL"; result.stage = "foreign_project"; }
            else {
                try {
                    // Delete only objects this script created; never close user projects.
                    if (chainComp !== null) chainComp.remove();
                    if (solidSource !== null) solidSource.remove();
                    if (solidFolder !== null && solidFolder !== owned.rootFolder && solidFolder.numItems === 0) solidFolder.remove();
                    if (comp !== null) comp.remove();
                    if (footage !== null) footage.remove();
                    owned.bitsPerChannel = initialBpc;
                    owned.workingSpace = initialSpace;
                    owned.linearizeWorkingSpace = initialLinearize;
                    // The Windows coordinator immediately starts another guarded
                    // test. Item removal leaves this disposable project dirty.
                    // Reset only the exact owned, fully cleaned unsaved project.
                    if (config.reset_owned_after_capture === true) {
                        if (app.project !== owned || owned.file !== null || owned.numItems !== 0)
                            throw new Error("Owned empty cleanup unavailable");
                        if (owned.close(CloseOptions.DO_NOT_SAVE_CHANGES) === false)
                            throw new Error("Owned cleanup close failed");
                        owned = null;
                        var freshProject = app.newProject();
                        if (freshProject === null || freshProject !== app.project)
                            throw new Error("Fresh project unavailable");
                        var freshState = elasticGridCurrentProjectState();
                        result.fresh_guard = freshState.guard;
                        result.fresh_project_revision = freshState.project_revision;
                        if (!elasticGridHasTestProjectOwnership(result.fresh_guard, result.fresh_project_revision))
                            throw new Error("Fresh project is not clean");
                    }
                } catch (cleanupError) { result.status = "FAIL"; result.stage = "cleanup"; }
            }
        }
        if (suppressing) {
            try { app.endSuppressDialogs(false); }
            catch (dialogError) { result.status = "FAIL"; result.stage = "dialogs"; }
        }
        if (resultFile !== null) {
            try {
                resultFile.encoding = "UTF-8";
                if (resultFile.exists || !resultFile.open("w")) throw new Error("Result file unavailable");
                resultFile.write('{"run_id":'+quote(result.run_id)+',"status":'+quote(result.status)+
                    ',"stage":'+quote(result.stage)+',"ae_version":'+quote(result.ae_version)+
                    ',"project_bpc":32,"fixture_color":"unmanaged","loaded_build_id":null,"guard":'+quote(result.guard)+
                    ',"project_revision":'+quote(result.project_revision)+
                    ',"fresh_guard":'+quote(result.fresh_guard || "NOT_CHECKED")+
                    ',"fresh_project_revision":'+quote(result.fresh_project_revision || "NOT_CHECKED")+
                    ',"error_number":'+(result.error_number == null ? 'null' : String(result.error_number))+
                    ',"error_line":'+(result.error_line == null ? 'null' : String(result.error_line))+'}');
                resultFile.close();
            } catch (writeError) { result.status = "FAIL"; }
        }
        app.exitCode = result.status === "CAPTURED" ? 0 : 90;
    }
    return app.exitCode;
}
