//! Source/FFI regression only. These tests do not run inside After Effects.
use super::*;

fn eligible(grid:&GridArb)->bool {
    initial_identity_values(grid,(4,4),1,0.0,0.0,0.5)
}

#[test]
fn first_neutral_frame_before_idle_reproduces_rejecting_baseline() {
    let grid=GridArb::default();
    assert!(eligible(&grid));
    // This is the exact policy called unconditionally by released 0.9.1.
    assert_eq!(comp_space_kind(0.0,true,true),Err(ae::Error::BadCallbackParameter));
    assert_eq!(resolve_comp_space_kind(0.0,true,true,eligible(&grid)),Ok(false));
    // Binding completion still selects the original paths, including 3D text.
    assert_eq!(resolve_comp_space_kind(1.0,true,true,true),Ok(false));
    assert_eq!(resolve_comp_space_kind(2.0,true,true,true),Ok(true));
    assert_eq!(resolve_comp_space_kind(3.0,true,true,true),Ok(false));
}

#[test]
fn pending_nonidentity_and_invalid_markers_still_fail_closed() {
    for identity in [false,true] {
        for kind in [-1.0,0.5,4.0,f64::NAN,f64::INFINITY] {
            assert!(resolve_comp_space_kind(kind,true,true,identity).is_err());
        }
    }
    assert_eq!(resolve_comp_space_kind(0.0,true,true,false),Err(ae::Error::BadCallbackParameter));
    assert_eq!(resolve_comp_space_kind(0.0,false,true,false),Ok(false));
    assert_eq!(resolve_comp_space_kind(0.0,true,false,false),Ok(false));
}

#[test]
fn even_one_ulp_of_grid_deformation_is_not_initial_identity() {
    for axis in 0..2 {
        for index in 1..5 {
            let mut grid=GridArb::default();
            let lines=if axis==0 {&mut grid.column_lines} else {&mut grid.row_lines};
            lines[index]=f32::from_bits(lines[index].to_bits()+1);
            assert!(!eligible(&grid));
            assert!(resolve_comp_space_kind(0.0,true,true,eligible(&grid)).is_err());
        }
    }
}

#[test]
fn invalid_grid_pins_dimensions_and_signed_zero_are_not_repaired() {
    let mut cases=Vec::new();
    let mut grid=GridArb::default(); grid.column_lines[0] = -0.0; cases.push(grid);
    let mut grid=GridArb::default(); grid.row_lines[2]=f32::NAN; cases.push(grid);
    let mut grid=GridArb::default(); grid.row_lines[2]=f32::INFINITY; cases.push(grid);
    let mut grid=GridArb::default(); grid.column_lines.pop(); cases.push(grid);
    let mut grid=GridArb::default(); grid.row_pins[2]=1; cases.push(grid);
    cases.push(GridArb {columns:3, ..GridArb::default()});
    cases.push(GridArb::uniform(3,4));
    for grid in cases {assert!(!eligible(&grid));}
}

#[test]
fn waves_easing_spacing_topology_and_four_corners_cannot_bypass_pending() {
    let grid=GridArb::default();
    for value in [f64::from_bits(1),0.001,15.0,-1.0,f64::NAN,f64::INFINITY,-0.0] {
        assert!(!initial_identity_values(&grid,(4,4),1,value,0.0,0.5));
        assert!(!initial_identity_values(&grid,(4,4),1,0.0,value,0.5));
    }
    for value in [0.0,0.1,25.0,f64::NAN,f64::INFINITY] {
        assert!(!initial_identity_values(&grid,(4,4),1,0.0,0.0,value));
    }
    for topology in [(0,4),(4,0),(5,4),(4,5),(50,50)] {
        assert!(!initial_identity_values(&grid,topology,1,0.0,0.0,0.5));
    }
    for mode in [0,2,3] {assert!(!initial_identity_values(&grid,(4,4),mode,0.0,0.0,0.5));}
}

// Use the actual production FFI renderer, not a reimplementation of its math.
trait Pixel: Copy + Default + std::fmt::Debug {
    const DEPTH:i32;
    fn sample(index:usize)->Self;
    fn same(self,other:Self)->bool;
}
impl Pixel for u8 {
    const DEPTH:i32=8;
    fn sample(i:usize)->Self {(i*71%256) as Self}
    fn same(self,other:Self)->bool {self==other}
}
impl Pixel for u16 {
    const DEPTH:i32=16;
    fn sample(i:usize)->Self {(i*7919%32769) as Self}
    fn same(self,other:Self)->bool {self==other}
}
impl Pixel for f32 {
    const DEPTH:i32=32;
    fn sample(i:usize)->Self {(i*7919%1021) as f32/128.0-2.0}
    fn same(self,other:Self)->bool {self.to_bits()==other.to_bits()}
}

fn exact_initial_pixels<T:Pixel>(easing:f32) {
    const W:usize=31; const H:usize=23;
    const IW:usize=19; const IH:usize=11;
    const IX:usize=3; const IY:usize=2;
    let grid=GridArb::default();
    assert!(eligible(&grid));
    let snapshot=SmartRenderSnapshot {
        grid, plane:plane::State::default(), tension_radius:3.0,falloff:2,
        elasticity_strength:1.0,min_spacing:0.005,stretch_easing:easing,easing_distance:0.25,
        wave_amplitude:0.0,wave_frequency:1.0,wave_phase:0.0,wave_speed:0.0,
        wave_axis:1,edge_mode:1,quality:2,time_seconds:0.0,
        canvas_width:W as i32,canvas_height:H as i32,
    };
    let mut p=snapshot.render_params(std::ptr::null_mut());
    p.abort_fn=None; p.threads=1;
    let stride=W*4+12; let input_stride=IW*4+8;
    let sentinel=T::sample(3);
    let mut source=vec![sentinel;stride*H];
    let mut crop=vec![sentinel;input_stride*IH];
    for y in 0..H {for x in 0..W*4 {source[y*stride+x]=T::default();}}
    for y in 0..IH {for x in 0..IW*4 {
        let value=T::sample(y*IW*4+x);
        crop[y*input_stride+x]=value;
        source[(y+IY)*stride+IX*4+x]=value;
    }}
    for sparse in [false,true] {
        let mut output=vec![sentinel;stride*H];
        let (input,iw,ih,pitch)=if sparse {
            p.input_origin_x=IX as i32; p.input_origin_y=IY as i32;
            (&crop,IW,IH,input_stride)
        } else {
            p.input_origin_x=0; p.input_origin_y=0; (&source,W,H,stride)
        };
        // SAFETY: typed allocations meet alignment, row strides include padding,
        // buffer extents match the supplied dimensions; all data outlive the call.
        let rc=unsafe {eg_render_frame_sparse(
            input.as_ptr().cast(),(pitch*std::mem::size_of::<T>()) as isize,iw as i32,ih as i32,
            output.as_mut_ptr().cast(),(stride*std::mem::size_of::<T>()) as isize,W as i32,H as i32,
            T::DEPTH,&p)};
        assert_eq!(rc,0);
        for (index,(actual,expected)) in output.iter().zip(&source).enumerate() {
            assert!(actual.same(*expected),"depth={} sparse={} index={} actual={:?} expected={:?}",
                T::DEPTH,sparse,index,actual,expected);
        }
    }
}
#[test] fn initial_identity_8bpc_dense_and_sparse() {for easing in [0.0,1.0] {exact_initial_pixels::<u8>(easing);}}
#[test] fn initial_identity_16bpc_dense_and_sparse() {for easing in [0.0,1.0] {exact_initial_pixels::<u16>(easing);}}
#[test] fn initial_identity_32bpc_extended_range_dense_and_sparse() {for easing in [0.0,1.0] {exact_initial_pixels::<f32>(easing);}}

#[test]
fn current_hidden_smoothing_default_is_a_proven_neutral_first_frame() {
    let grid=GridArb::default();
    assert!(initial_identity_values(&grid,(4,4),1,0.0,100.0,0.5));
    let mut edited=grid.clone();
    edited.column_lines[2]=f32::from_bits(edited.column_lines[2].to_bits()+1);
    assert!(!initial_identity_values(&edited,(4,4),1,0.0,100.0,0.5));
}
