//! Retained hidden placeholder for the former Version row's persistent disk ID.
//! Version is shown only by About; keep serialized data unchanged for old projects.
use super::*;

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq,PartialOrd)]
pub(crate) struct Data(u8);
impl ae::ArbitraryData<Data> for Data {
    fn interpolate(&self,_other:&Self,_value:f64)->Self {Self::default()}
}
