// Called by the runner with a fresh configuration; opening this file alone does nothing.
function elasticGridSmoke(config) {
    var owned = null, comp = null, footage = null, initialBpc = null;
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
    function frame(name, time) {
        own();
        var file = new File(config.folder + "/" + name + ".png");
        if (file.exists) throw new Error("Refusing old frame");
        comp.saveFrameToPng(time, file);
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
        fx.enabled = false; frame("bypass", 0.25);
        fx.enabled = true; frame("identity", 0.25);
        amplitude.setValue(10.0);
        frame("static_a", 0.25); frame("static_b", 0.75);
        speed.setValue(0.5);
        frame("animated_a", 0.125); frame("animated_b", 0.625);
        amplitude.setValue(0.0); frame("reset", 0.25);
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
