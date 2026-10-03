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
// Isolated text-layer baseline. Does not close or save an existing project.
function elasticGridTextPlaneProbe(config) {
    if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw new Error("Invalid run id");
    var dir = new Folder(config.folder);
    var file = new File(dir.fsName + "/text-plane.aep");
    if (!dir.exists || file.exists) throw new Error("Invalid/stale output");
    if (!app.project || app.project.file !== null || app.project.numItems !== 0 || app.project.dirty !== false)
        throw new Error("Requires clean empty project");
    var comp = app.project.items.addComp("__EGFX_TEXT_" + config.run_id, 640, 480, 1, 1, 30);
    comp.comment = "EGFX_TEXT_PROBE_V1:" + config.run_id;
    var layer = comp.layers.addText("GRID\rPLANE");
    layer.name = "__EGFX_TEST_TEXT";
    var source = layer.property("ADBE Text Properties").property("ADBE Text Document");
    var doc = source.value;
    doc.fontSize = 100; doc.applyFill = true; doc.fillColor = [1,1,1];
    // Do not inherit the user's last-used leading (386 px hid line 2 in QA).
    doc.autoLeading = false; doc.leading = 110;
    doc.justification = ParagraphJustification.CENTER_JUSTIFY;
    source.setValue(doc);
    layer.property("ADBE Transform Group").property("ADBE Position").setValue([320,200]);
    var fx = layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
    fx.property("Deformation Plane").setValue(1);
    egfxWaveParam(fx, "Wave Amplitude").setValue(0);
    app.project.save(file);
    comp.openInViewer(); layer.selected = true; fx.selected = true;
}

function elasticGridTextPlaneToggle(config) {
    if (!/^[a-f0-9]{32}$/.test(config.run_id) || typeof config.three_d !== "boolean")
        throw new Error("Invalid toggle config");
    var file = new File(config.folder + "/text-plane.aep");
    if (!app.project || !app.project.file || app.project.file.fsName !== file.fsName)
        throw new Error("Foreign project");
    var comp = app.project.activeItem;
    if (!comp || comp.name !== "__EGFX_TEXT_" + config.run_id ||
        comp.comment !== "EGFX_TEXT_PROBE_V1:" + config.run_id ||
        comp.numLayers !== 1 || comp.width !== 640 || comp.height !== 480)
        throw new Error("Foreign comp");
    var layer = comp.layer("__EGFX_TEST_TEXT");
    var text = layer && layer.property("ADBE Text Properties");
    var effects = layer && layer.property("ADBE Effect Parade");
    var fx = effects && effects.property("com.elasticgrid.fx.warp");
    if (!text || !text.property("ADBE Text Document") || !fx || layer.locked)
        throw new Error("Foreign layer");
    layer.threeDLayer = config.three_d === true;
    layer.selected = true;
    fx.selected = true;
}
