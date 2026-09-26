(function () {
    var MATCH_NAME = "com.elasticgrid.fx.warp";
    var AEP_PATH = "/tmp/ElasticGridFX-v1-project-roundtrip.aep";
    var PNG_PATH = "/tmp/ElasticGridFX-v1-project-roundtrip.png";
    var tempFile = new File(AEP_PATH);
    var pngFile = new File(PNG_PATH);
    var oldBpc = null;
    var stage = 40;

    function fail(code) {
        app.exitCode = code;
        return false;
    }
    function closeProjectNoSave() {
        try {
            if (app.project !== null) {
                app.project.close(CloseOptions.DO_NOT_SAVE_CHANGES);
            }
        } catch (e) {}
    }
    function cleanTempFiles() {
        try { if (tempFile.exists) tempFile.remove(); } catch (e1) {}
        try { if (pngFile.exists) pngFile.remove(); } catch (e2) {}
    }
    function approx(a, b, eps) {
        return Math.abs(a - b) <= eps;
    }

    app.exitCode = 0;
    app.beginSuppressDialogs();
    try {
        // Never run against user work.
        stage = 40;
        var dirty = false;
        try { dirty = (typeof app.project.dirty !== "undefined") ? app.project.dirty : false; } catch (_) {}
        if (app.project.file !== null || app.project.numItems > 0 || dirty) {
            fail(stage);
            return;
        }

        cleanTempFiles();
        oldBpc = app.project.bitsPerChannel;
        app.project.bitsPerChannel = 32;

        stage = 41;
        var comp = app.project.items.addComp("__ElasticGridFX_Project_Roundtrip__", 480, 270, 1.0, 2.0, 30.0);
        var layer = comp.layers.addSolid([0.11, 0.44, 0.83], "__ElasticGridFX_Roundtrip_Source__", 480, 270, 1.0, 2.0);
        var fx = layer.property("ADBE Effect Parade").addProperty(MATCH_NAME);
        if (fx === null || fx.matchName !== MATCH_NAME) {
            fail(stage);
            return;
        }

        stage = 42;
        var cols = fx.property("Columns");
        var rows = fx.property("Rows");
        var elasticity = fx.property("Elasticity Strength");
        var waveEnabled = fx.property("Wave Animation");
        var waveAmplitude = fx.property("Wave Amplitude");
        var waveSpeed = fx.property("Wave Speed");
        var edge = fx.property("Edge Behavior");
        var quality = fx.property("Render Quality");
        if (cols === null || rows === null || elasticity === null || waveEnabled === null || waveAmplitude === null || waveSpeed === null || edge === null || quality === null) {
            fail(stage);
            return;
        }

        cols.setValue(9);
        rows.setValue(7);
        elasticity.setValue(137.0);
        waveEnabled.setValue(1);
        waveSpeed.setValue(0.42);
        edge.setValue(3);      // Mirror
        quality.setValue(2);   // Final Bicubic
        waveAmplitude.setValueAtTime(0.0, 2.0);
        waveAmplitude.setValueAtTime(1.0, 8.0);

        stage = 43;
        // Project.save(file) returns no value; success is verified by the file.
        app.project.save(tempFile);
        if (!tempFile.exists || tempFile.length <= 0) {
            fail(stage);
            return;
        }

        stage = 44;
        closeProjectNoSave();
        app.open(tempFile);
        if (app.project === null || app.project.numItems < 1) {
            fail(stage);
            return;
        }

        stage = 45;
        var reopenedComp = null;
        for (var i = 1; i <= app.project.numItems; i++) {
            if (app.project.item(i).name === "__ElasticGridFX_Project_Roundtrip__") {
                reopenedComp = app.project.item(i);
                break;
            }
        }
        if (reopenedComp === null || reopenedComp.numLayers < 1) {
            fail(stage);
            return;
        }
        var reopenedFx = reopenedComp.layer(1).property("ADBE Effect Parade").property(MATCH_NAME);
        if (reopenedFx === null || reopenedFx.matchName !== MATCH_NAME) {
            fail(stage);
            return;
        }

        stage = 46;
        var rc = reopenedFx.property("Columns");
        var rr = reopenedFx.property("Rows");
        var re = reopenedFx.property("Elasticity Strength");
        var rwa = reopenedFx.property("Wave Amplitude");
        var rws = reopenedFx.property("Wave Speed");
        var redge = reopenedFx.property("Edge Behavior");
        var rq = reopenedFx.property("Render Quality");
        if (rc === null || rr === null || re === null || rwa === null || rws === null || redge === null || rq === null) {
            fail(stage);
            return;
        }
        if (rc.value !== 9 || rr.value !== 7 || !approx(re.value, 137.0, 0.001) || !approx(rws.value, 0.42, 0.001) || redge.value !== 3 || rq.value !== 2) {
            fail(stage);
            return;
        }
        if (rwa.numKeys !== 2 || !approx(rwa.keyValue(1), 2.0, 0.001) || !approx(rwa.keyValue(2), 8.0, 0.001)) {
            fail(stage);
            return;
        }

        stage = 47;
        if (typeof reopenedComp.saveFrameToPng !== "function") {
            fail(stage);
            return;
        }
        if (pngFile.exists) pngFile.remove();
        reopenedComp.saveFrameToPng(0.75, pngFile);
        if (!pngFile.exists || pngFile.length <= 0) {
            fail(stage);
            return;
        }

        app.exitCode = 0;
    } catch (e) {
        app.exitCode = stage;
    } finally {
        try { closeProjectNoSave(); } catch (_) {}
        try { app.newProject(); } catch (_) {}
        try { if (oldBpc !== null) app.project.bitsPerChannel = oldBpc; } catch (_) {}
        cleanTempFiles();
        try { app.endSuppressDialogs(false); } catch (_) {}
    }
})();
