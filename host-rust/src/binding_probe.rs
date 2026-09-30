//! Write experiment, opt-in only. Dedicated hidden streams on owned fixture.
use super::*;
use binding_transaction::{Host, Snapshot, Outcome};
const NAMES: [&str;4] = ["__FSTR Probe TL", "__FSTR Probe TR", "__FSTR Probe BR", "__FSTR Probe BL"];

pub fn add_params(params:&mut ae::Parameters<Params>)->Result<(),ae::Error>{
    for (id,name) in [Params::ResearchPlaneTL,Params::ResearchPlaneTR,
        Params::ResearchPlaneBR,Params::ResearchPlaneBL].into_iter().zip(NAMES) {
        params.add_with_flags(id,name,ae::PointDef::setup(|f| {
            f.set_default((0.0,0.0));f.set_restrict_bounds(false);
        }),ae::ParamFlag::empty(),ae::ParamUIFlags::INVISIBLE)?;
    }
    Ok(())
}
fn expressions()->[String;4]{
    [(false,false),(true,false),(true,true),(false,true)].map(|(right,bottom)|format!(
        "// FSTR research plane v1\nvar r=thisLayer.sourceRectAtTime(time,false);\nvar p=thisLayer.toComp([r.left{},r.top{},0]);\n[p[0],p[1]];",
        if right {"+r.width"} else {""},if bottom {"+r.height"} else {""}))
}
fn err(e:ae::Error)->String{format!("{e:?}")}
struct Adapter {
    id:ae::aegp::PluginId,
    effect:ae::aegp::EffectRefHandle,
    layer:ae::aegp::LayerHandle,
}
impl Adapter {
    fn stream(&mut self,i:usize)->Result<ae::aegp::StreamReferenceHandle,String>{
        self.validate_target()?;
        if i>=4 {return Err("Invalid hidden stream".into());}
        let suite=ae::aegp::suites::Stream::new().map_err(err)?;
        let stream=suite.new_effect_stream_by_index(self.effect,self.id,24+i as i32).map_err(err)?;
        if suite.stream_name(&stream,self.id,true).map_err(err)?!=NAMES[i] ||
            suite.stream_type(&stream).map_err(err)?!=ae::aegp::StreamType::TwoDSpatial {
            return Err("Hidden schema mismatch".into());
        }
        Ok(stream)
    }
}
impl Host for Adapter {
    fn validate_target(&mut self)->Result<(),String>{
        let flags=ae::aegp::suites::Layer::new().map_err(err)?.layer_flags(self.layer).map_err(err)?;
        if flags.contains(ae::aegp::LayerFlags::LOCKED) {return Err("Locked target".into());}
        let effects=ae::aegp::suites::Effect::new().map_err(err)?;
        let key=effects.installed_key_from_layer_effect(self.effect).map_err(err)?;
        if effects.effect_match_name(key).map_err(err)?!="com.elasticgrid.fx.warp" ||
            ae::aegp::suites::Stream::new().map_err(err)?.effect_num_param_streams(self.effect).map_err(err)?!=28 {
            return Err("Wrong effect schema".into());
        }
        Ok(())
    }
    fn read(&mut self,i:usize)->Result<Snapshot,String>{
        let stream=self.stream(i)?;let suite=ae::aegp::suites::Stream::new().map_err(err)?;
        let keys=ae::aegp::suites::Keyframe::new().map_err(err)?.stream_num_kfs(&stream).map_err(err)?;
        if keys<0 {return Err("Hidden stream cannot animate".into());}
        Ok(Snapshot {expression:suite.expression_string(&stream,self.id).map_err(err)?,
            enabled:suite.expression_state(&stream,self.id).map_err(err)?,keys:keys as usize})
    }
    fn begin_undo(&mut self)->Result<(),String>{ae::aegp::suites::Utility::new().map_err(err)?.start_undo_group("FSTR research binding").map_err(err)}
    fn end_undo(&mut self)->Result<(),String>{ae::aegp::suites::Utility::new().map_err(err)?.end_undo_group().map_err(err)}
    fn expression(&mut self,i:usize,value:&str)->Result<(),String>{
        let stream=self.stream(i)?;ae::aegp::suites::Stream::new().map_err(err)?.set_expression_string(&stream,self.id,value).map_err(err)
    }
    fn enable(&mut self,i:usize,value:bool)->Result<(),String>{
        let stream=self.stream(i)?;ae::aegp::suites::Stream::new().map_err(err)?.set_expression_state(&stream,self.id,value).map_err(err)
    }
}
// Caller has already restricted the operation to the owned fixture's newly
// added second effect, on the main thread, with a valid scoped suite context.
pub fn run(id:ae::aegp::PluginId,effect:ae::aegp::EffectRefHandle,layer:ae::aegp::LayerHandle)->Result<(),String>{
    let mut host=Adapter{id,effect,layer};let e=expressions();
    let first=binding_transaction::install(&mut host,&e).map_err(|x|format!("{x:?}"))?;
    if first!=Outcome::Installed {return Err("Expected pristine research target".into());}
    let repeat=binding_transaction::install(&mut host,&e).map_err(|x|format!("{x:?}"))?;
    if repeat!=Outcome::AlreadyInstalled {return Err("Idempotency failed".into());}
    Ok(())
}
