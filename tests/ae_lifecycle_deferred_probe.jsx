// Research only. Separate calls let AE reach idle before inspecting/removing.
function EGFXLifecycleDeferredProbe(config, action) {
    if (action !== 'add' && action !== 'remove') throw Error('Unknown action');
    var p=app.project, c=p && p.activeItem;
    if (!p || !p.file || p.file.fsName !== config.projectPath ||
        !(c instanceof CompItem) || c.name !== '__EGFX_TEXT_'+config.runId ||
        c.comment !== 'EGFX_TEXT_PROBE_V1:'+config.runId ||
        !/^[0-9a-f]{32}$/.test(config.runId) || c.width!==640 || c.height!==480 || c.numLayers!==1)
        throw Error('Not owned text fixture');
    var l=c.layer(1), effects=l.property('ADBE Effect Parade');
    if (l.locked || l.name!=='__EGFX_TEST_TEXT' || !l.property('ADBE Text Properties') ||
        effects.numProperties!==(action==='add'?1:2) ||
        effects.property(1).matchName!=='com.elasticgrid.fx.warp') throw Error('Fixture mismatch');
    if(action==='add') {
        if(p.dirty) throw Error('Requires saved fixture');
        l.selected=true;
        effects.addProperty('com.elasticgrid.fx.warp');
        if(l.property('ADBE Effect Parade').numProperties!==2) throw Error('Add count mismatch');
    } else {
        // Reacquire from the layer in a later invocation; no stale Property refs.
        if(effects.property(2).matchName!=='com.elasticgrid.fx.warp') throw Error('Wrong added effect');
        effects.property(2).remove();
        if(l.property('ADBE Effect Parade').numProperties!==1) throw Error('Remove count mismatch');
    }
    // Never save a modified fixture. The caller closes this owned project unsaved.
    return action+' PASS';
}
