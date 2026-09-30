put('host-rust/src/guide_density_tests.rs', [
    (0,0,r'''//! Pure state/production CPU FFI tests, NOT target-AE acceptance.
use super::*;

fn shape() -> GridArb {
    GridArb { column_lines:vec![0.0, 0.12, 0.37, 0.62, 0.91, 1.0],
        row_lines:vec![0.0, 0.09, 0.31, 0.68, 0.86, 1.0], ..GridArb::default() }
}
fn bytes(grid: &GridArb) -> Vec<u8> {
    bincode::serde::encode_to_vec(grid, bincode::config::legacy()).unwrap()
}
fn settings() -> EgElasticParams {
    EgElasticParams { tension_radius: 3.0, falloff: 2, elasticity_strength: 1.0, min_spacing: 0.005 }
}

#[test]
fn every_density_has_exact_count_and_keeps_or_subdivides_original_anchors() {
    for stored in 1..=50 {
        let grid = GridArb::uniform(stored, stored);
        for count in 1..=50 {
            let refs = guides(stored, count).unwrap();
            assert_eq!(refs.len(), count + 2);
            let view = view_grid(&grid, count, count).unwrap();
            assert_eq!(view.column_lines.len(), count + 2);
            assert!(view.is_valid());
            if count >= stored {
                for value in &grid.column_lines { assert!(view.column_lines.contains(value)); }
            } else {
                for value in &view.column_lines { assert!(grid.column_lines.contains(value)); }
            }
        }
    }
}

#[test]
fn increase_inserts_equal_subintervals_and_decrease_only_hides_data() {
    let grid = shape();
    let original = bytes(&grid);
    // 4 -> 9 inserts one midpoint in each of the five intervals.
    let view = view_grid(&grid, 9, 9).unwrap();
    for i in 0..5 {
        assert_eq!(view.column_lines[2*i].to_bits(), grid.column_lines[i].to_bits());
        assert!((view.column_lines[2*i+1] - (grid.column_lines[i]+grid.column_lines[i+1])/2.0).abs() < 1e-7);
    }
    for count in [1, 50, 2, 19, 4, 7, 50, 4] {
        let _ = view_grid(&grid, count, 51-count).unwrap();
        assert_eq!(bytes(&grid), original);
        assert_eq!(bytes(&render_grid(&grid).unwrap()), original);
    }
    assert_eq!(bytes(&view_grid(&grid, 4, 4).unwrap()), original);
}

#[test]
fn density_changes_do_not_touch_keys_or_interpolated_values() {
    use ae::ArbitraryData;
    let a = shape();
    let mut b = shape();
    b.column_lines[1] = 0.22; b.row_lines[3] = 0.74;
    let saved = (bytes(&a), bytes(&b));
    for frame in 0..=100 {
        let value = a.interpolate(&b, frame as f64 / 100.0);
        let original = bytes(&value);
        for count in [1, 2, 3, 4, 9, 19, 32, 50, 4] {
            let _ = view_grid(&value, count, 51-count).unwrap();
            assert_eq!(bytes(&render_grid(&value).unwrap()), original);
        }
    }
    assert_eq!((bytes(&a), bytes(&b)), saved);
}

#[test]
fn same_density_drag_is_exactly_the_existing_production_algorithm() {
    for count in [1, 4, 9, 50] {
        for kind in [1, 2, 3, 4] {
            let mut g = GridArb::uniform(count, count);
            let mut expected = g.clone();
            let config = EgElasticParams { falloff: kind, ..settings() };
            let index = (count / 2).max(1);
            let target = g.column_lines[index] + 0.06;
            unsafe { assert_eq!(eg_drag_axis(expected.column_lines.as_mut_ptr(),
                expected.column_pins.as_mut_ptr(), expected.column_lines.len() as i32,
                index as i32, target, &config), 0); }
            assert!(drag_control(&mut g, &GridArb::uniform(count,count), Drag {columns:true,visible:count,index,target,mapping:Mapping::default()}, &config).unwrap());
            assert_eq!(bytes(&g), bytes(&expected));
        }
    }
}

#[test]
fn added_virtual_guides_are_draggable_without_replacing_the_key_basis() {
    for visible in [1, 2, 3, 5, 9, 19, 50] {
        for index in 1..=visible {
            let mut g = shape();
            let view = view_grid(&g, visible, 4).unwrap();
            let target = view.column_lines[index] + 0.0001;
            let old_rows = g.row_lines.clone();
            assert!(drag_control(&mut g, &shape(), Drag {columns:true,visible,index,target,mapping:Mapping::default()}, &settings()).unwrap());
            assert_eq!((g.columns, g.rows), (4, 4));
            assert_eq!(g.row_lines, old_rows);
            assert!(g.is_valid());
            let moved = view_grid(&g, visible, 4).unwrap().column_lines[index];
            assert!((moved-target).abs() < 2e-7, "count={visible}, index={index}, got={moved}, target={target}");
        }
    }
}

#[test]
fn zero_motion_and_zero_strength_never_write_a_different_arbitrary_value() {
    for visible in [1, 4, 9, 50] {
        let mut g = shape(); let saved = bytes(&g);
        for index in 1..=visible {
            let target = view_grid(&g, visible, 4).unwrap().column_lines[index];
            assert!(!drag_control(&mut g, &shape(), Drag {columns:true,visible,index,target,mapping:Mapping::default()}, &settings()).unwrap());
            assert!(!drag_control(&mut g, &shape(), Drag {columns:true,visible,index,target:target+0.1,mapping:Mapping::default()},
                &EgElasticParams { elasticity_strength: 0.0, ..settings() }).unwrap());
        }
        assert_eq!(bytes(&g), saved);
    }
}

#[test]
fn zero_radius_virtual_control_and_extreme_pointer_remain_valid() {
    for radius in [0.0, 0.01, 3.0, 20.0] {
        let config = EgElasticParams { tension_radius: radius, ..settings() };
        let mut g = shape();
        for target in [-1.0e30_f32, 1.0e30, 0.15, 0.7, 0.12, 0.18] {
            drag_control(&mut g, &shape(), Drag {columns:true,visible:19,index:5,target,mapping:Mapping::default()}, &config).unwrap();
            assert!(g.is_valid());
            assert_eq!(g.columns, 4);
        }
    }
}

#[test]
fn invalid_inputs_fail_without_mutation_and_counts_are_not_clamped_silently() {
    let mut g = shape(); let before = bytes(&g);
    for count in [0, 51, usize::MAX] {
        assert!(view_grid(&g, count, 4).is_err());
        assert!(drag_control(&mut g, &shape(), Drag {columns:true,visible:count,index:1,target:0.2,mapping:Mapping::default()}, &settings()).is_err());
    }
    for target in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(drag_control(&mut g, &shape(), Drag {columns:true,visible:19,index:1,target,mapping:Mapping::default()}, &settings()).is_err());
    }
    assert_eq!(bytes(&g), before);
    g.column_lines[2] = f32::NAN;
    assert!(render_grid(&g).is_err());
    assert!(view_grid(&g, 4, 4).is_err());
}

trait Pixel: Copy + Default + PartialEq + std::fmt::Debug {
    const DEPTH: i32;
    fn sample(i: usize) -> Self;
    fn bits(self) -> u32;
}
impl Pixel for u8 { const DEPTH:i32=8; fn sample(i:usize)->Self {(i*71%256) as Self} fn bits(self)->u32 {u32::from(self)} }
impl Pixel for u16 { const DEPTH:i32=16; fn sample(i:usize)->Self {(i*7919%32769) as Self} fn bits(self)->u32 {u32::from(self)} }
impl Pixel for f32 { const DEPTH:i32=32; fn sample(i:usize)->Self {(i*7919%1021) as f32/128.0-2.0} fn bits(self)->u32 {self.to_bits()} }
fn snapshot(grid: GridArb) -> SmartRenderSnapshot {
    SmartRenderSnapshot {grid, plane:plane::State::default(), tension_radius:3.0,falloff:2,
        elasticity_strength:1.0,min_spacing:0.005,stretch_easing:0.63,easing_distance:0.25,
        wave_amplitude:0.12,wave_frequency:1.3,wave_phase:42.0,wave_speed:0.8,
        wave_axis:1,edge_mode:1,quality:2,time_seconds:0.73,canvas_width:31,canvas_height:23}
}
fn render_pixels<T:Pixel>(grid:&GridArb, quality:i32, time:f32) -> Vec<T> {
    let mut state = snapshot(render_grid(grid).unwrap());
    state.quality = quality; state.time_seconds = time;
    let mut p = state.render_params(std::ptr::null_mut()); p.abort_fn = None; p.threads = 1;
    let stride = 31*4+8;
    let input:Vec<T> = (0..stride*23).map(T::sample).collect();
    let mut output=vec![T::default();stride*23];
    let pitch=(stride*std::mem::size_of::<T>()) as isize;
    // Production CPU function: neither a mock nor a duplicate renderer.
    let code=unsafe {eg_render_frame_detail(input.as_ptr().cast(),pitch,31,23,
        output.as_mut_ptr().cast(),pitch,31,23,T::DEPTH,&p,&detail_map::Maps::from_grid(&state.grid),1)};
    assert_eq!(code,0);
    output
}
fn render_invariance<T:Pixel>() {
    use ae::ArbitraryData;
    let a=shape(); let mut b=shape(); b.column_lines[2]=0.43; b.row_lines[1]=0.16;
    for frame in 0..=6 { for quality in [1,2] {
        let time=frame as f64/6.0;
        let grid=a.interpolate(&b,time);
        let original=render_pixels::<T>(&grid,quality,time as f32);
        for count in [1,2,4,9,19,50,4] {
            let _=view_grid(&grid,count,51-count).unwrap();
            let actual=render_pixels::<T>(&grid,quality,time as f32);
            assert!(actual.iter().zip(&original).all(|(a,b)|a.bits()==b.bits()));
        }
    }}
}
#[test] fn rendered_8bpc_animation_is_density_invariant() {render_invariance::<u8>();}
#[test] fn rendered_16bpc_animation_is_density_invariant() {render_invariance::<u16>();}
#[test] fn rendered_32bpc_animation_is_density_invariant() {render_invariance::<f32>();}

#[test]
fn added_controls_edit_local_geometry_without_moving_original_anchors_at_zero_radius() {
    let mut g=shape(); let base=g.clone();
    let before=view_grid(&g,9,4).unwrap();
    let target=before.column_lines[1]+0.0001;
    assert!(drag_control(&mut g,&base,Drag {columns:true,visible:9,index:1,target,mapping:Mapping::default()},
        &EgElasticParams {tension_radius:0.0,..settings()}).unwrap());
    let after=view_grid(&g,9,4).unwrap();
    assert_eq!(g.column_lines,base.column_lines);
    assert_eq!(g.row_lines,base.row_lines);
    assert_eq!(g.column_detail.len(),detail_map::SAMPLES);
    assert!((after.column_lines[1]-target).abs()<2e-7);
    for i in 2..before.column_lines.len() {assert_eq!(after.column_lines[i].to_bits(),before.column_lines[i].to_bits());}
    // Local geometric independence above; production pixel response is tested
    // by the wider-field case below (a narrow support can miss a 31px fixture).
}

#[test]
fn detail_keys_roundtrip_and_interpolate_without_embedding_visible_counts() {
    use ae::ArbitraryData;
    let a=shape(); let mut b=shape();
    drag_control(&mut b,&a,Drag {columns:true,visible:19,index:7,target:0.38,mapping:Mapping::default()},&settings()).unwrap();
    let encoded=bytes(&b);
    let (decoded,used): (GridArb,usize)=bincode::serde::decode_from_slice(&encoded,bincode::config::legacy()).unwrap();
    assert_eq!(used,encoded.len()); assert_eq!(b,decoded);
    assert_eq!((b.columns,b.rows),(4,4));
    let original=bytes(&a);
    let (old,used): (GridArb,usize)=bincode::serde::decode_from_slice(&original,bincode::config::legacy()).unwrap();
    assert_eq!(used,original.len());assert_eq!(bytes(&old),original);
    for frame in 0..=100 {
        let value=a.interpolate(&b,frame as f64/100.0);assert!(value.is_valid());
        let saved=bytes(&value);
        for count in [1,2,4,9,19,50,4] {
            view_grid(&value,count,count).unwrap();assert_eq!(bytes(&value),saved);
        }
    }
}

#[test]
fn detail_deformation_pixels_stay_exact_for_all_density_changes() {
    let mut g=shape();let base=g.clone();
    drag_control(&mut g,&base,Drag {columns:true,visible:19,index:7,target:0.38,mapping:Mapping::default()},&settings()).unwrap();
    let expected=render_pixels::<f32>(&g,2,0.7);
    assert_ne!(expected,render_pixels::<f32>(&base,2,0.7));
    let keys=bytes(&g);
    for count in [1,50,2,9,19,4] {
        view_grid(&g,count,count).unwrap();
        assert_eq!(render_pixels::<f32>(&g,2,0.7),expected); assert_eq!(bytes(&g),keys);
    }
}

#[test]
fn mapping_matches_actual_inverse_with_easing_and_detail_field() {
    let mut g=shape();let base=g.clone();let mapping=Mapping {easing:0.8,distance:0.45};
    let before=view_grid_mapped(&g,19,4,mapping).unwrap();let target=before.column_lines[7]+0.002;
    drag_control(&mut g,&base,Drag {columns:true,visible:19,index:7,target,mapping},&settings()).unwrap();
    let moved=view_grid_mapped(&g,19,4,mapping).unwrap().column_lines[7];
    assert!((moved-target).abs()<3e-6);
    assert_eq!(std::mem::size_of::<detail_map::Maps>(),32);
}
'''),
])
