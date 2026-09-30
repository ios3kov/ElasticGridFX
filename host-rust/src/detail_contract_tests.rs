//! Additional wire and production ABI coverage; not an AE execution test.
use super::*;
fn shape()->GridArb {GridArb {column_lines:vec![0.0,0.12,0.37,0.62,0.91,1.0],
    row_lines:vec![0.0,0.09,0.31,0.68,0.86,1.0],..GridArb::default()}}
fn bytes(g:&GridArb)->Vec<u8> {bincode::serde::encode_to_vec(g,bincode::config::legacy()).unwrap()}
fn settings()->EgElasticParams {EgElasticParams {tension_radius:3.0,falloff:2,elasticity_strength:1.0,min_spacing:0.005}}

#[test]
fn untouched_key_wire_is_exact_v3_and_bad_detail_payloads_are_rejected() {
    let g=shape();
    let legacy=(GRID_WIRE_MARKER | (3u16<<8) | g.columns,g.rows,
        &g.column_lines,&g.row_lines,&g.column_pins,&g.row_pins);
    assert_eq!(bytes(&g),bincode::serde::encode_to_vec(legacy,bincode::config::legacy()).unwrap());
    let valid:Vec<f32>=(0..detail_map::SAMPLES).map(|i|i as f32/256.0).collect();
    let mut cases=vec![vec![0.0],vec![0.0;256],vec![0.0;258]];
    for (index,value) in [(0,0.1),(256,0.9),(50,f32::NAN),(50,f32::INFINITY),(50,-0.2),(50,valid[49])] {
        let mut map=valid.clone();map[index]=value;cases.push(map);
    }
    for bad in cases {
        let key=GridArb {column_detail:bad,..shape()};
        assert!(!key.is_valid());
        assert!(bincode::serde::decode_from_slice::<GridArb,_>(&bytes(&key),bincode::config::legacy()).is_err());
    }
}

#[test]
fn added_row_control_and_combined_details_preserve_other_axis_and_other_key() {
    let base=shape();let retained=bytes(&base);let mut key=base.clone();
    let target=view_grid(&key,4,19).unwrap().row_lines[7]+0.003;
    assert!(drag_control(&mut key,&base,Drag {columns:false,visible:19,index:7,target,mapping:Mapping::default()},&settings()).unwrap());
    assert!(key.column_detail.is_empty());assert_eq!(key.column_lines,base.column_lines);
    assert!((view_grid(&key,4,19).unwrap().row_lines[7]-target).abs()<3e-6);
    let rows=key.row_detail.clone();
    let target=view_grid(&key,19,19).unwrap().column_lines[7]+0.003;
    assert!(drag_control(&mut key,&base,Drag {columns:true,visible:19,index:7,target,mapping:Mapping::default()},&settings()).unwrap());
    assert_eq!(key.row_detail,rows);assert_eq!(bytes(&base),retained);assert!(key.is_valid());
    let data=bytes(&key);
    let (restored,used): (GridArb,usize)=bincode::serde::decode_from_slice(&data,bincode::config::legacy()).unwrap();
    assert_eq!(restored,key);assert_eq!(used,data.len());
}

#[test]
fn empty_detail_dense_path_matches_the_original_export() {
    let grid=shape();
    let snapshot=SmartRenderSnapshot {grid,plane:plane::State::default(),tension_radius:3.0,falloff:2,
        elasticity_strength:1.0,min_spacing:0.005,stretch_easing:0.5,easing_distance:0.25,
        wave_amplitude:0.0,wave_frequency:1.0,wave_phase:0.0,wave_speed:0.0,wave_axis:1,
        edge_mode:1,quality:2,time_seconds:0.0,canvas_width:31,canvas_height:23};
    let mut p=snapshot.render_params(std::ptr::null_mut());p.abort_fn=None;p.threads=1;
    let input:Vec<u8>=(0..31*23*4).map(|i|(i*71%256) as u8).collect();
    let mut before=vec![0u8;input.len()];let mut after=before.clone();
    unsafe {
        assert_eq!(eg_render_frame(input.as_ptr().cast(),124,31,23,before.as_mut_ptr().cast(),124,31,23,8,&p),0);
        assert_eq!(eg_render_frame_detail(input.as_ptr().cast(),124,31,23,after.as_mut_ptr().cast(),124,31,23,8,&p,
            &detail_map::Maps::from_grid(&snapshot.grid),0),0);
    }
    assert_eq!(before,after);
}
