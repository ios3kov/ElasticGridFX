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
// PNG export may complete after saveFrameToPng returns. Re-read the File
// object while the owned project remains open; closing it can cancel export.
function egfxWaitForPng(path) {
    for (var attempt = 0; attempt < 50; attempt++) {
        var freshFile = new File(path);
        if (freshFile.exists && freshFile.length > 0) return freshFile;
        $.sleep(100);
    }
    return null;
}
(function () {
    var MATCH_NAME = "com.elasticgrid.fx.warp";
    var config = typeof ELASTICGRID_ROUNDTRIP_CONFIG === "undefined" ? null : ELASTICGRID_ROUNDTRIP_CONFIG;
    var tempFile = null;
    var pngFile = null;
    var ownedProject = null;
    var runFolder = null;
    var oldBpc = null;
    var stage = 40;

    function fail(code) {
        app.exitCode = code;
        return false;
    }
    function closeOwnedProject() {
        // A rejected test never owns the current project. Nor may a host error
        // or context change grant ownership of a different project.
        if (ownedProject === null || app.project !== ownedProject) {
            throw new Error("The active project is not owned by this test.");
        }
        if (ownedProject.close(CloseOptions.DO_NOT_SAVE_CHANGES) === false) {
            throw new Error("Could not close the test project.");
        }
        ownedProject = null;
    }
    function prepareWorkspace() {
        var runId = new Date().getTime().toString(36) + "-" + Math.floor(Math.random() * 0x7fffffff).toString(36);
        runFolder = new Folder(config === null ? Folder.temp.fsName + "/ElasticGridFX-roundtrip-" + runId : config.folder);
        // Never reuse or clean a previous run's path. Retain evidence on failure.
        if (runFolder.exists || !runFolder.create()) {
            throw new Error("Could not reserve a new test workspace.");
        }
        tempFile = new File(runFolder.fsName + "/project.aep");
        pngFile = new File(runFolder.fsName + "/frame.png");
        if (tempFile.exists || pngFile.exists) {
            throw new Error("Test workspace is not empty.");
        }
    }
    function approx(a, b, eps) {
        return Math.abs(a - b) <= eps;
    }

    app.exitCode = 0;
    app.beginSuppressDialogs();
    try {
        // Never run against user work.
        stage = 40;
        if (app.project === null) {
            fail(stage);
            return;
        }
        // A failing state query must fail closed, not assume an empty project.
        var dirty = app.project.dirty;
        if (typeof dirty !== "boolean" || app.project.file !== null || app.project.numItems > 0 || dirty) {
            fail(stage);
            return;
        }

        oldBpc = app.project.bitsPerChannel;
        prepareWorkspace();
        ownedProject = app.project;
        ownedProject.bitsPerChannel = 32;

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
        var elasticity = (fx.property("Follow Strength") || fx.property("Elasticity Strength"));
        var waveEnabled = fx.property("Wave Animation");
        var waveAmplitude = egfxWaveParam(fx, "Wave Amplitude");
        var waveSpeed = egfxWaveParam(fx, "Wave Speed");
        var edge = fx.property("__FSTR Edge Value");
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
        closeOwnedProject();
        var reopenedProject = app.open(tempFile);
        if (reopenedProject === null || reopenedProject !== app.project ||
                reopenedProject.file === null || reopenedProject.file.fsName !== tempFile.fsName) {
            fail(stage);
            return;
        }
        ownedProject = reopenedProject;
        if (ownedProject.numItems < 1) {
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
        var re = (reopenedFx.property("Follow Strength") || reopenedFx.property("Elasticity Strength"));
        var rwa = egfxWaveParam(reopenedFx, "Wave Amplitude");
        var rws = egfxWaveParam(reopenedFx, "Wave Speed");
        var redge = reopenedFx.property("__FSTR Edge Value");
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
        if (pngFile.exists) {
            fail(stage);
            return;
        }
        reopenedComp.saveFrameToPng(0.75, pngFile);
        pngFile = egfxWaitForPng(pngFile.fsName);
        if (pngFile === null) {
            fail(stage);
            return;
        }

        app.exitCode = 0;
    } catch (e) {
        app.exitCode = stage;
    } finally {
        // `return` from a safety guard still executes finally. Cleanup must
        // therefore be conditional on ownership, not just reaching this block.
        if (ownedProject !== null && app.project === ownedProject) {
            try {
                closeOwnedProject();
                var emptyProject = app.newProject();
                if (emptyProject === null || emptyProject !== app.project) {
                    throw new Error("Could not restore an empty project.");
                }
                if (oldBpc !== null) emptyProject.bitsPerChannel = oldBpc;
            } catch (_) {
                // Cleanup failure is a failed test, never a false PASS.
                if (app.exitCode === 0) app.exitCode = 48;
            }
        } else if (app.exitCode === 0) {
            app.exitCode = 48;
        }
        // The unique AEP/PNG workspace is intentionally retained as evidence.

        try { app.endSuppressDialogs(false); } catch (_) {}
        // Existing-host -r transport completion is not the script's result.
        // Emit only after all state assertions and owned-project cleanup finish.
        if (config !== null) {
            var result = new File(config.result_file);
            var pending = new File(config.result_file + ".pending");
            try {
                if (!/^[0-9a-f]{32}$/.test(config.run_id) ||
                    !/^EGFX-[0-9a-f]{24}$/.test(config.build_id)) throw Error("invalid run identity");
                if (result.exists || pending.exists) throw Error("result unavailable");
                pending.encoding = "UTF-8";
                if (!pending.open("w")) throw Error("result unavailable");
                if (!pending.write('{"schema":1,"run_id":"' + config.run_id +
                    '","build_id":"' + config.build_id + '","status":"' +
                    (app.exitCode === 0 ? "PASS" : "FAIL") +
                    '","exit_code":' + app.exitCode + '}')) throw Error("write failed");
                if (!pending.close()) throw Error("close failed");
                if (!pending.rename(result.name)) throw Error("completion publish failed");
            } catch (_) {
                if (app.exitCode === 0) app.exitCode = 49;
            }
        }
    }
})();
