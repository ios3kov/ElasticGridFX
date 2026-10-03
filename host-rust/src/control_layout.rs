//! Static viewer references, independent of retained animated deformation.
use super::*;

fn bounded<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<f32>, D::Error> {
    struct V;
    impl<'de> Visitor<'de> for V {
        type Value = Vec<f32>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("at most52 viewer references") }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<f32>, A::Error> {
            if seq.size_hint().is_some_and(|n| n>MAX_GUIDES+2) {
                return Err(serde::de::Error::custom("viewer layout exceeds guide limit"));
            }
            let mut out=Vec::new();
            while let Some(v)=seq.next_element::<f32>()? {
                if out.len()>=MAX_GUIDES+2 { return Err(serde::de::Error::custom("viewer layout exceeds guide limit")); }
                out.push(v);
            }
            Ok(out)
        }
    }
    d.deserialize_seq(V)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub(crate) struct State {
    #[serde(deserialize_with="bounded")]
    pub columns: Vec<f32>,
    #[serde(deserialize_with="bounded")]
    pub rows: Vec<f32>,
}
impl ae::ArbitraryData<State> for State {
    fn interpolate(&self, _other: &Self, _value: f64) -> Self { self.clone() }
}
impl State {
    pub fn valid_axis(refs: &[f32]) -> bool {
        refs.is_empty() || ((3..=MAX_GUIDES+2).contains(&refs.len()) && refs[0]==0.0 &&
            refs[refs.len()-1]==1.0 && refs.iter().all(|v|v.is_finite() && (0.0..=1.0).contains(v)) &&
            refs.windows(2).all(|p|p[1]>p[0]))
    }
    pub fn valid(&self) -> bool { Self::valid_axis(&self.columns) && Self::valid_axis(&self.rows) }
}

pub(crate) fn reflow(in_data: &ae::InData, params: &mut ae::Parameters<Params>, column: bool)
    -> Result<(), ae::Error> {
    let saved=grid_snapshot(params)?;
    let p=evaluated_params(params,*in_data,&saved)?;
    let (columns,rows)=plane::evaluated_axes(&p)?;
    let mut layout=params.get(Params::ControlLayout)?.as_arbitrary()?.value::<State>()?.clone();
    if !layout.valid() { return Err(ae::Error::BadCallbackParameter); }
    let param=if column {Params::Columns} else {Params::Rows};
    let count=params.get(param)?.as_slider()?.value().clamp(1,MAX_GUIDES as i32) as usize;
    let refs=control_grid::reflow_axis(if column {&columns} else {&rows},count,p.stretch_easing,p.easing_distance)?;
    if column {layout.columns=refs;} else {layout.rows=refs;}
    params.get_mut(Params::ControlLayout)?.as_arbitrary_mut()?.set_value(layout)?;
    Ok(())
}

pub(crate) fn freeze_for_drag(in_data: &ae::InData,params: &mut ae::Parameters<Params>,column: bool)
    -> Result<(),ae::Error> {
    let saved=grid_snapshot(params)?;
    let count=params.get(if column {Params::Columns} else {Params::Rows})?.as_slider()?.value() as usize;
    let layout=params.get(Params::ControlLayout)?.as_arbitrary()?.value::<State>()?.clone();
    let refs=if column {&layout.columns} else {&layout.rows};
    let retained=if column {saved.columns} else {saved.rows} as usize;
    if refs.len()!=count+2 && count!=retained {reflow(in_data,params,column)?;}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_rejects_unbounded_nonfinite_crossed_and_outside_references() {
        for refs in [vec![0.0,1.0],vec![0.0,f32::NAN,1.0],vec![0.0,0.5,0.4,1.0],
            vec![0.0,1.5,1.0],vec![0.0;53]] { assert!(!State::valid_axis(&refs)); }
        assert!(State::default().valid());
        assert!(State::valid_axis(&[0.0,0.3,0.9,1.0]));
        let oversized=State {columns:vec![0.0;53],rows:Vec::new()};
        let wire=bincode::serde::encode_to_vec(oversized,bincode::config::legacy()).unwrap();
        assert!(bincode::serde::decode_from_slice::<State,_>(&wire,bincode::config::legacy()).is_err());
    }
}
