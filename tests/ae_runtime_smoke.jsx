(function () {
    var MATCH_NAME = "com.elasticgrid.fx.warp";
    var PNG_PATH = "/tmp/ElasticGridFX-v0.9-smoke.png";
    var comp = null;
    var solidSource = null;
    var oldBpc = null;
    var stage = 90;

    app.exitCode = 0;
    app.beginSuppressDialogs();
    try {
        // Safety gate: never run the automated smoke against user work.
        // Run it only from AE's empty, unsaved project state.
        stage = 20;
        var dirty = false;
        try {
            dirty = (typeof app.project.dirty !== "undefined") ? app.project.dirty : false;
        } catch (dirtyProbeError) {
            dirty = false;
        }
        if (app.project.file !== null || app.project.numItems > 0 || dirty) {
            app.exitCode = stage;
            return;
        }

        stage = 21;
        var found = false;
        for (var i = 0; i < app.effects.length; i++) {
            if (app.effects[i].matchName === MATCH_NAME) {
                found = true;
                break;
            }
        }
        if (!found) {
            app.exitCode = stage;
            return;
        }

        stage = 22;
        oldBpc = app.project.bitsPerChannel;
        app.project.bitsPerChannel = 32;
        comp = app.project.items.addComp("__ElasticGridFX_Runtime_Smoke__", 320, 240, 1.0, 1.0, 30.0);
        var layer = comp.layers.addSolid([0.23, 0.57, 0.91], "__ElasticGridFX_Source__", 320, 240, 1.0, 1.0);
        solidSource = layer.source;
        var parade = layer.property("ADBE Effect Parade");
        var fx = parade.addProperty(MATCH_NAME);
        if (fx === null || fx.matchName !== MATCH_NAME) {
            app.exitCode = stage;
            return;
        }

        stage = 23;
        var quality = fx.property("Render Quality");
        var waveEnabled = fx.property("Wave Animation");
        var waveAmplitude = fx.property("Wave Amplitude");
        var waveSpeed = fx.property("Wave Speed");
        var edge = fx.property("Edge Behavior");
        if (quality === null || waveEnabled === null || waveAmplitude === null || waveSpeed === null || edge === null) {
            app.exitCode = stage;
            return;
        }
        quality.setValue(2);       // Final (Bicubic)
        waveEnabled.setValue(1);
        waveAmplitude.setValue(5.0);
        waveSpeed.setValue(0.35);
        edge.setValue(3);          // Mirror

        stage = 24;
        if (typeof comp.saveFrameToPng !== "function") {
            app.exitCode = stage;
            return;
        }

        stage = 25;
        var png = new File(PNG_PATH);
        if (png.exists) {
            png.remove();
        }
        comp.saveFrameToPng(0.4, png);
        if (!png.exists || png.length <= 0) {
            app.exitCode = stage;
            return;
        }

        app.exitCode = 0;
    } catch (e) {
        app.exitCode = stage;
    } finally {
        try {
            if (comp !== null) {
                comp.remove();
            }
        } catch (cleanupCompError) {}
        try {
            if (solidSource !== null) {
                solidSource.remove();
            }
        } catch (cleanupSourceError) {}
        try {
            if (oldBpc !== null) {
                app.project.bitsPerChannel = oldBpc;
            }
        } catch (cleanupBpcError) {}
        try {
            app.endSuppressDialogs(false);
        } catch (cleanupDialogError) {}
    }
})();
