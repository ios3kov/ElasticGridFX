//! Real Rust-to-C++ ABI checks; not AE host/render acceptance.
use super::plane::*;

#[test]
fn derived_region_does_not_expose_public_corner_controls() {
    let corners=[0.0,0.0,100.0,0.0,100.0,100.0,0.0,100.0];
    let mut state=State {corners:Some(corners), ..State::default()};
    assert!(state.geometry().is_some());
    assert!(state.corner_controls().is_none());
    state.editable_corners=true;
    assert_eq!(state.corner_controls(),Some(corners));
    // Invalid user quads retain repair controls, but not valid geometry.
    state.corners=Some([0.0;8]);
    assert!(state.geometry().is_none());
    assert_eq!(state.corner_controls(),Some([0.0;8]));
    // Mode changes also revoke permission for an in-flight corner drag.
    state.editable_corners=false;
    assert!(state.corner_controls().is_none());
    assert!(State::default().corner_controls().is_none());
}

#[test]
fn shared_geometry_roundtrip_and_owned_snapshot() {
    let mut state=State {corners:Some([10.0,20.0,180.0,35.0,130.0,160.0,-15.0,115.0]), ..State::default()};
    let snapshot=state.clone();
    let geometry=snapshot.geometry().unwrap();
    state.corners=Some([0.0;8]);
    assert!(state.geometry().is_none());
    for y in 0..=20 {for x in 0..=20 {
        let local=(x as f64/20.0,y as f64/20.0);
        let screen=geometry.map(false,local.0,local.1).unwrap();
        let roundtrip=geometry.map(true,screen.0,screen.1).unwrap();
        assert!((roundtrip.0-local.0).abs()<1e-10);
        assert!((roundtrip.1-local.1).abs()<1e-10);
    }}
    assert!(geometry.map(false,f64::NAN,0.0).is_none());
    assert!(State::default().corners.is_none());
}

#[test]
fn saved_parameter_names_and_order_are_append_only() {
    use super::Params::*;
    let legacy=[Columns,Rows,GridState,TensionRadius,Falloff,ElasticityStrength,
        MinSpacing,StretchEasing,EasingDistance,WaveEnabled,WaveAmplitude,
        WaveFrequency,WavePhase,WaveSpeed,WaveAxis,EdgeMode,Quality];
    let names=["Columns","Rows","GridState","TensionRadius","Falloff","ElasticityStrength",
        "MinSpacing","StretchEasing","EasingDistance","WaveEnabled","WaveAmplitude",
        "WaveFrequency","WavePhase","WaveSpeed","WaveAxis","EdgeMode","Quality"];
    for (index,(param,name)) in legacy.into_iter().zip(names).enumerate() {
        assert_eq!(format!("{param:?}"),name); // host derives saved ID from this name
        assert_eq!(param as usize,index);
    }
    assert_eq!(PlaneMode as usize,17);
    assert_eq!(super::plane::CORNERS,[PlaneTopLeft,PlaneTopRight,PlaneBottomRight,PlaneBottomLeft]);
}

#[test]
fn plane_abi_layout_and_identity_roundtrip() {
    assert_eq!(std::mem::size_of::<Frame>(), 152);
    assert_eq!(std::mem::offset_of!(Frame, abort_fn), 136);
    assert_eq!(std::mem::size_of::<Image>(), 24);
    assert_eq!(std::mem::size_of::<Report>(), 24);
    let axis = [0.0, 0.5, 1.0];
    let mut input: Vec<f32> = (0..36).map(|v| v as f32 - 8.0).collect();
    let mut output = vec![0.0_f32; 36];
    let src = Image {pixels: input.as_mut_ptr().cast(), row_bytes: 48, width: 3, height: 3};
    let dst = Image {pixels: output.as_mut_ptr().cast(), row_bytes: 48, width: 3, height: 3};
    let mut frame = Frame {
        corners: [0.0,0.0,2.0,0.0,2.0,2.0,0.0,2.0], columns: axis.as_ptr(), rows: axis.as_ptr(),
        column_count: 3, row_count: 3, surface_units_x: 1.0, surface_units_y: 1.0,
        canvas_width: 3, canvas_height: 3, source_x: 0, source_y: 0, output_x: 0, output_y: 0,
        easing: 0.0, easing_distance: 0.25, abort_fn: None, abort_refcon: std::ptr::null_mut(),
    };
    let mut report = Report::default();
    // All slices/views remain valid, distinct and aligned for the synchronous call.
    assert_eq!(unsafe {eg_render_plane(&src,&dst,32,&frame,&mut report)}, 0);
    assert_eq!(output, input);
    assert_eq!(report.invalid_plane, 0);
    frame.corners=[0.2,0.3,1.8,0.0,1.9,1.8,0.1,1.7];
    assert_eq!(unsafe {eg_render_plane_region(&src,&dst,32,&frame,&mut report,1,0)},0);
    assert_eq!(output,input); // neutral skewed region is exact pass-through
    frame.corners=[0.0,0.0,2.0,0.0,2.0,2.0,0.0,2.0];
    let projected_source = frame.corners;
    assert_eq!(unsafe {eg_render_plane_between(&src,&dst,32,&frame,&mut report,1,0,projected_source.as_ptr())},0);
    assert_eq!(output,input);
    assert_eq!(unsafe {eg_render_plane_between(&src,&dst,32,&frame,&mut report,1,0,std::ptr::null())},1);
    assert_eq!(output,input);
    frame.corners = [0.0,0.0,2.5,0.0,2.5,2.5,0.0,2.5];
    assert_eq!(unsafe {eg_render_plane_projected(&src,&dst,32,&frame,&mut report,1,0,2.5,2.5)},0);
    for (actual,expected) in output.iter().zip(&input) {
        assert!((actual-expected).abs()<1e-5);
    }
    assert_eq!(unsafe {eg_render_plane_projected(&src,&dst,32,&frame,&mut report,1,0,f64::NAN,2.5)},1);
    frame.corners = [0.0,0.0,2.0,0.0,2.0,2.0,0.0,2.0];
    for quality in 0..2 {
        for edge in 0..3 {
            assert_eq!(unsafe {eg_render_plane_sampled(&src,&dst,32,&frame,&mut report,quality,edge)}, 0);
            assert_eq!(output, input);
        }
    }
    assert_eq!(unsafe {eg_render_plane_sampled(&src,&dst,32,&frame,&mut report,2,0)}, 1);
    assert_eq!(output, input);
    frame.corners = [0.0; 8];
    assert_eq!(unsafe {eg_render_plane(&src,&dst,32,&frame,&mut report)}, 0);
    assert_eq!(output, input);
    assert_eq!(report.invalid_plane, 1);
}
