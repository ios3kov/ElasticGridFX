//! Actual shared native sampler/dispatch tests, not AE runtime acceptance.
use super::*;

#[test]
fn old_mode_values_and_new_choices_are_unambiguous() {
    assert_eq!(Mode::from_value(1),Ok(Mode::Comp));
    assert_eq!(Mode::from_value(2),Ok(Mode::Flat));
    assert_eq!(Mode::from_value(3),Ok(Mode::Layer));
    assert_eq!(Mode::from_value(4),Ok(Mode::Perspective));
    for value in [-1,0,5,i32::MAX] {assert!(Mode::from_value(value).is_err());}
    // Non-rounded source boundaries agree with the PF-scaled public corners.
    assert_eq!(source_rectangle(319.0*0.5,241.0*0.5,7.0,3.0).unwrap(),
        [-7.0,-3.0,152.5,-3.0,152.5,117.5,-7.0,117.5]);
    for size in [0.0,-1.0,f64::NAN,f64::INFINITY] {
        assert!(source_rectangle(size,10.0,0.0,0.0).is_err());
    }
}

trait Pixel: Copy+Default+PartialEq+std::fmt::Debug {
    const DEPTH:i32;
    fn sample(i:usize)->Self;
    fn value(self)->f64;
}
impl Pixel for u8 {
    const DEPTH:i32=8;
    fn sample(i:usize)->Self {(i%180+30) as Self}
    fn value(self)->f64 {f64::from(self)}
}
impl Pixel for u16 {
    const DEPTH:i32=16;
    fn sample(i:usize)->Self {(i%20000+1000) as Self}
    fn value(self)->f64 {f64::from(self)}
}
impl Pixel for f32 {
    const DEPTH:i32=32;
    fn sample(i:usize)->Self {i as f32/512.0-0.4}
    fn value(self)->f64 {f64::from(self)}
}

fn check_modes<T:Pixel>() {
    const W:usize=16;const H:usize=12;
    let pitch=W*4+8;
    let mut input=vec![T::default();pitch*H];
    for y in 0..H {for x in 0..W {for c in 0..4 {
        input[y*pitch+x*4+c]=T::sample(y*W*4+x*4+c);
    }}}
    let mut output=vec![T::default();pitch*H];
    let image=|pixels:*mut T|Image {pixels:pixels.cast(),width:W as i32,height:H as i32,
        row_bytes:(pitch*std::mem::size_of::<T>()) as isize};
    let src=image(input.as_mut_ptr());let dst=image(output.as_mut_ptr());
    let source=source_rectangle(W as f64,H as f64,0.0,0.0).unwrap();
    let mut axis=[0.0,0.5,1.0];
    let rows=[0.0,0.5,1.0];
    // Source rectangle is pinned into a smaller translated rectangle.
    // Destination (5,4) must sample source (6,4); neutral Flat is unchanged.
    let target=[2.0,2.0,10.0,2.0,10.0,8.0,2.0,8.0];
    let mut frame=Frame {corners:target,columns:axis.as_mut_ptr(),rows:rows.as_ptr(),
        column_count:3,row_count:3,surface_units_x:1.0,surface_units_y:1.0,
        canvas_width:W as i32,canvas_height:H as i32,source_x:0,source_y:0,output_x:0,output_y:0,
        easing:0.0,easing_distance:0.25,abort_fn:None,abort_refcon:std::ptr::null_mut()};
    let mut state=State {corners:Some(target),editable_corners:true,..State::default()};
    let mut report=Report::default();
    for quality in 0..=1 {for edge in 0..=2 {
        state.render_kind=RenderKind::Region;
        assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(quality,edge),&state),0);
        assert_eq!(output,input); // no new perspective in legacy Flat
        state.render_kind=RenderKind::Perspective;state.source_corners=Some(source);
        assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(quality,edge),&state),0);
        assert!(output[..4].iter().all(|p|*p==T::default())); // outside transparent
        for c in 0..4 {
            assert!((output[4*pitch+5*4+c].value()-input[4*pitch+6*4+c].value()).abs()<1e-5);
        }
        // Correct source-boundary convention: Fit Layer gives exact identity.
        frame.corners=source;state.corners=Some(source);
        assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(quality,edge),&state),0);
        assert_eq!(output,input);
        frame.corners=target;state.corners=Some(target);
    }}
    // New 2D Layer mode also warps coverage beyond its vector bounds instead
    // of leaving the old unchanged fringe. This dispatch is independent of comp_space.
    axis[1]=0.75;frame.columns=axis.as_ptr();
    state.render_kind=RenderKind::Layer;state.editable_corners=false;
    assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(1,0),&state),0);
    let layer_output=output.clone();
    state.render_kind=RenderKind::Region;
    assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(1,0),&state),0);
    assert_ne!(output,layer_output);
    assert_eq!(&output[4*pitch..4*pitch+4],&input[4*pitch..4*pitch+4]);
    // Invalid user quad preserves the existing fallback instead of failing a render.
    frame.corners=[0.0;8];state.corners=Some(frame.corners);state.render_kind=RenderKind::Perspective;
    assert_eq!(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut report,(1,0),&state),0);
    assert_eq!(report.invalid_plane,1);assert_eq!(output,input);
}
#[test] fn three_depths_both_qualities_all_edges_use_shared_mode_dispatch(){
    check_modes::<u8>();check_modes::<u16>();check_modes::<f32>();
}

#[test] fn perspective_uses_projected_text_source_and_owned_snapshot() {
    let target=[2.0,2.0,10.0,2.0,10.0,8.0,2.0,8.0];
    let basis=[4.0,2.0,12.0,2.0,12.0,8.0,4.0,8.0];
    let mut state=State {corners:Some(target),editable_corners:true,comp_space:true,
        parameter_basis:Some(basis),source_corners:Some(basis),render_kind:RenderKind::Perspective};
    let snapshot=state.clone();state.source_corners=Some([0.0;8]);
    let axis=[0.0,0.5,1.0];let mut input:Vec<f32>=(0..16*12*4).map(|i|i as f32/512.0).collect();
    let mut out=vec![0.0;input.len()];
    let src=Image {pixels:input.as_mut_ptr().cast(),row_bytes:16*4*4,width:16,height:12};
    let dst=Image {pixels:out.as_mut_ptr().cast(),..src};
    let frame=Frame {corners:target,columns:axis.as_ptr(),rows:axis.as_ptr(),column_count:3,row_count:3,
        surface_units_x:1.0,surface_units_y:1.0,canvas_width:16,canvas_height:12,source_x:0,source_y:0,
        output_x:0,output_y:0,easing:0.0,easing_distance:0.25,abort_fn:None,abort_refcon:std::ptr::null_mut()};
    let mut report=Report::default();
    assert_eq!(dispatch_render(&src,&dst,32,&frame,&mut report,(1,0),&snapshot),0);
    // Destination5,4 maps to unedited projected source7,4, not canvas6,4.
    assert_eq!(&out[(4*16+5)*4..(4*16+6)*4],&input[(4*16+7)*4..(4*16+8)*4]);
    assert_eq!(dispatch_render(&src,&dst,32,&frame,&mut report,(1,0),&state),1);
}
