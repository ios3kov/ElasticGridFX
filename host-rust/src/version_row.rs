//! Read-only version in Effect Controls; no animated/render dependency.
use super::*;
use ae::drawbot::{ColorRgba,PointF32,TextAlignment,TextTruncation};

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq,PartialOrd)]
pub(crate) struct Data(u8);
impl ae::ArbitraryData<Data> for Data {
    fn interpolate(&self,_other:&Self,_value:f64)->Self {Self::default()}
}
#[cfg(not(feature = "render-diagnostics"))]
const LABEL: &str = concat!(env!("CARGO_PKG_VERSION"), " Dev 1");
#[cfg(feature = "render-diagnostics")]
const LABEL: &str = concat!(env!("CARGO_PKG_VERSION"), " Dev 2");
pub(crate) fn draw(event: &mut ae::EventExtra) -> Result<(),ae::Error> {
    if event.effect_area()!=ae::EffectArea::Title {return Ok(());}
    let frame=event.param_title_frame();
    let left=frame.left as f32+(frame.right-frame.left) as f32*0.5+16.0;
    let width=frame.right as f32-left-5.0;
    if width<60.0 {return Ok(());}
    let bot=event.context_handle().drawing_reference()?;
    let supplier=bot.supplier()?;let surface=bot.surface()?;
    let size=supplier.default_font_size()?;let font=supplier.new_default_font(size)?;
    let c=ae::pf::suites::App::new()?.color(ae::AppColorType::ButtonText)?;
    let brush=supplier.new_brush(&ColorRgba {red:c.red as f32/65535.0,
        green:c.green as f32/65535.0,blue:c.blue as f32/65535.0,alpha:1.0})?;
    surface.draw_string(&brush,&font,LABEL,
        &PointF32{x:left,y:frame.top as f32+(16.0+size)*0.5-2.0},
        TextAlignment::Left,TextTruncation::End,width)?;
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);Ok(())
}
