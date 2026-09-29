// Called by the runner with a fresh configuration; opening this file alone does nothing.
function elasticGridSmoke(config) {
    var owned = null, comp = null, chainComp = null, footage = null, initialBpc = null;
    var resultFile = null, suppressing = false;
    var result = {run_id: config.run_id, status: "FAIL", stage: "guard", ae_version: "", loaded_build_id: null};
    function quote(s) {
        return '"' + String(s).replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\r/g, "\\r").replace(/\n/g, "\\n").replace(/\t/g, "\\t") + '"';
    }
    function own() {
        if (owned === null || app.project !== owned) throw new Error("Project ownership changed");
    }
    function property(fx, name) {
        var value = fx.property(name);
        if (value === null) throw new Error("Required parameter is missing");
        return value;
    }
    function frame(targetComp, name, time) {
        own();
        var file = new File(config.folder + "/" + name + ".png");
        if (file.exists) throw new Error("Refusing old frame");
        targetComp.saveFrameToPng(time, file);
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
        if (app.project === null || app.project.file !== null || app.project.numItems !== 0 ||
                typeof app.project.dirty !== "boolean" || app.project.dirty) {
            throw new Error("Requires an empty, unsaved, non-dirty test project");
        }
        result.ae_version = app.version;
        app.beginSuppressDialogs(); suppressing = true;
        initialBpc = app.project.bitsPerChannel;
        owned = app.project;
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
        // Store the stage, not arbitrary exception text that may include user paths.
    } finally {
        if (owned !== null) {
            if (app.project !== owned) { result.status = "FAIL"; result.stage = "foreign_project"; }
            else {
                try {
                    // Delete only objects this script created; never close user projects.
                    if (chainComp !== null) chainComp.remove();
                    if (comp !== null) comp.remove();
                    if (footage !== null) footage.remove();
                    owned.bitsPerChannel = initialBpc;
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
                    ',"project_bpc":32,"loaded_build_id":null}');
                resultFile.close();
            } catch (writeError) { result.status = "FAIL"; }
        }
        app.exitCode = result.status === "CAPTURED" ? 0 : 90;
    }
    return app.exitCode;
}
