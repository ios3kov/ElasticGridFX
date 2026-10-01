//! Actual production Fit + native geometry/render checks; not AE host evidence.
use super::*;
use crate::plane::{eg_render_plane_region, Frame, Geometry, Image, Report};

fn quad(points: [(f32, f32); 4], scale: f64) -> [f64; 8] {
    std::array::from_fn(|i| f64::from(if i % 2 == 0 { points[i / 2].0 } else { points[i / 2].1 }) * scale)
}

#[test]
fn all_eight_fit_values_match_initial_boundaries_and_order() {
    for (w, h) in [(1920, 1080), (1080, 1920), (1919, 1079), (1, 1),
                   (1, 1080), (1920, 1), (3840, 2160), (32767, 32767)] {
        let actual = points(w, h).unwrap();
        let expected = [(0.0, 0.0), (w as f32, 0.0), (w as f32, h as f32), (0.0, h as f32)];
        assert_eq!(actual, expected);
        assert!(Geometry::new(&quad(actual, 1.0)).is_some());
    }
}

#[test]
fn fit_refuses_invalid_or_unrepresentable_dimensions() {
    for (w, h) in [(0, 1080), (1920, 0), (-1, 1080), (1920, -1),
                   (32768, 1080), (1920, 32768), (i32::MAX, i32::MAX)] {
        assert!(points(w, h).is_err());
    }
}

#[test]
fn repeated_fit_does_not_flag_unchanged_point_keys() {
    let target = points(1920, 1080).unwrap();
    let initial = target.map(|(x, y)| (f64::from(x), f64::from(y)));
    assert_eq!(changes(&initial, &target), [false; 4]);
    let edited = [(258.5, 163.3), (1920.0, 0.0), (1680.5, 935.8), (261.2, 919.5)];
    assert_eq!(changes(&edited, &target), [true, false, true, true]);
    for i in 0..4 {
        for axis in 0..2 {
            let mut edited = initial;
            if axis == 0 { edited[i].0 += 0.125; } else { edited[i].1 += 0.125; }
            let expected = std::array::from_fn(|j| j == i);
            assert_eq!(changes(&edited, &target), expected);
        }
    }
    // Explicit Fit repairs nonfinite input, rather than considering NaN equal.
    let mut invalid = initial;
    invalid[0].0 = f64::NAN;
    assert_eq!(changes(&invalid, &target), [true, false, false, false]);
}

#[test]
fn fitted_fractional_and_translated_quads_roundtrip_at_all_scales() {
    for (w, h) in [(1920, 1080), (1919, 1079), (1, 1), (1, 1080), (1920, 1)] {
        for scale in [1.0, 0.5, 0.25] {
            for offset in [(0.0, 0.0), (12.25, -8.5)] {
                let mut corners = quad(points(w, h).unwrap(), scale);
                for i in 0..4 { corners[2*i] += offset.0; corners[2*i+1] += offset.1; }
                let g = Geometry::new(&corners).unwrap();
                for (i, (u, v)) in [(0., 0.), (1., 0.), (1., 1.), (0., 1.)].into_iter().enumerate() {
                    let (x, y) = g.map(false, u, v).unwrap();
                    assert!((x - corners[2*i]).abs() < 1e-8 && (y - corners[2*i+1]).abs() < 1e-8);
                }
                for x in 0..=20 { for y in 0..=20 {
                    let (u, v) = (x as f64 / 20.0, y as f64 / 20.0);
                    let q = g.map(false, u, v).unwrap();
                    let r = g.map(true, q.0, q.1).unwrap();
                    assert!((r.0-u).abs() < 1e-8 && (r.1-v).abs() < 1e-8);
                }}
            }
        }
    }
}

fn pixel_matrix<T: Copy + Default + PartialEq + std::fmt::Debug>(depth: i32, sample: fn(usize) -> T) {
    let columns = [0.0_f32, 0.14, 0.5, 0.68, 0.86, 1.0];
    let rows = [0.0_f32, 0.27, 0.39, 0.55, 0.79, 1.0];
    for (width, height) in [(32, 18), (31, 17), (1, 1), (1, 17), (19, 1)] {
        for scale in [1.0_f64, 0.5, 0.25] {
            let w = (f64::from(width) * scale).round().max(1.0) as i32;
            let h = (f64::from(height) * scale).round().max(1.0) as i32;
            // Native input is larger than the output, has a negative origin and
            // nonpixel row padding. Fit still comes from SOURCE dimensions.
            let stride = (w as usize + 6) * 4 + 8;
            let mut input: Vec<T> = (0..stride * (h as usize + 4)).map(sample).collect();
            let src = Image {pixels: input.as_mut_ptr().cast(),
                row_bytes: (stride * std::mem::size_of::<T>()) as isize, width: w+6, height: h+4};
            let out_stride = w as usize * 4 + 4;
            let make_image = |data: &mut Vec<T>| Image {pixels: data.as_mut_ptr().cast(),
                row_bytes: (out_stride * std::mem::size_of::<T>()) as isize, width: w, height: h};
            for quality in 0..2 { for edge in 0..3 {
                let mut before = vec![T::default(); out_stride * h as usize];
                let mut after = before.clone();
                let target = quad(points(width, height).unwrap(), scale);
                // Independently express the initial 0/100-percent boundary quad.
                let initial = [0., 0., f64::from(width)*scale, 0.,
                    f64::from(width)*scale, f64::from(height)*scale, 0., f64::from(height)*scale];
                let mut frame = Frame {corners: initial, columns: columns.as_ptr(), rows: rows.as_ptr(),
                    column_count: 6, row_count: 6, surface_units_x: 1., surface_units_y: 1.,
                    canvas_width: w, canvas_height: h, source_x: -3, source_y: -2,
                    output_x: 0, output_y: 0, easing: 0.35, easing_distance: 0.25,
                    abort_fn: None, abort_refcon: std::ptr::null_mut()};
                let mut report = Report::default();
                // SAFETY: aligned, distinct live Vec buffers; row strides and
                // capacities cover every pixel and padding; FFI is synchronous.
                assert_eq!(unsafe {eg_render_plane_region(&src, &make_image(&mut before), depth,
                    &frame, &mut report, quality, edge)}, 0);
                assert_eq!(report.invalid_plane, 0);
                frame.corners = target;
                assert_eq!(unsafe {eg_render_plane_region(&src, &make_image(&mut after), depth,
                    &frame, &mut report, quality, edge)}, 0);
                assert_eq!(report.invalid_plane, 0);
                assert_eq!(after, before);
                if width == 32 && height == 18 && scale == 1.0 {
                    // Baseline width-1/height-1 was not just a label difference.
                    frame.corners = [0., 0., 31., 0., 31., 17., 0., 17.];
                    assert_eq!(unsafe {eg_render_plane_region(&src, &make_image(&mut after), depth,
                        &frame, &mut report, quality, edge)}, 0);
                    assert_ne!(after, before);
                }
            }}
        }
    }
}

#[test]
fn fit_default_deformed_pixels_8bpc_match_exactly() {
    pixel_matrix(8, |i| ((i * 173) % 256) as u8);
}
#[test]
fn fit_default_deformed_pixels_16bpc_match_exactly() {
    pixel_matrix(16, |i| ((i * 251) % 32769) as u16);
}
#[test]
fn fit_default_deformed_pixels_float_extended_range_match_exactly() {
    pixel_matrix(32, |i| ((i * 137) % 421) as f32 / 73.0 - 2.0);
}
