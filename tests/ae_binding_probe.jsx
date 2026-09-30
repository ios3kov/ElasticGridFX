// Read-only verifier for the opt-in hidden-binding experiment.
function EGFXBindingProbe(config, enabled) {
    var p=app.project,c=p && p.activeItem;
    if(typeof enabled!=='boolean' || !p || !p.file || p.file.fsName!==config.projectPath ||
       !(c instanceof CompItem) || c.name!=='__EGFX_TEXT_'+config.runId ||
       c.comment!=='EGFX_TEXT_PROBE_V1:'+config.runId || c.numLayers!==1 || c.width!==640 || c.height!==480)
        throw Error('Not owned fixture');
    var l=c.layer(1),effects=l.property('ADBE Effect Parade');
    if(l.name!=='__EGFX_TEST_TEXT' || !l.property('ADBE Text Properties') || effects.numProperties!==2)
        throw Error('Wrong text fixture');
    var fx=effects.property(2),names=['__FSTR Probe TL','__FSTR Probe TR','__FSTR Probe BR','__FSTR Probe BL'];
    if(fx.matchName!=='com.elasticgrid.fx.warp' || fx.numProperties!==29 ||
       fx.property(29).matchName!=='ADBE Effect Built In Params') throw Error('Wrong research schema');
    var result=[];
    for(var i=0;i<4;i++) {
        var q=fx.property(24+i);
        if(q.name!==names[i] || q.numKeys!==0 || q.expressionEnabled!==enabled) throw Error('Binding state mismatch '+i);
        if(enabled && (q.expression.indexOf('// FSTR research plane v1')!==0 || q.expressionError!=='')) throw Error('Expression mismatch '+i);
        if(!enabled && q.expression!=='') throw Error('Undo left an expression '+i);
        var v=q.value;
        if(!isFinite(v[0]) || !isFinite(v[1])) throw Error('Nonfinite coordinate');
        result.push(v);
    }
    var kind=fx.property(28);
    if(kind.name!=='__FSTR Plane Kind'||kind.numKeys!==0||kind.expressionEnabled!==enabled||
       (enabled&&(kind.expressionError!==''||kind.value!==(l.threeDLayer?2:1)))||
       (!enabled&&kind.expression!==''))throw Error('Kind binding mismatch');
    // The original effect must retain pristine hidden streams: target isolation.
    var original=effects.property(1);
    for(var i=0;i<5;i++) if(original.property(24+i).expression!=='') throw Error('Original effect was modified');
    return 'PASS enabled='+enabled+' corners='+result.toSource();
}
