//! Real Rust-to-C++ ABI checks; not AE host/render acceptance.
use std::ffi::c_void;
#[repr(C)]
struct Frame {
    corners: [f64; 8],
    columns: *const f32,
    rows: *const f32,
    column_count: i32,
    row_count: i32,
    surface_units_x: f64,
    surface_units_y: f64,
    canvas_width: i32,
    canvas_height: i32,
    source_x: i32,
    source_y: i32,
    output_x: i32,
    output_y: i32,
    easing: f32,
    easing_distance: f32,
    abort_fn: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    abort_refcon: *mut c_void,
}
#[repr(C)]
struct Image { pixels: *mut c_void, row_bytes: isize, width: i32, height: i32 }
#[repr(C)]
#[derive(Default)]
struct Report { invalid_plane: i32, reserved: i32, outside_pixels: u64, invalid_projection_pixels: u64 }
unsafe extern "C" {
    fn eg_render_plane(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report) -> i32;
    fn eg_render_plane_sampled(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32) -> i32;
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
