use after_effects as ae;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use std::fmt;
use std::ffi::c_void;

mod ui;
mod ui_projection;
mod plane;
#[cfg(fstr_lifecycle_probe)]
mod lifecycle_probe;
#[cfg(test)]
mod plane_contract_tests;
#[cfg(any(test, fstr_binding_probe))]
mod binding_transaction;
#[cfg(fstr_binding_probe)]
mod binding_probe;
#[cfg(all(fstr_binding_probe, not(fstr_lifecycle_probe)))]
compile_error!("Binding research requires lifecycle research context");
#[cfg(all(fstr_auto_binding,not(fstr_binding_probe)))]
compile_error!("Automatic binding requires binding parameters");
mod build_identity {
    include!(concat!(env!("OUT_DIR"), "/build_identity.rs"));
}

const GRID_REFCON: u64 = 0x4547_4658_4752_4944; // "EGFXGRID"
const MAX_GUIDES: usize = 50;
const MAX_LEGACY_MESH_POINTS: usize = (128 + 1) * (128 + 1);

const GRID_WIRE_VERSION: u16 = 3;
const GRID_WIRE_MARKER: u16 = 0x8000;

// IMPORTANT: parameter IDs in the Rust AE host are derived from these Debug
// names. Existing variants must never be renamed once a project can be saved.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub(crate) enum Params {
    Columns,
    Rows,
    GridState,
    TensionRadius,
    Falloff,
    ElasticityStrength,
    MinSpacing,
    StretchEasing,
    EasingDistance,
    WaveEnabled,
    WaveAmplitude,
    WaveFrequency,
    WavePhase,
    WaveSpeed,
    WaveAxis,
    EdgeMode,
    Quality,
    PlaneMode,
    PlaneTopLeft,
    PlaneTopRight,
    PlaneBottomRight,
    PlaneBottomLeft,
    ResetPlane,
    #[cfg(fstr_binding_probe)] ResearchPlaneTL,
    #[cfg(fstr_binding_probe)] ResearchPlaneTR,
    #[cfg(fstr_binding_probe)] ResearchPlaneBR,
    #[cfg(fstr_binding_probe)] ResearchPlaneBL,
    #[cfg(fstr_binding_probe)] ResearchPlaneKind,
}

#[derive(Default)]
struct Plugin {
    #[cfg(fstr_lifecycle_probe)]
    lifecycle_probe: lifecycle_probe::Probe,
}

ae::define_effect!(Plugin, (), Params);

#[derive(Clone, Debug, Serialize, PartialEq, PartialOrd)]
pub(crate) struct GridArb {
    #[serde(serialize_with = "serialize_grid_columns")]
    pub(crate) columns: u16,
    pub(crate) rows: u16,
    // Full normalized axes including implicit 0/1 boundaries.
    // columns/rows are counts of INTERNAL guide lines, matching GridWarp.
    pub(crate) column_lines: Vec<f32>,
    pub(crate) row_lines: Vec<f32>,
    // Retained only for our existing project wire format/FFI. Original GridWarp
    // has no pin concept; new states always pin boundaries only.
    pub(crate) column_pins: Vec<u8>,
    pub(crate) row_pins: Vec<u8>,
}

fn serialize_grid_columns<S>(columns: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let guides = (*columns).clamp(1, MAX_GUIDES as u16);
    let wire = GRID_WIRE_MARKER | ((GRID_WIRE_VERSION & 0x7f) << 8) | (guides & 0xff);
    serializer.serialize_u16(wire)
}

fn deserialize_grid_columns<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let wire = u16::deserialize(deserializer)?;
    if wire & GRID_WIRE_MARKER == 0 {
        if (1..=MAX_GUIDES as u16).contains(&wire) {
            return Ok(wire);
        }
        return Err(serde::de::Error::custom("invalid legacy ElasticGrid guide count"));
    }
    let version = (wire >> 8) & 0x7f;
    let guides = wire & 0xff;
    if version != 1 && version != 2 && version != GRID_WIRE_VERSION {
        return Err(serde::de::Error::custom("unsupported ElasticGrid grid-state version"));
    }
    if !(1..=MAX_GUIDES as u16).contains(&guides) {
        return Err(serde::de::Error::custom("invalid ElasticGrid guide count"));
    }
    Ok(guides)
}

struct BoundedF32VecVisitor;
impl<'de> Visitor<'de> for BoundedF32VecVisitor {
    type Value = Vec<f32>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an ElasticGrid axis vector within the 50-guide topology limit")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de> {
        if seq.size_hint().is_some_and(|n| n > MAX_LEGACY_MESH_POINTS) {
            return Err(serde::de::Error::custom("ElasticGrid vector exceeds topology limit"));
        }
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(MAX_LEGACY_MESH_POINTS));
        while let Some(value) = seq.next_element::<f32>()? {
            if out.len() >= MAX_LEGACY_MESH_POINTS {
                return Err(serde::de::Error::custom("ElasticGrid vector exceeds topology limit"));
            }
            out.push(value);
        }
        Ok(out)
    }
}

struct BoundedU8VecVisitor;
impl<'de> Visitor<'de> for BoundedU8VecVisitor {
    type Value = Vec<u8>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an ElasticGrid pin vector within the 50-guide topology limit")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de> {
        if seq.size_hint().is_some_and(|n| n > MAX_LEGACY_MESH_POINTS) {
            return Err(serde::de::Error::custom("ElasticGrid pin vector exceeds topology limit"));
        }
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(MAX_LEGACY_MESH_POINTS));
        while let Some(value) = seq.next_element::<u8>()? {
            if out.len() >= MAX_LEGACY_MESH_POINTS {
                return Err(serde::de::Error::custom("ElasticGrid pin vector exceeds topology limit"));
            }
            out.push(value);
        }
        Ok(out)
    }
}

fn deserialize_bounded_f32_vec<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
where D: Deserializer<'de> {
    deserializer.deserialize_seq(BoundedF32VecVisitor)
}

fn deserialize_bounded_u8_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de> {
    deserializer.deserialize_seq(BoundedU8VecVisitor)
}

#[derive(Deserialize)]
struct GridArbWire {
    #[serde(deserialize_with = "deserialize_grid_columns")]
    columns: u16,
    rows: u16,
    #[serde(deserialize_with = "deserialize_bounded_f32_vec")]
    column_lines: Vec<f32>,
    #[serde(deserialize_with = "deserialize_bounded_f32_vec")]
    row_lines: Vec<f32>,
    #[serde(deserialize_with = "deserialize_bounded_u8_vec")]
    column_pins: Vec<u8>,
    #[serde(deserialize_with = "deserialize_bounded_u8_vec")]
    row_pins: Vec<u8>,
}

impl<'de> Deserialize<'de> for GridArb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let wire = GridArbWire::deserialize(deserializer)?;
        // Preserve/read the legacy wire fields so old project blobs keep the
        // same six-field layout; original GridWarp itself has no internal pins.
        let _legacy_pin_fields = (&wire.column_pins, &wire.row_pins);
        let columns = wire.columns as usize;
        let rows = wire.rows as usize;

        // Compatibility:
        // - v0/v1 axis state stored N+1 values (N cells + boundaries);
        // - abandoned 2D-mesh v2 stored (columns+1)*(rows+1) X/Y points;
        // - parity v3 stores N internal guides + two boundaries = N+2 values.
        //
        // A 2D mesh cannot be represented losslessly by the original GridWarp
        // separable guide model. Migrate it safely to a fresh uniform grid
        // instead of returning an AE InternalStructDamaged error.
        let legacy_mesh_count = (columns + 1).saturating_mul(rows + 1);
        let looks_like_legacy_mesh =
            wire.column_lines.len() == legacy_mesh_count &&
            wire.row_lines.len() == legacy_mesh_count &&
            legacy_mesh_count > columns + 2 &&
            legacy_mesh_count > rows + 2;

        let (column_lines, row_lines) = if looks_like_legacy_mesh {
            (
                GridArb::axis_uniform(columns).0,
                GridArb::axis_uniform(rows).0,
            )
        } else {
            let x = if wire.column_lines.len() == columns + 1 {
                GridArb::resample_axis_raw(&wire.column_lines, columns)
            } else {
                wire.column_lines
            };
            let y = if wire.row_lines.len() == rows + 1 {
                GridArb::resample_axis_raw(&wire.row_lines, rows)
            } else {
                wire.row_lines
            };
            (x, y)
        };

        let mut out = Self {
            columns: wire.columns,
            rows: wire.rows,
            column_lines,
            row_lines,
            column_pins: Vec::new(),
            row_pins: Vec::new(),
        };
        out.canonicalize_pins();
        if out.is_valid() {
            Ok(out)
        } else {
            Err(serde::de::Error::custom("invalid ElasticGrid grid state"))
        }
    }
}

impl Default for GridArb {
    fn default() -> Self {
        Self::uniform(4, 4)
    }
}

impl GridArb {
    fn axis_uniform(guides: usize) -> (Vec<f32>, Vec<u8>) {
        let guides = guides.clamp(1, MAX_GUIDES);
        let segments = guides + 1;
        let mut lines = Vec::with_capacity(guides + 2);
        for i in 0..=segments {
            lines.push(i as f32 / segments as f32);
        }
        let mut pins = vec![0u8; guides + 2];
        pins[0] = 1;
        *pins.last_mut().unwrap() = 1;
        (lines, pins)
    }

    fn uniform(columns: usize, rows: usize) -> Self {
        let columns = columns.clamp(1, MAX_GUIDES);
        let rows = rows.clamp(1, MAX_GUIDES);
        let (column_lines, column_pins) = Self::axis_uniform(columns);
        let (row_lines, row_pins) = Self::axis_uniform(rows);
        Self {
            columns: columns as u16,
            rows: rows as u16,
            column_lines,
            row_lines,
            column_pins,
            row_pins,
        }
    }

    fn canonicalize_pins(&mut self) {
        self.column_pins = vec![0; self.column_lines.len()];
        self.row_pins = vec![0; self.row_lines.len()];
        if !self.column_pins.is_empty() {
            self.column_pins[0] = 1;
            *self.column_pins.last_mut().unwrap() = 1;
        }
        if !self.row_pins.is_empty() {
            self.row_pins[0] = 1;
            *self.row_pins.last_mut().unwrap() = 1;
        }
    }

    fn valid_axis(lines: &[f32], pins: &[u8], guides: usize) -> bool {
        if guides == 0 || guides > MAX_GUIDES ||
            lines.len() != guides + 2 || pins.len() != guides + 2 {
            return false;
        }
        if !lines.iter().all(|v| v.is_finite()) ||
            lines.first().copied() != Some(0.0) ||
            lines.last().copied() != Some(1.0) {
            return false;
        }
        if pins.first().copied() != Some(1) || pins.last().copied() != Some(1) ||
            pins[1..pins.len()-1].iter().any(|p| *p != 0) {
            return false;
        }
        lines.windows(2).all(|w| w[1] > w[0])
    }

    fn is_valid(&self) -> bool {
        Self::valid_axis(&self.column_lines, &self.column_pins, self.columns as usize)
            && Self::valid_axis(&self.row_lines, &self.row_pins, self.rows as usize)
    }

    fn resample_axis_raw(lines: &[f32], new_guides: usize) -> Vec<f32> {
        let new_guides = new_guides.clamp(1, MAX_GUIDES);
        let new_segments = new_guides + 1;
        if lines.len() < 2 || !lines.iter().all(|v| v.is_finite()) {
            return Self::axis_uniform(new_guides).0;
        }
        let old_segments = lines.len() - 1;
        let mut out = Vec::with_capacity(new_guides + 2);
        for i in 0..=new_segments {
            if i == 0 {
                out.push(0.0);
                continue;
            }
            if i == new_segments {
                out.push(1.0);
                continue;
            }
            let old_pos = i as f32 * old_segments as f32 / new_segments as f32;
            let left = old_pos.floor() as usize;
            let right = (left + 1).min(old_segments);
            let t = old_pos - left as f32;
            out.push(lines[left] + (lines[right] - lines[left]) * t);
        }
        out
    }

    fn resized(&self, columns: usize, rows: usize) -> Self {
        let columns = columns.clamp(1, MAX_GUIDES);
        let rows = rows.clamp(1, MAX_GUIDES);
        if !self.is_valid() {
            return Self::uniform(columns, rows);
        }
        if self.columns as usize == columns && self.rows as usize == rows {
            return self.clone();
        }

        // Original GridWarp does not preserve/resample deformation when
        // Num Columns/Rows changes. Only the changed axis is reset uniformly.
        let column_lines = if self.columns as usize == columns {
            self.column_lines.clone()
        } else {
            Self::axis_uniform(columns).0
        };
        let row_lines = if self.rows as usize == rows {
            self.row_lines.clone()
        } else {
            Self::axis_uniform(rows).0
        };

        let mut out = Self {
            columns: columns as u16,
            rows: rows as u16,
            column_lines,
            row_lines,
            column_pins: Vec::new(),
            row_pins: Vec::new(),
        };
        out.canonicalize_pins();
        out
    }
}

impl ae::ArbitraryData<GridArb> for GridArb {
    fn interpolate(&self, other: &GridArb, value: f64) -> GridArb {
        let t = value.clamp(0.0, 1.0) as f32;
        if !self.is_valid() {
            return other.clone();
        }
        if !other.is_valid() {
            return self.clone();
        }
        if self.columns != other.columns || self.rows != other.rows {
            return if t < 0.5 { self.clone() } else { other.clone() };
        }

        let mut out = self.clone();
        for (dst, (a, b)) in out.column_lines.iter_mut().zip(self.column_lines.iter().zip(other.column_lines.iter())) {
            *dst = *a + (*b - *a) * t;
        }
        for (dst, (a, b)) in out.row_lines.iter_mut().zip(self.row_lines.iter().zip(other.row_lines.iter())) {
            *dst = *a + (*b - *a) * t;
        }
        out.column_lines[0] = 0.0;
        *out.column_lines.last_mut().unwrap() = 1.0;
        out.row_lines[0] = 0.0;
        *out.row_lines.last_mut().unwrap() = 1.0;
        out.canonicalize_pins();
        out
    }
}

#[repr(C)]
struct EgRenderParams {
    columns: i32,
    rows: i32,
    column_lines: *const f32,
    column_line_count: i32,
    column_pins: *const u8,
    row_lines: *const f32,
    row_line_count: i32,
    row_pins: *const u8,
    tension_radius: f32,
    falloff: i32,
    elasticity_strength: f32,
    min_spacing: f32,
    stretch_easing: f32,
    easing_distance: f32,
    wave_enabled: i32,
    wave_amplitude: f32,
    wave_frequency: f32,
    wave_phase: f32,
    wave_speed: f32,
    wave_axis: i32,
    edge_mode: i32,
    quality: i32,
    time_seconds: f32,
    threads: u32,
    canvas_width: i32,
    canvas_height: i32,
    input_origin_x: i32,
    input_origin_y: i32,
    output_origin_x: i32,
    output_origin_y: i32,
    abort_fn: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    abort_refcon: *mut c_void,
}

#[derive(Clone, Debug)]
struct SmartRenderSnapshot {
    grid: GridArb,
    plane: plane::State,
    tension_radius: f32,
    falloff: i32,
    elasticity_strength: f32,
    min_spacing: f32,
    stretch_easing: f32,
    easing_distance: f32,
    wave_amplitude: f32,
    wave_frequency: f32,
    wave_phase: f32,
    wave_speed: f32,
    wave_axis: i32,
    edge_mode: i32,
    quality: i32,
    time_seconds: f32,
    canvas_width: i32,
    canvas_height: i32,
}

impl SmartRenderSnapshot {
    fn render_params(&self, abort_refcon: *mut c_void) -> EgRenderParams {
        EgRenderParams {
            columns: self.grid.columns as i32,
            rows: self.grid.rows as i32,
            column_lines: self.grid.column_lines.as_ptr(),
            column_line_count: self.grid.column_lines.len() as i32,
            column_pins: self.grid.column_pins.as_ptr(),
            row_lines: self.grid.row_lines.as_ptr(),
            row_line_count: self.grid.row_lines.len() as i32,
            row_pins: self.grid.row_pins.as_ptr(),
            tension_radius: self.tension_radius,
            falloff: self.falloff,
            elasticity_strength: self.elasticity_strength,
            min_spacing: self.min_spacing,
            stretch_easing: self.stretch_easing,
            easing_distance: self.easing_distance,
            // Original GridWarp has no Wave Enable switch; amplitude=0 disables it.
            wave_enabled: 1,
            wave_amplitude: self.wave_amplitude,
            wave_frequency: self.wave_frequency,
            wave_phase: self.wave_phase,
            wave_speed: self.wave_speed,
            wave_axis: self.wave_axis,
            edge_mode: self.edge_mode,
            quality: self.quality,
            time_seconds: self.time_seconds,
            threads: 0,
            canvas_width: self.canvas_width,
            canvas_height: self.canvas_height,
            input_origin_x: 0,
            input_origin_y: 0,
            output_origin_x: 0,
            output_origin_y: 0,
            abort_fn: Some(ae_abort_trampoline),
            abort_refcon,
        }
    }
}

fn checked_slider(params: &ae::Parameters<Params>, param: Params) -> Result<i32, ae::Error> {
    let checked = params.checkout(param)?;
    Ok(checked.as_slider()?.value())
}

fn checked_float(params: &ae::Parameters<Params>, param: Params) -> Result<f64, ae::Error> {
    let checked = params.checkout(param)?;
    Ok(checked.as_float_slider()?.value())
}

fn checked_popup(params: &ae::Parameters<Params>, param: Params) -> Result<i32, ae::Error> {
    let checked = params.checkout(param)?;
    Ok(checked.as_popup()?.value())
}

fn smart_render_snapshot(
    params: &ae::Parameters<Params>,
    in_data: ae::InData,
) -> Result<SmartRenderSnapshot, ae::Error> {
    // PF_Cmd_SMART_PRE_RENDER / SMART_RENDER do not receive a valid normal
    // parameter array. Every render dependency must be checked out here and
    // copied into owned pre_render_data for the matching SmartRender call.
    let columns = checked_slider(params, Params::Columns)?.clamp(1, MAX_GUIDES as i32) as usize;
    let rows = checked_slider(params, Params::Rows)?.clamp(1, MAX_GUIDES as i32) as usize;
    let grid = {
        let checked = params.checkout(Params::GridState)?;
        let value = checked.as_arbitrary()?.value::<GridArb>()?;
        (*value).resized(columns, rows)
    };
    let time_seconds = if in_data.time_scale() != 0 {
        in_data.current_time() as f32 / in_data.time_scale() as f32
    } else {
        0.0
    };

    let (canvas_width, canvas_height) = rendered_canvas(in_data);

    Ok(SmartRenderSnapshot {
        grid,
        plane: plane::State::read(params, &in_data, true, true)?,
        tension_radius: checked_float(params, Params::TensionRadius)? as f32,
        falloff: checked_popup(params, Params::Falloff)?,
        elasticity_strength: checked_float(params, Params::ElasticityStrength)? as f32 / 100.0,
        min_spacing: checked_float(params, Params::MinSpacing)? as f32 / 100.0,
        stretch_easing: checked_float(params, Params::StretchEasing)? as f32 / 100.0,
        easing_distance: checked_float(params, Params::EasingDistance)? as f32 / 100.0,
        wave_amplitude: checked_float(params, Params::WaveAmplitude)? as f32 / 100.0,
        wave_frequency: checked_float(params, Params::WaveFrequency)? as f32,
        wave_phase: checked_float(params, Params::WavePhase)? as f32,
        wave_speed: checked_float(params, Params::WaveSpeed)? as f32,
        wave_axis: checked_popup(params, Params::WaveAxis)?,
        edge_mode: checked_popup(params, Params::EdgeMode)?,
        quality: checked_popup(params, Params::Quality)?,
        time_seconds,
        canvas_width,
        canvas_height,
    })
}

#[repr(C)]
pub(crate) struct EgElasticParams {
    pub(crate) tension_radius: f32,
    pub(crate) falloff: i32,
    pub(crate) elasticity_strength: f32,
    pub(crate) min_spacing: f32,
}

unsafe extern "C" {
    fn eg_render_frame(
        input_data: *const c_void,
        input_row_bytes: isize,
        input_width: i32,
        input_height: i32,
        output_data: *mut c_void,
        output_row_bytes: isize,
        output_width: i32,
        output_height: i32,
        bit_depth: i32,
        params: *const EgRenderParams,
    ) -> i32;

    fn eg_render_frame_sparse(
        input_data: *const c_void,
        input_row_bytes: isize,
        input_width: i32,
        input_height: i32,
        output_data: *mut c_void,
        output_row_bytes: isize,
        output_width: i32,
        output_height: i32,
        bit_depth: i32,
        params: *const EgRenderParams,
    ) -> i32;

    pub(crate) fn eg_drag_axis(
        lines: *mut f32,
        pins: *mut u8,
        line_count: i32,
        line_index: i32,
        target: f32,
        params: *const EgElasticParams,
    ) -> i32;

    #[cfg(target_os = "macos")]
    fn eg_metal_create(device: *mut c_void, command_queue: *mut c_void) -> *mut c_void;
    #[cfg(target_os = "macos")]
    fn eg_metal_destroy(state: *mut c_void);
    #[cfg(target_os = "macos")]
    fn eg_metal_render(
        state: *mut c_void,
        input_buffer: *mut c_void,
        input_row_bytes: isize,
        input_width: i32,
        input_height: i32,
        output_buffer: *mut c_void,
        output_row_bytes: isize,
        output_width: i32,
        output_height: i32,
        params: *const EgRenderParams,
    ) -> i32;
}


unsafe extern "C" fn ae_abort_trampoline(refcon: *mut c_void) -> i32 {
    if refcon.is_null() {
        return 1;
    }
    let raw = unsafe { &*(refcon as *const ae::sys::PF_InData) };
    let Some(abort) = raw.inter.abort else {
        return 1;
    };
    if unsafe { abort(raw.effect_ref) } == 0 { 0 } else { 1 }
}

#[cfg(target_os = "macos")]
struct MetalGpuData {
    state: *mut c_void,
}

#[cfg(target_os = "macos")]
// SAFETY: the pointed-to native Metal state owns only immutable Metal device/
// pipeline objects plus an atomic/CAS plan-buffer pool. The native teardown
// protocol marks shutdown and waits for in-flight renders before destruction.
// No Rust reference aliases the pointee; the pointer is an opaque ownership token.
unsafe impl Send for MetalGpuData {}
#[cfg(target_os = "macos")]
// SAFETY: see Send above. Concurrent AE MFR calls are synchronized entirely by
// the native atomic lifetime/pool protocol and Metal's thread-safe objects.
unsafe impl Sync for MetalGpuData {}

#[cfg(target_os = "macos")]
impl Drop for MetalGpuData {
    fn drop(&mut self) {
        if !self.state.is_null() {
            unsafe { eg_metal_destroy(self.state) };
            self.state = std::ptr::null_mut();
        }
    }
}

fn setup_float(
    f: &mut ae::FloatSliderDef,
    valid_range: (f32, f32),
    slider_range: (f32, f32),
    default: f64,
    precision: i16,
    percent: bool,
) {
    f.set_valid_min(valid_range.0);
    f.set_valid_max(valid_range.1);
    f.set_slider_min(slider_range.0);
    f.set_slider_max(slider_range.1);
    f.set_default(default);
    f.set_precision(precision);
    if percent {
        f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
    }
}

fn wave_is_time_varying(enabled: bool, amplitude: f64, speed: f64) -> bool {
    enabled && amplitude.abs() > 1.0e-12 && speed.abs() > 1.0e-12
}

fn topology(params: &ae::Parameters<Params>) -> Result<(usize, usize), ae::Error> {
    Ok((
        params.get(Params::Columns)?.as_slider()?.value().clamp(1, MAX_GUIDES as i32) as usize,
        params.get(Params::Rows)?.as_slider()?.value().clamp(1, MAX_GUIDES as i32) as usize,
    ))
}

pub(crate) fn grid_snapshot(params: &ae::Parameters<Params>) -> Result<GridArb, ae::Error> {
    let (columns, rows) = topology(params)?;
    let grid = params.get(Params::GridState)?.as_arbitrary()?.value::<GridArb>()?;
    Ok((*grid).resized(columns, rows))
}

fn sync_grid_topology(params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
    let (columns, rows) = topology(params)?;
    let current = {
        let grid = params.get(Params::GridState)?.as_arbitrary()?.value::<GridArb>()?;
        (*grid).clone()
    };
    if current.columns as usize == columns && current.rows as usize == rows && current.is_valid() {
        return Ok(());
    }
    let next = current.resized(columns, rows);
    params.get_mut(Params::GridState)?.as_arbitrary_mut()?.set_value(next)?;
    Ok(())
}

pub(crate) fn elastic_params(params: &ae::Parameters<Params>) -> Result<EgElasticParams, ae::Error> {
    Ok(EgElasticParams {
        tension_radius: params.get(Params::TensionRadius)?.as_float_slider()?.value() as f32,
        falloff: params.get(Params::Falloff)?.as_popup()?.value(),
        elasticity_strength: params.get(Params::ElasticityStrength)?.as_float_slider()?.value() as f32 / 100.0,
        min_spacing: params.get(Params::MinSpacing)?.as_float_slider()?.value() as f32 / 100.0,
    })
}

fn evaluated_params(
    params: &mut ae::Parameters<Params>,
    in_data: ae::InData,
    grid: &GridArb,
) -> Result<EgRenderParams, ae::Error> {
    let time_seconds = if in_data.time_scale() != 0 {
        in_data.current_time() as f32 / in_data.time_scale() as f32
    } else {
        0.0
    };

    Ok(EgRenderParams {
        columns: grid.columns as i32,
        rows: grid.rows as i32,
        column_lines: grid.column_lines.as_ptr(),
        column_line_count: grid.column_lines.len() as i32,
        column_pins: grid.column_pins.as_ptr(),
        row_lines: grid.row_lines.as_ptr(),
        row_line_count: grid.row_lines.len() as i32,
        row_pins: grid.row_pins.as_ptr(),
        tension_radius: params.get(Params::TensionRadius)?.as_float_slider()?.value() as f32,
        falloff: params.get(Params::Falloff)?.as_popup()?.value(),
        elasticity_strength: params.get(Params::ElasticityStrength)?.as_float_slider()?.value() as f32 / 100.0,
        min_spacing: params.get(Params::MinSpacing)?.as_float_slider()?.value() as f32 / 100.0,
        stretch_easing: params.get(Params::StretchEasing)?.as_float_slider()?.value() as f32 / 100.0,
        easing_distance: params.get(Params::EasingDistance)?.as_float_slider()?.value() as f32 / 100.0,
        // Original GridWarp has no Wave Enable switch; amplitude=0 disables it.
        wave_enabled: 1,
        wave_amplitude: params.get(Params::WaveAmplitude)?.as_float_slider()?.value() as f32 / 100.0,
        wave_frequency: params.get(Params::WaveFrequency)?.as_float_slider()?.value() as f32,
        wave_phase: params.get(Params::WavePhase)?.as_float_slider()?.value() as f32,
        wave_speed: params.get(Params::WaveSpeed)?.as_float_slider()?.value() as f32,
        wave_axis: params.get(Params::WaveAxis)?.as_popup()?.value(),
        edge_mode: params.get(Params::EdgeMode)?.as_popup()?.value(),
        quality: params.get(Params::Quality)?.as_popup()?.value(),
        time_seconds,
        threads: 0,
        canvas_width: 0,
        canvas_height: 0,
        input_origin_x: 0,
        input_origin_y: 0,
        output_origin_x: 0,
        output_origin_y: 0,
        abort_fn: Some(ae_abort_trampoline),
        abort_refcon: in_data.as_ptr() as *mut c_void,
    })
}

fn rendered_canvas(in_data: ae::InData) -> (i32, i32) {
    let sx = f32::from(in_data.downsample_x());
    let sy = f32::from(in_data.downsample_y());
    let w = ((in_data.width().max(1) as f32) * sx).round().max(1.0) as i32;
    let h = ((in_data.height().max(1) as f32) * sy).round().max(1.0) as i32;
    (w, h)
}

fn clamp_request_to_canvas(mut rect: ae::Rect, canvas_width: i32, canvas_height: i32) -> ae::Rect {
    let cw = canvas_width.max(0);
    let ch = canvas_height.max(0);
    rect.left = rect.left.clamp(0, cw);
    rect.right = rect.right.clamp(0, cw);
    rect.top = rect.top.clamp(0, ch);
    rect.bottom = rect.bottom.clamp(0, ch);
    if rect.right < rect.left { rect.right = rect.left; }
    if rect.bottom < rect.top { rect.bottom = rect.top; }
    rect
}

fn apply_spatial_context(
    in_data: ae::InData,
    input: &ae::Layer,
    output: &ae::Layer,
    p: &mut EgRenderParams,
) {
    let (cw, ch) = rendered_canvas(in_data);
    let input_origin = input.origin();
    let output_origin = output.origin();
    p.canvas_width = cw;
    p.canvas_height = ch;
    p.input_origin_x = input_origin.h;
    p.input_origin_y = input_origin.v;
    p.output_origin_x = output_origin.h;
    p.output_origin_y = output_origin.v;
}

fn render(
    in_layer: &ae::Layer,
    out_layer: &mut ae::Layer,
    p: &EgRenderParams,
) -> Result<(), ae::Error> {
    if in_layer.bit_depth() != out_layer.bit_depth() {
        return Err(ae::Error::BadCallbackParameter);
    }

    let rc = unsafe {
        eg_render_frame(
            in_layer.data_ptr().cast(),
            in_layer.row_bytes(),
            in_layer.width() as i32,
            in_layer.height() as i32,
            out_layer.data_ptr_mut().cast(),
            out_layer.row_bytes(),
            out_layer.width() as i32,
            out_layer.height() as i32,
            out_layer.bit_depth() as i32,
            p,
        )
    };

    match rc {
        0 => Ok(()),
        5 => Err(ae::Error::InterruptCancel),
        1 | 2 | 4 => Err(ae::Error::BadCallbackParameter),
        _ => Err(ae::Error::InternalStructDamaged),
    }
}

fn render_sparse(
    input: Option<&ae::Layer>,
    out_layer: &mut ae::Layer,
    p: &EgRenderParams,
) -> Result<(), ae::Error> {
    if p.canvas_width <= 0 || p.canvas_height <= 0 {
        return Err(ae::Error::BadCallbackParameter);
    }
    if let Some(layer) = input {
        if layer.bit_depth() != out_layer.bit_depth() {
            return Err(ae::Error::BadCallbackParameter);
        }
    }

    let (input_data, input_row_bytes, input_width, input_height) = if let Some(layer) = input {
        (unsafe { layer.data_ptr().cast() }, layer.row_bytes(), layer.width() as i32, layer.height() as i32)
    } else {
        (std::ptr::null(), 0, 0, 0)
    };

    let rc = unsafe {
        eg_render_frame_sparse(
            input_data,
            input_row_bytes,
            input_width,
            input_height,
            out_layer.data_ptr_mut().cast(),
            out_layer.row_bytes(),
            out_layer.width() as i32,
            out_layer.height() as i32,
            out_layer.bit_depth() as i32,
            p,
        )
    };

    match rc {
        0 => Ok(()),
        5 => Err(ae::Error::InterruptCancel),
        1 | 2 | 4 => Err(ae::Error::BadCallbackParameter),
        _ => Err(ae::Error::InternalStructDamaged),
    }
}


#[cfg(target_os = "macos")]
fn render_metal(
    in_data: ae::InData,
    extra: &ae::SmartRenderExtra,
    mut input: ae::Layer,
    mut output: ae::Layer,
    p: &EgRenderParams,
    gpu_data: &MetalGpuData,
) -> Result<(), ae::Error> {
    if extra.what_gpu() != ae::GpuFramework::Metal || extra.bit_depth() != 32 || gpu_data.state.is_null() {
        return Err(ae::Error::BadCallbackParameter);
    }
    if input.row_bytes() <= 0 || output.row_bytes() <= 0 || input.row_bytes() % 16 != 0 || output.row_bytes() % 16 != 0 {
        return Err(ae::Error::BadCallbackParameter);
    }

    // Metal command buffers cannot be cancelled once committed. Poll AE immediately
    // before dispatch and again after completion; GPU work is short and never
    // calls host callbacks from a Metal/GCD worker thread.
    in_data.interact().abort()?;
    let gpu = ae::pf::suites::GPUDevice::new()?;
    let input_buffer = gpu.gpu_world_data(in_data.effect_ref(), &mut input)?;
    let output_buffer = gpu.gpu_world_data(in_data.effect_ref(), &mut output)?;
    let rc = unsafe {
        eg_metal_render(
            gpu_data.state,
            input_buffer, input.row_bytes(), input.width() as i32, input.height() as i32,
            output_buffer, output.row_bytes(), output.width() as i32, output.height() as i32,
            p,
        )
    };

    if rc == 0 {
        in_data.interact().abort()?;
        return Ok(());
    }
    match rc {
        1 => Err(ae::Error::BadCallbackParameter),
        4 | 7 => Err(ae::Error::InternalStructDamaged),
        _ => Err(ae::Error::InternalStructDamaged),
    }
}

impl AdobePluginGlobal for Plugin {
    fn params_setup(
        &self,
        params: &mut ae::Parameters<Params>,
        in_data: ae::InData,
        _: ae::OutData,
    ) -> Result<(), ae::Error> {
        // UI order is independent of persistent IDs (derived from unchanged Params names).
        params.add_with_flags(Params::PlaneMode, "Deformation Plane", ae::PopupDef::setup(|f| {
            f.set_options(&["Layer Plane", "Four Corners"]);
            f.set_default(1); f.set_value(1);
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;
        for (id, name, point) in [
            (Params::PlaneTopLeft, "Plane Top Left", (0.0, 0.0)),
            (Params::PlaneTopRight, "Plane Top Right", (100.0, 0.0)),
            (Params::PlaneBottomRight, "Plane Bottom Right", (100.0, 100.0)),
            (Params::PlaneBottomLeft, "Plane Bottom Left", (0.0, 100.0)),
        ] {
            params.add_with_flags(id, name, ae::PointDef::setup(|f| {
                f.set_default(point); f.set_restrict_bounds(false);
            }), ae::ParamFlag::empty(), ae::ParamUIFlags::DISABLED)?;
        }
        params.add_with_flags(Params::ResetPlane, "Reset Plane", ae::ButtonDef::setup(|f| {
            f.set_label("Fit Layer");
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::DISABLED)?;
        params.add_with_flags(Params::Columns, "Columns", ae::SliderDef::setup(|f| {
            f.set_valid_min(1);
            f.set_valid_max(MAX_GUIDES as i32);
            f.set_slider_min(1);
            f.set_slider_max(32);
            f.set_default(4);
            f.set_value(f.default());
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;
        params.add_with_flags(Params::Rows, "Rows", ae::SliderDef::setup(|f| {
            f.set_valid_min(1);
            f.set_valid_max(MAX_GUIDES as i32);
            f.set_slider_min(1);
            f.set_slider_max(32);
            f.set_default(4);
            f.set_value(f.default());
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;

        let mut grid_state_def = ae::ArbitraryDef::new();
        grid_state_def.set_default(GridArb::default())?;
        grid_state_def.set_refcon(GRID_REFCON as *mut c_void);
        params.add_customized(Params::GridState, "Grid Positions", grid_state_def, |param| {
            // Keep the native animation row/stopwatch visible. Only the custom
            // diagnostic text is omitted by draw_effect_control.
            param.set_ui_flags(ae::ParamUIFlags::CONTROL);
            param.set_ui_width(300);
            param.set_ui_height(32);
            -1
        })?;

        params.add(Params::TensionRadius, "Tension Radius", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 20.0), (0.0, 8.0), 3.0, 1, false);
        }))?;
        params.add(Params::Falloff, "Falloff", ae::PopupDef::setup(|f| {
            // Keep saved numeric values and all four slots. Relabel to the
            // existing FFI behavior; ordinal 4 is the legacy Smoothstep alias.
            f.set_options(&["Smoothstep", "Gaussian", "Linear", "Smoothstep (Legacy)"]);
            f.set_default(2);
            f.set_value(f.default());
        }))?;
        params.add(Params::ElasticityStrength, "Elasticity Strength", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 200.0), (0.0, 200.0), 100.0, 1, true);
        }))?;
        params.add(Params::MinSpacing, "Min Line Spacing", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 25.0), (0.0, 5.0), 0.5, 2, true);
        }))?;
        params.add(Params::StretchEasing, "Stretch Easing", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 100.0), (0.0, 100.0), 0.0, 1, true);
        }))?;
        params.add(Params::EasingDistance, "Easing Distance", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (1.0, 50.0), (1.0, 50.0), 25.0, 1, true);
        }))?;

        // The parity implementation uses amplitude=0 to disable waves. Keep
        // this old checkbox slot/ID/type for projects, but do not expose a no-op.
        params.add_with_flags(Params::WaveEnabled, "Wave Animation", ae::CheckBoxDef::setup(|f| {
            f.set_label("Enable");
            f.set_default(false);
            f.set_value(f.default());
        }), ae::ParamFlag::empty(), ae::ParamUIFlags::INVISIBLE)?;
        params.add(Params::WaveAmplitude, "Wave Amplitude", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 25.0), (0.0, 10.0), 0.0, 2, true);
        }))?;
        params.add(Params::WaveFrequency, "Wave Frequency", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 10.0), (0.0, 10.0), 1.0, 2, false);
        }))?;
        params.add(Params::WavePhase, "Wave Phase", ae::FloatSliderDef::setup(|f| {
            // Stored values already use degrees; do not rescale old keyframes.
            setup_float(f, (-360.0, 360.0), (-180.0, 180.0), 0.0, 2, false);
        }))?;
        params.add(Params::WaveSpeed, "Wave Speed", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (-10.0, 10.0), (-2.0, 2.0), 0.0, 2, false);
        }))?;
        params.add(Params::WaveAxis, "Wave Axis", ae::PopupDef::setup(|f| {
            f.set_options(&["Both", "Columns Only", "Rows Only"]);
            f.set_default(1);
            f.set_value(f.default());
        }))?;

        params.add(Params::EdgeMode, "Edge Behavior", ae::PopupDef::setup(|f| {
            f.set_options(&["Clamp", "Wrap", "Mirror"]);
            f.set_default(1);
            f.set_value(f.default());
        }))?;
        params.add(Params::Quality, "Render Quality", ae::PopupDef::setup(|f| {
            f.set_options(&["Preview (Bilinear)", "Final (Bicubic)"]);
            f.set_default(2);
            f.set_value(f.default());
        }))?;

        #[cfg(fstr_binding_probe)]
        binding_probe::add_params(params)?;

        in_data.interact().register_ui(
            ae::CustomUIInfo::new().events(
                ae::CustomEventFlags::COMP |
                ae::CustomEventFlags::LAYER |
                ae::CustomEventFlags::EFFECT,
            ),
        )?;

        Ok(())
    }

    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        mut out_data: ae::OutData,
        params: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        #[cfg(fstr_lifecycle_probe)]
        self.lifecycle_probe.observe(&cmd, &in_data);
        match cmd {
            ae::Command::GlobalSetup => {
                out_data.set_out_flag(ae::OutFlags::SendUpdateParamsUi, true);
                // One noninteractive diagnostic per host setup, never per frame.
                eprintln!("{}", build_identity::ABOUT.replace('\r', " | "));
            }
            ae::Command::About => {
                out_data.set_return_msg(build_identity::ABOUT);
                #[cfg(all(fstr_lifecycle_probe,not(feature="native-plane")))]
                out_data.set_return_msg(&format!("LIFECYCLE RESEARCH ONLY\r{}\r{}",
                    build_identity::ABOUT, self.lifecycle_probe.report()));
            }
            ae::Command::UserChangedParam { param_index } => {
                if params.index(Params::PlaneMode)==Some(param_index) {
                    plane::update_ui(params)?;
                    out_data.set_out_flag(ae::OutFlags::ForceRerender,true);
                }
                if params.index(Params::ResetPlane) == Some(param_index) {
                    // Layer-space last pixel centers match the legacy raster
                    // grid domain. UI command: never consult frame-only origin.
                    let w=in_data.width().saturating_sub(1).max(0) as f32;
                    let h=in_data.height().saturating_sub(1).max(0) as f32;
                    for (id,point) in plane::CORNERS.into_iter().zip([(0.0,0.0),(w,0.0),(w,h),(0.0,h)]) {
                        let mut param=params.get_mut(id)?;
                        param.as_point_mut()?.set_value(point);
                        param.set_value_changed();
                    }
                    out_data.set_out_flag(ae::OutFlags::ForceRerender, true);
                }
                if params.index(Params::Columns) == Some(param_index)
                    || params.index(Params::Rows) == Some(param_index)
                {
                    sync_grid_topology(params)?;

                }
            }
            ae::Command::UpdateParamsUi => plane::update_ui(params)?,
            ae::Command::QueryDynamicFlags => {
                // Wave animation uses current_time even when no parameter has a
                // keyframe. Tell AE only when that time dependency is active,
                // so caching stays correct without disabling useful cache hits
                // for static warps. The PiPL/global setup advertises the flag
                // because AE requires dynamically-cleared flags to start set.
                let amplitude = params.checkout(Params::WaveAmplitude)?.as_float_slider()?.value();
                let speed = params.checkout(Params::WaveSpeed)?.as_float_slider()?.value();
                out_data.set_out_flag(
                    ae::OutFlags::NonParamVary,
                    wave_is_time_varying(true, amplitude, speed),
                );
            }
            ae::Command::ArbitraryCallback { mut extra } => {
                extra.dispatch::<GridArb, Params>(Params::GridState)?;
            }
            ae::Command::Event { mut extra } => {
                match extra.event() {
                    ae::Event::Click(_) => {
                        if extra.send_drag() {
                            ui::drag(&in_data, params, &mut extra)?;
                        } else {
                            ui::click(&in_data, params, &mut extra)?;
                        }
                    }
                    ae::Event::Drag(_) => ui::drag(&in_data, params, &mut extra)?,
                    ae::Event::Draw(_) => ui::draw(&in_data, params, &mut extra)?,
                    ae::Event::AdjustCursor(_) => ui::adjust_cursor(&in_data, params, &mut extra)?,
                    ae::Event::Deactivate | ae::Event::CloseContext | ae::Event::MouseExited => ui::release_cursor(),
                    _ => {}
                }
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let grid = grid_snapshot(params)?;
                let mut p = evaluated_params(params, in_data, &grid)?;
                apply_spatial_context(in_data, &in_layer, &out_layer, &mut p);
                let plane = plane::State::read(params, &in_data, false, true)?;
                if plane.corners.is_some() {
                    plane::render(Some(&in_layer), &mut out_layer, &p, &plane)?;
                } else { render(&in_layer, &mut out_layer, &p)?; }
            }
            ae::Command::SmartPreRender { mut extra } => {
                let snapshot = smart_render_snapshot(params, in_data)?;
                let output_request = extra.output_request();
                let (cw, ch) = (snapshot.canvas_width, snapshot.canvas_height);

                // Correctness-first SmartFX checkout: a guide warp can pull
                // pixels across cell boundaries, and bicubic filtering needs
                // neighboring taps. Always checkout the complete source canvas
                // until the ROI implementation is proven seam-free in AE.
                let mut request = output_request;
                request.rect.left = 0;
                request.rect.top = 0;
                request.rect.right = cw;
                request.rect.bottom = ch;
                let input = extra.callbacks().checkout_layer(
                    0, 0, &request,
                    in_data.current_time(), in_data.time_step(), in_data.time_scale(),
                )?;

                // Store the exact checked-out parameter state used for this
                // SmartFX request. SmartRender must not read the ordinary
                // params array because AE does not provide valid values there.
                extra.set_pre_render_data(snapshot);

                // A grid warp does not create pixels outside its logical layer
                // canvas. Downstream effects may request a subset; advertise only
                // that intersection and keep max bounds invariant across requests.
                let canvas_rect = ae::Rect { left: 0, top: 0, right: cw, bottom: ch };
                let requested: ae::Rect = output_request.rect.into();
                extra.set_result_rect(clamp_request_to_canvas(requested, cw, ch));
                extra.set_max_result_rect(canvas_rect);
                let _ = input; // checkout establishes dependency even if AE returns compact storage
                #[cfg(target_os = "macos")]
                extra.set_gpu_render_possible(false);
            }
            ae::Command::SmartRender { extra } => {
                let snapshot = extra
                    .pre_render_data::<SmartRenderSnapshot>()
                    .ok_or(ae::Error::InternalStructDamaged)?;
                let cb = extra.callbacks();
                let input = cb.checkout_layer_pixels(0)?;
                // checkout_layer_pixels may legitimately return None for an empty
                // adjustment-layer source. That is transparent input, not a reason
                // to leave AE's output buffer untouched.
                let result = (|| -> Result<(), ae::Error> {
                    if let Some(mut output) = cb.checkout_output()? {
                        let mut p = snapshot.render_params(in_data.as_ptr() as *mut c_void);
                        p.canvas_width = snapshot.canvas_width;
                        p.canvas_height = snapshot.canvas_height;
                        if let Some(layer) = input.as_ref() {
                            let origin = layer.origin();
                            p.input_origin_x = origin.h;
                            p.input_origin_y = origin.v;
                        }
                        let output_origin = output.origin();
                        p.output_origin_x = output_origin.h;
                        p.output_origin_y = output_origin.v;
                        if snapshot.plane.corners.is_some() {
                            plane::render(input.as_ref(), &mut output, &p, &snapshot.plane)?;
                        } else { render_sparse(input.as_ref(), &mut output, &p)?; }
                    }
                    Ok(())
                })();
                let checkin = cb.checkin_layer_pixels(0);
                match (result, checkin) {
                    (Err(e), _) => return Err(e),
                    (Ok(_), Err(e)) => return Err(e),
                    (Ok(_), Ok(_)) => {}
                }
            }
            #[cfg(target_os = "macos")]
            ae::Command::GpuDeviceSetup { mut extra } => {
                if extra.what_gpu() == ae::GpuFramework::Metal {
                    let gpu = ae::pf::suites::GPUDevice::new()?;
                    let info = gpu.device_info(in_data.effect_ref(), extra.device_index())?;
                    let state = unsafe { eg_metal_create(info.devicePV, info.command_queuePV) };
                    if !state.is_null() {
                        extra.set_gpu_data(MetalGpuData { state });
                        out_data.set_out_flag2(ae::OutFlags2::SupportsGpuRenderF32, true);
                    }
                }
            }
            #[cfg(target_os = "macos")]
            ae::Command::GpuDeviceSetdown { mut extra } => {
                if extra.what_gpu() == ae::GpuFramework::Metal {
                    let has_data = unsafe {
                        let raw = extra.as_ptr();
                        !raw.is_null() && !(*raw).input.is_null() && !(*(*raw).input).gpu_data.is_null()
                    };
                    if has_data {
                        // MetalGpuData owns the native state. Dropping the boxed
                        // GPU data invokes its Drop implementation exactly once.
                        extra.destroy_gpu_data::<MetalGpuData>();
                    }
                }
            }
            #[cfg(target_os = "macos")]
            ae::Command::SmartRenderGpu { extra } => {
                let cb = extra.callbacks();
                let Some(input) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };
                let result = (|| -> Result<(), ae::Error> {
                    if let Some(output) = cb.checkout_output()? {
                        let snapshot = extra
                            .pre_render_data::<SmartRenderSnapshot>()
                            .ok_or(ae::Error::InternalStructDamaged)?;
                        let mut p = snapshot.render_params(in_data.as_ptr() as *mut c_void);
                        apply_spatial_context(in_data, &input, &output, &mut p);
                        let gpu_data = extra.gpu_data::<MetalGpuData>().ok_or(ae::Error::InternalStructDamaged)?;
                        render_metal(in_data, &extra, input, output, &p, gpu_data)?;
                    }
                    Ok(())
                })();
                let checkin = cb.checkin_layer_pixels(0);
                match (result, checkin) {
                    (Err(e), _) => return Err(e),
                    (Ok(_), Err(e)) => return Err(e),
                    (Ok(_), Ok(_)) => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct LegacyGridArb {
        columns: u16,
        rows: u16,
        column_lines: Vec<f32>,
        row_lines: Vec<f32>,
        column_pins: Vec<u8>,
        row_pins: Vec<u8>,
    }

    #[test]
    fn saved_falloff_ordinals_keep_their_render_behavior() {
        // Baseline e1aa77e: retain all four numeric values while fixing labels.
        let expected = [
            (1, [0.0, 0.27407408, 0.5, 0.6740741, 0.82592595, 1.0]),
            (2, [0.0, 0.26411805, 0.5, 0.66411805, 0.8169013, 1.0]),
            (3, [0.0, 0.26666665, 0.5, 0.6666667, 0.8333333, 1.0]),
            (4, [0.0, 0.27407408, 0.5, 0.6740741, 0.82592595, 1.0]),
        ];
        for (falloff, reference) in expected {
            let mut lines = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
            let mut pins = [1, 0, 0, 0, 0, 1];
            let settings = EgElasticParams {
                tension_radius: 3.0, falloff, elasticity_strength: 1.0, min_spacing: 0.005,
            };
            // SAFETY: arrays contain exactly the six elements passed to the ABI.
            assert_eq!(unsafe {
                eg_drag_axis(lines.as_mut_ptr(), pins.as_mut_ptr(), 6, 2, 0.5, &settings)
            }, 0);
            for (actual, expected) in lines.into_iter().zip(reference) {
                assert!((actual - expected).abs() < 1.0e-6);
            }
        }
    }

    #[test]
    fn smart_render_snapshot_preserves_deformed_grid() {
        let mut grid = GridArb::uniform(4, 4);
        grid.column_lines[2] = 0.47;
        grid.row_lines[3] = 0.66;
        assert!(grid.is_valid());

        let snapshot = SmartRenderSnapshot {
            grid: grid.clone(),
            plane: plane::State::default(),
            tension_radius: 3.0,
            falloff: 2,
            elasticity_strength: 1.0,
            min_spacing: 0.005,
            stretch_easing: 0.25,
            easing_distance: 0.2,
            wave_amplitude: 0.0,
            wave_frequency: 1.0,
            wave_phase: 0.0,
            wave_speed: 0.0,
            wave_axis: 1,
            edge_mode: 1,
            quality: 2,
            time_seconds: 0.0,
            canvas_width: 640,
            canvas_height: 360,
        };
        let p = snapshot.render_params(std::ptr::null_mut());

        assert_eq!(p.columns, 4);
        assert_eq!(p.rows, 4);
        assert_eq!(p.column_line_count as usize, grid.column_lines.len());
        assert_eq!(p.row_line_count as usize, grid.row_lines.len());
        unsafe {
            assert_eq!(*p.column_lines.add(2), 0.47);
            assert_eq!(*p.row_lines.add(3), 0.66);
        }
    }

    fn legacy_from(g: &GridArb) -> LegacyGridArb {
        LegacyGridArb {
            columns: g.columns,
            rows: g.rows,
            column_lines: g.column_lines.clone(),
            row_lines: g.row_lines.clone(),
            column_pins: g.column_pins.clone(),
            row_pins: g.row_pins.clone(),
        }
    }

    #[test]
    fn grid_wire_roundtrip_and_legacy_migration() {
        let mut g = GridArb::uniform(7, 5);
        g.column_lines[3] = 0.39;
        g.row_lines[2] = 0.31;
        let encoded = bincode::serde::encode_to_vec(&g, bincode::config::legacy()).unwrap();
        let decoded: GridArb = bincode::serde::decode_from_slice(&encoded, bincode::config::legacy()).unwrap().0;
        assert_eq!(decoded, g);

        // v0.8 serialized the same six fields with a plain columns value.
        let legacy = legacy_from(&g);
        let legacy_bytes = bincode::serde::encode_to_vec(&legacy, bincode::config::legacy()).unwrap();
        let migrated: GridArb = bincode::serde::decode_from_slice(&legacy_bytes, bincode::config::legacy()).unwrap().0;
        assert_eq!(migrated, g);
    }

    #[test]
    fn grid_wire_rejects_unknown_version() {
        let g = GridArb::uniform(4, 4);
        let mut legacy = legacy_from(&g);
        legacy.columns = GRID_WIRE_MARKER | (4u16 << 8) | 4u16;
        let bytes = bincode::serde::encode_to_vec(&legacy, bincode::config::legacy()).unwrap();
        let decoded = bincode::serde::decode_from_slice::<GridArb, _>(&bytes, bincode::config::legacy());
        assert!(decoded.is_err());
    }

    #[test]
    fn abandoned_2d_mesh_state_migrates_without_host_error() {
        let columns = 4u16;
        let rows = 4u16;
        let side_x = columns as usize + 1;
        let side_y = rows as usize + 1;
        let mut legacy = LegacyGridArb {
            columns: GRID_WIRE_MARKER | (2u16 << 8) | columns,
            rows,
            column_lines: Vec::with_capacity(side_x * side_y),
            row_lines: Vec::with_capacity(side_x * side_y),
            column_pins: vec![0; side_x * side_y],
            row_pins: vec![0; side_x * side_y],
        };
        for r in 0..side_y {
            for c in 0..side_x {
                legacy.column_lines.push(c as f32 / columns as f32);
                legacy.row_lines.push(r as f32 / rows as f32);
            }
        }
        let bytes = bincode::serde::encode_to_vec(&legacy, bincode::config::legacy()).unwrap();
        let migrated: GridArb =
            bincode::serde::decode_from_slice(&bytes, bincode::config::legacy()).unwrap().0;
        assert!(migrated.is_valid());
        assert_eq!(migrated.column_lines.len(), columns as usize + 2);
        assert_eq!(migrated.row_lines.len(), rows as usize + 2);
    }

    #[test]
    fn grid_resize_resets_changed_axes_like_original() {
        let mut g = GridArb::uniform(4, 4);
        g.column_lines[1] = 0.18;
        g.row_lines[1] = 0.16;
        let r = g.resized(9, 4);
        assert_eq!(r.column_lines.len(), 11);
        assert_eq!(r.row_lines.len(), 6);
        // Changed X topology resets to uniform positions.
        for (i, value) in r.column_lines.iter().enumerate() {
            assert!((*value - i as f32 / 10.0).abs() < 1.0e-6);
        }
        // Unchanged Y topology preserves its deformation.
        assert!((r.row_lines[1] - 0.16).abs() < 1.0e-6);
    }

    #[test]
    fn arbitrary_interpolation_blends_same_topology() {
        let a = GridArb::uniform(4, 4);
        let mut b = a.clone();
        b.column_lines[2] = 0.5;
        let m = a.interpolate(&b, 0.5);
        // Four internal guides are uniformly 0.2/0.4/0.6/0.8.
        // Blend a valid guide #2 position from 0.4 to 0.5.
        assert!((m.column_lines[2] - 0.45).abs() < 1.0e-6);
        assert!(m.column_lines.windows(2).all(|w| w[1] > w[0]));
    }

    #[test]
    fn arbitrary_interpolation_steps_topology_changes() {
        let a = GridArb::uniform(4, 4);
        let b = GridArb::uniform(7, 5);
        assert_eq!(a.interpolate(&b, 0.49).columns, 4);
        assert_eq!(a.interpolate(&b, 0.51).columns, 7);
    }

    #[test]
    fn wave_cache_dependency_only_when_time_changes_output() {
        assert!(!wave_is_time_varying(false, 5.0, 1.0));
        assert!(!wave_is_time_varying(true, 0.0, 1.0));
        assert!(!wave_is_time_varying(true, 5.0, 0.0));
        assert!(wave_is_time_varying(true, 5.0, 1.0));
        assert!(wave_is_time_varying(true, 5.0, -1.0));
    }


    #[test]
    fn grid_wire_rejects_oversized_and_noncanonical_vectors() {
        let g = GridArb::uniform(4, 4);
        let mut oversized = legacy_from(&g);
        oversized.column_lines = vec![0.0; MAX_GUIDES + 2];
        let bytes = bincode::serde::encode_to_vec(&oversized, bincode::config::legacy()).unwrap();
        let decoded = bincode::serde::decode_from_slice::<GridArb, _>(&bytes, bincode::config::legacy());
        assert!(decoded.is_err());

        // Pin bytes from our old wire format are deliberately ignored during
        // migration because original GridWarp has no internal pin concept.
        let mut old_pin = legacy_from(&g);
        old_pin.column_pins[1] = 1;
        let bytes = bincode::serde::encode_to_vec(&old_pin, bincode::config::legacy()).unwrap();
        let decoded = bincode::serde::decode_from_slice::<GridArb, _>(&bytes, bincode::config::legacy()).unwrap().0;
        assert_eq!(decoded.column_pins[1], 0);
    }

    #[test]
    fn smart_render_snapshot_carries_logical_canvas() {
        let grid = GridArb::uniform(4, 4);
        let snapshot = SmartRenderSnapshot {
            grid,
            plane: plane::State::default(),
            tension_radius: 3.0,
            falloff: 2,
            elasticity_strength: 1.0,
            min_spacing: 0.005,
            stretch_easing: 0.0,
            easing_distance: 0.25,
            wave_amplitude: 0.0,
            wave_frequency: 1.0,
            wave_phase: 0.0,
            wave_speed: 0.0,
            wave_axis: 1,
            edge_mode: 1,
            quality: 2,
            time_seconds: 0.0,
            canvas_width: 640,
            canvas_height: 360,
        };
        let p = snapshot.render_params(std::ptr::null_mut());
        assert_eq!((p.canvas_width, p.canvas_height), (640, 360));
    }

    #[test]
    fn smart_result_rect_is_clamped_to_logical_canvas() {
        let r = clamp_request_to_canvas(
            ae::Rect { left: -50, top: 20, right: 900, bottom: 500 },
            640, 360
        );
        assert_eq!((r.left, r.top, r.right, r.bottom), (0, 20, 640, 360));
        let empty = clamp_request_to_canvas(
            ae::Rect { left: 700, top: 500, right: 650, bottom: 400 },
            640, 360
        );
        assert_eq!((empty.left, empty.top, empty.right, empty.bottom), (640, 360, 640, 360));
    }

    #[test]
    fn ffi_layout_is_frozen_on_64_bit_hosts() {
        assert_eq!(std::mem::size_of::<usize>(), 8);
        assert_eq!(std::mem::size_of::<EgRenderParams>(), 160);
        assert_eq!(std::mem::offset_of!(EgRenderParams, abort_fn), 144);
        assert_eq!(std::mem::offset_of!(EgRenderParams, abort_refcon), 152);
        assert_eq!(std::mem::size_of::<EgElasticParams>(), 16);
    }
}
