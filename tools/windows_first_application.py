#!/usr/bin/env python3
"""Cold-start first-application validation for an already-installed Windows .aex.

No installation is performed. Run this only after manually starting a fresh,
selected After Effects process with an empty unsaved project.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
import tempfile
import time
import uuid

from smoke_pixels import difference, read_png
import windows_ae_validation as common


def _write_wrapper(script_dir: Path, evidence_dir: Path, run_id: str, depth: int, kind: str, three_d: bool) -> Path:
    guard = (common.ROOT / "tests/ae_runtime_smoke.jsx").read_text(encoding="utf-8")
    fixture = (common.ROOT / "tests/ae_first_application.jsx").read_text(encoding="utf-8")
    config = dict(run_id=run_id, folder=str(evidence_dir), depth=depth, kind=kind, three_d=three_d)
    wrapper = r'''
(function () {
    var result = {run_id: CONFIG.run_id, status:"FAIL", value:"", error:""};
    function q(s) {
        var input=String(s), out='"', bs=String.fromCharCode(92);
        for (var i=0; i<input.length; i++) {
            var code=input.charCodeAt(i);
            if (code===92) out+=bs+bs;
            else if (code===34) out+=bs+'"';
            else if (code===13) out+=bs+'r';
            else if (code===10) out+=bs+'n';
            else out+=input.charAt(i);
        }
        return out+'"';
    }
    try {
        result.value = elasticGridFirstApplicationFixture(CONFIG);
        result.status = result.value === "CAPTURED_NOT_FULL_ACCEPTANCE" ? "CAPTURED" : "FAIL";
    } catch(e) {
        result.error = String(e);
    } finally {
        var file = new File(CONFIG.folder + "/first-result.json");
        try {
            file.encoding="UTF-8";
            if (file.exists || !file.open("w")) throw Error("result unavailable");
            file.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"value":'+q(result.value)+',"error":'+q(result.error)+'}');
            file.close();
        } catch(_) {}
        app.exitCode = result.status === "CAPTURED" ? 0 : 94;
    }
})();
'''.replace("CONFIG", json.dumps(config))
    path = script_dir / "first.jsx"
    path.write_text(guard + "\n" + fixture + "\n" + wrapper, encoding="utf-8")
    return path


def _write_followup(script_dir: Path, evidence_dir: Path, run_id: str) -> Path:
    config = dict(run_id=run_id, folder=str(evidence_dir))
    source = r'''
(function(){
    var config=CONFIG, result={run_id:config.run_id,status:"FAIL",stage:"guard"};
    function q(s) {
        var input=String(s), out='"', bs=String.fromCharCode(92);
        for (var i=0; i<input.length; i++) {
            var code=input.charCodeAt(i);
            if (code===92) out+=bs+bs;
            else if (code===34) out+=bs+'"';
            else out+=input.charAt(i);
        }
        return out+'"';
    }
    try {
        var projectFile=new File(config.folder+"/first-application.aep");
        if (app.project===null || app.project.file===null ||
            app.project.file.fsName!==projectFile.fsName) throw Error("owned project unavailable");
        var compName="__EGFX_FIRST_"+config.run_id, comp=null;
        for (var i=1;i<=app.project.numItems;i++) {
            if (app.project.item(i).name===compName) {
                if (comp!==null) throw Error("duplicate owned comp");
                comp=app.project.item(i);
            }
        }
        if (comp===null || comp.numLayers<1) throw Error("owned comp unavailable");
        var layer=comp.layer(1), fx=layer.property("ADBE Effect Parade").property("com.elasticgrid.fx.warp");
        if (fx===null || fx.matchName!=="com.elasticgrid.fx.warp") throw Error("effect unavailable");
        var amp=fx.property("Wave Amplitude");
        if (amp===null) throw Error("wave amplitude unavailable");
        amp.setValue(10.0);
        result.stage="render";
        var item=app.project.renderQueue.items.add(comp);
        try {
            var output=item.outputModule(1);
            output.applyTemplate("_HIDDEN X-Factor 16");
            output=item.outputModule(1);
            output.file=new File(config.folder+"/deformed-[#####].png");
            item.setSettings({"Quality":"Best","Resolution":"Full","Color Depth":"Current Settings","Effects":"Current Settings"});
            item.timeSpanStart=0; item.timeSpanDuration=comp.frameDuration;
            app.project.renderQueue.render();
            if (item.status!==RQItemStatus.DONE) throw Error("deformed render failed");
        } finally { try{item.remove();}catch(_){} }
        result.status="CAPTURED"; result.stage="captured";
    } catch(e) {
        result.error=String(e);
    } finally {
        try {
            if (app.project!==null && app.project.file!==null &&
                app.project.file.fsName===new File(config.folder+"/first-application.aep").fsName) {
                app.project.close(CloseOptions.DO_NOT_SAVE_CHANGES);
                var fresh=app.newProject();
                if (fresh===null || fresh!==app.project || fresh.file!==null || fresh.numItems!==0)
                    throw Error("fresh project unavailable");
            }
        } catch(cleanupError) {
            result.status="FAIL"; result.stage="cleanup"; result.error=String(cleanupError);
        }
        var file=new File(config.folder+"/followup.json");
        try {
            file.encoding="UTF-8";
            if (file.exists || !file.open("w")) throw Error("result unavailable");
            file.write('{"run_id":'+q(result.run_id)+',"status":'+q(result.status)+',"stage":'+q(result.stage)+',"error":'+q(result.error||"")+'}');
            file.close();
        } catch(_) { result.status="FAIL"; }
        app.exitCode=result.status==="CAPTURED"?0:95;
    }
})();
'''.replace("CONFIG", json.dumps(config))
    path=script_dir/"followup.jsx"
    path.write_text(source,encoding="utf-8")
    return path

def _single_png(folder: Path, prefix: str) -> Path:
    files=sorted(folder.glob(prefix+"-*.png"))
    if len(files)!=1 or files[0].stat().st_size<=0:
        raise ValueError(f"expected one {prefix} PNG, found {len(files)}")
    return files[0]


def execute(args) -> dict:
    common._require_windows()
    manifest=common.load_manifest(args.manifest.resolve(strict=True))
    identity=common.verify_candidate(args.candidate,args.installed_aex,manifest)
    pid=common.running_selected_ae(args.afterfx)

    run_id=uuid.uuid4().hex
    root=args.run_root.resolve()
    root.mkdir(parents=True,exist_ok=True)
    folder=root/("EGFX-first-"+run_id)
    folder.mkdir()
    control=root/("EGFX-first-control-"+run_id)
    control.mkdir()
    first=_write_wrapper(control,folder,run_id,args.depth,args.kind,args.three_d)
    followup=_write_followup(control,folder,run_id)

    result=dict(schema=1,status="FAIL",artifact=identity,pid=pid,workspace=str(folder),
                case=dict(depth=args.depth,kind=args.kind,three_d=args.three_d),checks={})
    result["checks"]["first_transport"]=common.run_jsx(args.afterfx,first,folder/"first-result.json")
    first_result=json.loads((folder/"first-result.json").read_text(encoding="utf-8-sig"))
    if first_result.get("run_id")!=run_id or first_result.get("status")!="CAPTURED":
        raise ValueError("first-application fixture failed")

    loaded=common.verify_loaded_effect(pid,args.installed_aex,manifest)
    result["loaded_module"]=loaded
    bypass=_single_png(folder,"bypass")
    neutral=_single_png(folder,"first-neutral")
    neutral_diff=difference(read_png(bypass),read_png(neutral))
    result["checks"]["neutral_vs_bypass"]=neutral_diff
    if neutral_diff["max_abs"]!=0.0:
        raise ValueError("first neutral frame differs from bypass")

    # A separate command turn gives the registered idle hook a normal host-idle
    # opportunity. A non-neutral render must then succeed and visibly deform.
    time.sleep(0.25)
    result["checks"]["followup_transport"]=common.run_jsx(args.afterfx,followup,folder/"followup.json")
    follow=json.loads((folder/"followup.json").read_text(encoding="utf-8-sig"))
    if follow.get("run_id")!=run_id or follow.get("status")!="CAPTURED":
        raise ValueError("post-idle deformation fixture failed")
    deformed=_single_png(folder,"deformed")
    deformation=difference(read_png(neutral),read_png(deformed))
    result["checks"]["deformation_vs_neutral"]=deformation
    if deformation["changed_fraction"]<=0.001:
        raise ValueError("post-idle frame did not visibly deform")
    if common.running_selected_ae(args.afterfx)!=pid:
        raise ValueError("After Effects process changed during first-application validation")

    result["status"]="LIMITED"
    result["loaded_build_id"]=loaded["build_id"]
    result["remaining"]=[
        "repeat matrix across solid/text/checker_precomp, 8/16/32 bpc and 2D/3D",
        "visual confirmation that no transient modal error appeared",
        "interactive guide/Undo/MFR validation",
    ]
    (folder/"first-application-validation.json").write_text(
        json.dumps(result,indent=2,sort_keys=True)+"\n",encoding="utf-8")
    return result


def main() -> int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate",required=True,type=Path)
    parser.add_argument("--manifest",required=True,type=Path)
    parser.add_argument("--installed-aex",required=True,type=Path)
    parser.add_argument("--afterfx",required=True,type=Path)
    parser.add_argument("--depth",required=True,type=int,choices=(8,16,32))
    parser.add_argument("--kind",required=True,choices=("solid","text","checker_precomp"))
    parser.add_argument("--three-d",action="store_true")
    parser.add_argument("--run-root",type=Path,default=Path(tempfile.gettempdir())/"ElasticGridFX-first-application")
    args=parser.parse_args()
    try:
        result=execute(args)
    except Exception as error:
        print("ERROR: "+str(error),file=sys.stderr)
        return 1
    print(json.dumps(result,indent=2,sort_keys=True))
    return 3 if result["status"]=="LIMITED" else 0


if __name__=="__main__":
    raise SystemExit(main())
