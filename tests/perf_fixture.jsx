// Creates only a synthetic performance project in a unique controlled workspace.
function elasticGridPerfFixture(config) {
    var owned = null, footage = null, comp = null, initialBpc = null;
    var resultFile = null, suppressing = false, projectFile = null, saved = false;
    var result = {run_id:config.run_id,status:"FAIL",stage:"guard",ae_version:""};
    function q(s) { return '"' + String(s).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\r/g,"\\r").replace(/\n/g,"\\n") + '"'; }
    function param(fx,name) { var p=fx.property(name); if (p===null) throw new Error("Missing parameter"); return p; }
    function hasTemplate(list,name) { for (var i=0;i<list.length;i++) if (list[i]===name) return true; return false; }
    app.exitCode = 92;
    try {
        if (!/^[a-f0-9]{32}$/.test(config.run_id)) throw new Error("Invalid run identifier");
        if (!((config.width===1920 && config.height===1080) || (config.width===3840 && config.height===2160) ||
              (config.width===7680 && config.height===4320))) throw new Error("Unsupported fixture size");
        if (!(config.bit_depth===8 || config.bit_depth===16 || config.bit_depth===32)) throw new Error("Unsupported bit depth");
        if (!(config.fps===30 && config.duration===2)) throw new Error("Unsupported fixture timing");
        if (!(config.mode==="static" || config.mode==="animated")) throw new Error("Unsupported fixture mode");
        if (!(config.geometry==="grid" || config.geometry==="four_corners")) throw new Error("Unsupported fixture geometry");
        var folder=new Folder(config.folder);
        if (!folder.exists) throw new Error("Workspace missing");
        var input=new File(config.folder+"/pattern.png");
        if (!input.exists || input.length<=0) throw new Error("Pattern missing");
        projectFile=new File(config.folder+"/EGFX_PERF.aep");
        resultFile=new File(config.folder+"/capture.json");
        if (projectFile.exists || resultFile.exists) { resultFile=null; throw new Error("Stale fixture evidence"); }
        if (app.project===null || app.project.file!==null || app.project.numItems!==0 ||
            typeof app.project.dirty!=="boolean" || app.project.dirty) throw new Error("Requires empty unsaved clean test project");

        result.ae_version=app.version;
        app.beginSuppressDialogs(); suppressing=true;
        initialBpc=app.project.bitsPerChannel;
        owned=app.project;
        owned.bitsPerChannel=config.bit_depth;
        if (owned.bitsPerChannel!==config.bit_depth) throw new Error("Bit depth not applied");
        owned.workingSpace="";
        owned.linearizeWorkingSpace=false;
        if (owned.workingSpace!=="" || owned.linearizeWorkingSpace!==false) throw new Error("Color management contract not applied");

        result.stage="fixture";
        footage=owned.importFile(new ImportOptions(input));
        comp=owned.items.addComp("EGFX_PERF",config.width,config.height,1.0,config.duration,config.fps);
        comp.resolutionFactor=[1,1];
        var layer=comp.layers.add(footage);
        var fx=layer.property("ADBE Effect Parade").addProperty("com.elasticgrid.fx.warp");
        if (fx===null || fx.matchName!=="com.elasticgrid.fx.warp") throw new Error("ElasticGrid unavailable");
        param(fx,"Columns").setValue(8); param(fx,"Rows").setValue(8);
        param(fx,"Render Quality").setValue(2); param(fx,"Edge Behavior").setValue(1);
        param(fx,"Wave Axis").setValue(1); param(fx,"Wave Frequency").setValue(1.3);
        param(fx,"Wave Phase").setValue(35.0); param(fx,"Stretch Easing").setValue(0.0);
        param(fx,"Wave Amplitude").setValue(10.0);
        param(fx,"Wave Speed").setValue(config.mode==="animated" ? 0.5 : 0.0);
        if (config.geometry==="four_corners") {
            param(fx,"Deformation Plane").setValue(2);
            param(fx,"Plane Top Left").setValue([config.width*0.08,config.height*0.08]);
            param(fx,"Plane Top Right").setValue([config.width*0.92,config.height*0.03]);
            param(fx,"Plane Bottom Right").setValue([config.width*0.95,config.height*0.90]);
            param(fx,"Plane Bottom Left").setValue([config.width*0.05,config.height*0.94]);
        } else {
            param(fx,"Deformation Plane").setValue(1);
        }

        result.stage="render_queue";
        var rq=owned.renderQueue.items.add(comp);
        result.stage="render_template";
        if (!hasTemplate(rq.templates,"Best Settings")) throw new Error("Best Settings template unavailable");
        rq.applyTemplate("Best Settings");
        rq.timeSpanStart=0.0; rq.timeSpanDuration=config.duration; rq.skipFrames=0; rq.render=true;
        var om=rq.outputModule(1);
        result.stage="output_template";
        var outputTemplate=hasTemplate(om.templates,"PNG Sequence") ? "PNG Sequence" : (hasTemplate(om.templates,"png") ? "png" : null);
        if (outputTemplate===null) throw new Error("PNG Sequence template unavailable");
        om.applyTemplate(outputTemplate);
        om=rq.outputModule(1);
        result.stage="output_path";
        var outputFolder=new Folder(config.folder+"/fixture-output");
        if (outputFolder.exists || !outputFolder.create()) throw new Error("Fixture output directory collision/failure");
        om.file=new File(config.folder+"/fixture-output/frame_[#####].png");
        var settings=om.getSettings(GetSettingsFormat.STRING);
        result.stage="output_format";
        if (!settings || String(settings.Format)!=="PNG Sequence") throw new Error("Output format is not PNG Sequence");
        if (String(settings.Resize)!=="false" || String(settings.Crop)!=="false") throw new Error("Output geometry changed");

        result.stage="save";
        owned.save(projectFile);
        if (!projectFile.exists || projectFile.length<=0 || owned.file===null || owned.file.fsName!==projectFile.fsName)
            throw new Error("Fixture project save failed");
        saved=true;
        result.status="PREPARED";
        result.stage="prepared";
        result.width=config.width; result.height=config.height; result.fps=config.fps;
        result.duration=config.duration; result.bit_depth=config.bit_depth; result.mode=config.mode;
        result.geometry=config.geometry; result.color_management="none-linearize-off";
        result.composition="EGFX_PERF"; result.rqindex=1;
        result.render_template="Best Settings"; result.output_template=outputTemplate;
        result.output_format="PNG Sequence"; result.output_pattern="frame_[#####].png";
    } catch (error) {
        result.status="FAIL";
        result.error_number=typeof error.number==='number' ? error.number : null;
        result.error_line=typeof error.line==='number' ? error.line : null;
    } finally {
        if (owned!==null) {
            if (app.project!==owned) { result.status="FAIL"; result.stage="foreign_project"; }
            else {
                try {
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
                    ',"mode":'+q(result.mode||"")+',"geometry":'+q(result.geometry||"")+
                    ',"color_management":'+q(result.color_management||"")+',"composition":'+q(result.composition||"")+
                    ',"rqindex":'+(result.rqindex||0)+',"render_template":'+q(result.render_template||"")+
                    ',"output_template":'+q(result.output_template||"")+
                    ',"output_format":'+q(result.output_format||"")+
                    ',"output_pattern":'+q(result.output_pattern||"")+
                    ',"saved":'+(saved?"true":"false")+
                    ',"error_number":'+(result.error_number==null?'null':result.error_number)+
                    ',"error_line":'+(result.error_line==null?'null':result.error_line)+'}');
                resultFile.close();
            } catch (writeError) { result.status="FAIL"; }
        }
        app.exitCode=result.status==="PREPARED" ? 0 : 92;
    }
    return app.exitCode;
}
