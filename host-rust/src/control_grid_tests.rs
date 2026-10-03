//! Real Rust data/FFI checks; no After Effects process is used by these tests.
use super::*;
use ae::ArbitraryData;

fn bytes(grid: &GridArb) -> Vec<u8> {
    bincode::serde::encode_to_vec(grid,bincode::config::legacy()).unwrap()
}

#[test]
fn counts_change_neither_key_bytes_nor_interpolated_render_lattice() {
    let mut left = GridArb::uniform(7,4);
    left.column_lines[2]=0.28; left.row_lines[2]=0.45;
    let mut right=left.clone(); right.column_lines[4]=0.57; right.row_lines[3]=0.68;
    let key_bytes=[bytes(&left),bytes(&right)];
    for time in [0.0,0.01,0.25,0.5,0.75,0.99,1.0] {
        let frame=left.interpolate(&right,time);
        let expected=bytes(&frame);
        for x in [1,4,7,19,50,1,50,7] {for y in [1,4,19,50,4] {
            let controls=view(&frame,(x,y),0.75,0.5).unwrap();
            assert_eq!(controls.grid.column_lines.len(),x+2);
            assert_eq!(controls.grid.row_lines.len(),y+2);
            assert_eq!(bytes(&retained_grid(&frame).unwrap()),expected);
            assert_eq!([bytes(&left),bytes(&right)],key_bytes);
        }}
    }
}

#[test]
fn added_guides_keep_original_knots_and_removed_guides_keep_hidden_data() {
    let mut saved=GridArb::default(); saved.column_lines[2]=0.47;
    let snapshot=bytes(&saved);
    let mut last=Vec::new();
    for count in 1..=50 {
        let controls=view(&saved,(count,count),0.0,0.25).unwrap();
        for ref_index in &last {assert!(controls.column_refs.contains(ref_index));}
        if count>=4 {for index in 0..6 {assert!(controls.column_refs.contains(&(index as f32)));}}
        last=controls.column_refs;
        assert_eq!(bytes(&saved),snapshot);
    }
    assert_eq!(view(&saved,(4,4),0.0,0.25).unwrap().grid,saved);
}

#[test]
fn click_or_zero_drag_cannot_change_animated_bytes() {
    let mut saved=GridArb::default();saved.column_lines[2]=0.47;
    let before=bytes(&saved);
    let controls=view(&saved,(50,50),0.0,0.25).unwrap();
    let elastic=EgElasticParams {tension_radius:3.0,falloff:2,elasticity_strength:1.0,min_spacing:0.005};
    for index in 1..51 {
        drag(&mut saved.column_lines,&saved.column_pins,controls.column_refs[index],0.0,&elastic).unwrap();
        assert_eq!(bytes(&saved),before);
    }
}

#[test]
fn all_added_controls_are_usable_without_resampling_the_saved_lattice() {
    let initial=GridArb::default();
    let controls=view(&initial,(50,50),0.0,0.25).unwrap();
    let elastic=EgElasticParams {tension_radius:0.0,falloff:2,elasticity_strength:1.0,min_spacing:0.005};
    for reference in controls.column_refs.iter().skip(1).take(50) {
        let mut saved=initial.clone();
        drag(&mut saved.column_lines,&saved.column_pins,*reference,0.001,&elastic).unwrap();
        assert!(saved.is_valid()); assert_ne!(saved,initial);
        assert_eq!(saved.columns,initial.columns); assert_eq!(saved.row_lines,initial.row_lines);
        assert_eq!(saved.column_lines.len(),initial.column_lines.len());
    }
}

#[test]
fn malformed_lattice_and_invalid_counts_are_not_silently_repaired() {
    let initial=GridArb::default();
    for counts in [(0,4),(4,0),(51,4),(4,51)] {assert!(view(&initial,counts,0.0,0.25).is_err());}
    let mut invalid=initial.clone();invalid.column_lines[2]=f32::NAN;
    assert!(retained_grid(&invalid).is_err());
    assert!(view(&invalid,(4,4),0.0,0.25).is_err());
}

#[test]
fn every_density_uses_a_bounded_catalog_of_fifty_exact_references() {
    // U5 design prerequisite: retaining editable moves must not require an
    // unbounded gesture log or change reference identity after a density edit.
    // Exercise the real C++ control reader for every supported base topology.
    for guides in 1..=MAX_GUIDES {
        let initial = GridArb::uniform(guides, guides);
        let before = bytes(&initial);
        let catalog = view(&initial, (MAX_GUIDES, MAX_GUIDES), 0.0, 0.25).unwrap();
        let exact: std::collections::HashSet<_> = catalog.column_refs.iter()
            .map(|reference| reference.to_bits()).collect();
        assert_eq!(exact.len(), MAX_GUIDES + 2);
        for count in 1..=MAX_GUIDES {
            let controls = view(&initial, (count, count), 0.0, 0.25).unwrap();
            for reference in controls.column_refs.iter().chain(&controls.row_refs) {
                assert!(exact.contains(&reference.to_bits()),
                    "base={guides}, density={count}, ref={reference}");
            }
            assert_eq!(bytes(&initial), before);
        }
    }
}

#[test]
fn density_reflow_is_visually_uniform_without_changing_deformation_or_keys() {
    let mut saved=GridArb::uniform(4,4);
    saved.column_lines[1]=0.03;saved.column_lines[2]=0.18;
    saved.column_lines[3]=0.71;saved.column_lines[4]=0.95;
    let original=bytes(&saved);
    for easing in [0.0,0.5,1.0] {for count in 1..=MAX_GUIDES {
        let layout=reflow_axis(&saved.column_lines,count,easing,0.25).unwrap();
        let (positions,refs)=sample_layout(&saved.column_lines,&layout,easing,0.25).unwrap();
        for (i,p) in positions.iter().enumerate() {
            assert!((*p-i as f32/(count+1) as f32).abs()<0.00001,
                "density={count}, easing={easing}, index={i}, position={p}");
        }
        assert!(refs.windows(2).all(|p|p[1]>p[0]));
        assert_eq!(bytes(&saved),original);
    }}
}

#[test]
fn redistributed_handles_move_and_keep_their_source_identity_after_drag() {
    let mut saved=GridArb::default();saved.column_lines[2]=0.47;
    let layout=reflow_axis(&saved.column_lines,7,0.0,0.25).unwrap();
    let (before,refs)=sample_layout(&saved.column_lines,&layout,0.0,0.25).unwrap();
    let elastic=EgElasticParams {tension_radius:0.0,falloff:2,elasticity_strength:1.0,min_spacing:0.0};
    drag(&mut saved.column_lines,&saved.column_pins,refs[3],0.01,&elastic).unwrap();
    let (after,after_refs)=sample_layout(&saved.column_lines,&layout,0.0,0.25).unwrap();
    assert_eq!(refs,after_refs);assert!(after[3]>before[3]);assert!(saved.is_valid());
    let wire=bincode::serde::encode_to_vec(control_layout::State {columns:layout,rows:Vec::new()},bincode::config::legacy()).unwrap();
    let (reopened,_)=bincode::serde::decode_from_slice::<control_layout::State,_>(&wire,bincode::config::legacy()).unwrap();
    assert_eq!(sample_layout(&saved.column_lines,&reopened.columns,0.0,0.25).unwrap().0,after);
}
