// Research only: called after ae_text_plane_probe.jsx. Never installs expressions
// in a user project; corner expressions/values and rotation are restored.
function elasticGridTextPlaneExpressionProbe(config) {
    elasticGridTextPlaneToggle(config); // ownership/type checks before mutation
    var layer=app.project.activeItem.layer("__EGFX_TEST_TEXT");
    var fx=layer.property("ADBE Effect Parade").property("com.elasticgrid.fx.warp");
    var names=["Plane Top Left","Plane Top Right","Plane Bottom Right","Plane Bottom Left"];
    var points=["[r.left,r.top,0]","[r.left+r.width,r.top,0]",
                "[r.left+r.width,r.top+r.height,0]","[r.left,r.top+r.height,0]"];
    var saved=[],samples=[],rotation=layer.transform.yRotation.value;
    for(var i=0;i<4;i++) {
        var p=fx.property(names[i]);
        if(!p.canSetExpression || p.expression!=="" || p.numKeys!==0)
            throw new Error("Unexpected fixture corner state");
        saved.push(p.value);
    }
    try {
        for(var i=0;i<4;i++) fx.property(names[i]).expression=
            "var r=thisLayer.sourceRectAtTime(time,false); var p=thisLayer.toComp("+
            points[i]+"); [p[0],p[1]];";
        for(var a=0;a<2;a++) {
            layer.transform.yRotation.setValue(a*30);
            var corners=[];
            for(var i=0;i<4;i++) {
                var p=fx.property(names[i]),v=p.value;
                if(p.expressionError!=="") throw new Error(p.expressionError);
                if(!isFinite(v[0]) || !isFinite(v[1])) throw new Error("Nonfinite corner");
                corners.push([v[0],v[1]]);
            }
            samples.push({rotation:a*30,corners:corners});
        }
    } finally {
        layer.transform.yRotation.setValue(rotation);
        for(var i=0;i<4;i++) {
            fx.property(names[i]).expression="";
            fx.property(names[i]).setValue(saved[i]);
        }
    }
    return samples;
}
