use after_effects as ae;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use std::fmt;
use std::ffi::c_void;

mod ui;

const GRID_REFCON: u64 = 0x4547_4658_4752_4944; // "EGFXGRID"
const MAX_GUIDES: usize = 50;
const MAX_AXIS_POINTS: usize = MAX_GUIDES + 2;

const GRID_WIRE_VERSION: u16 = 2;
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
}

#[derive(Default)]
struct Plugin {}

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
    if version != 1 && version != GRID_WIRE_VERSION {
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
        if seq.size_hint().is_some_and(|n| n > MAX_AXIS_POINTS) {
            return Err(serde::de::Error::custom("ElasticGrid vector exceeds topology limit"));
        }
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(MAX_AXIS_POINTS));
        while let Some(value) = seq.next_element::<f32>()? {
            if out.len() >= MAX_AXIS_POINTS {
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
        if seq.size_hint().is_some_and(|n| n > MAX_AXIS_POINTS) {
            return Err(serde::de::Error::custom("ElasticGrid pin vector exceeds topology limit"));
        }
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0).min(MAX_AXIS_POINTS));
        while let Some(value) = seq.next_element::<u8>()? {
            if out.len() >= MAX_AXIS_POINTS {
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
        let columns = wire.columns as usize;
        let rows = wire.rows as usize;

        // v1 interpreted the same numeric value as cell count and stored N+1
        // points. v2 stores N internal guides + two boundaries = N+2 points.
        let column_lines = if wire.column_lines.len() == columns + 1 {
            GridArb::resample_axis_raw(&wire.column_lines, columns)
        } else {
            wire.column_lines
        };
        let row_lines = if wire.row_lines.len() == rows + 1 {
            GridArb::resample_axis_raw(&wire.row_lines, rows)
        } else {
            wire.row_lines
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

        let column_lines = if self.columns as usize == columns {
            self.column_lines.clone()
        } else {
            Self::resample_axis_raw(&self.column_lines, columns)
        };
        let row_lines = if self.rows as usize == rows {
            self.row_lines.clone()
        } else {
            Self::resample_axis_raw(&self.row_lines, rows)
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

#[repr(C)]
pub(crate) struct EgElasticParams {
    pub(crate) tension_radius: f32,
    pub(crate) falloff: i32,
    pub(crate) elasticity_strength: f32,
    pub(crate) min_spacing: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct EgRectI32 {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

unsafe extern "C" {
    fn eg_required_source_rect(
        canvas_width: i32,
        canvas_height: i32,
        output_rect: EgRectI32,
        params: *const EgRenderParams,
        source_rect: *mut EgRectI32,
    ) -> i32;

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
        tension_radius: params.get(Params::TensionRadius)?.as_slider()?.value() as f32,
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
        tension_radius: params.get(Params::TensionRadius)?.as_slider()?.value() as f32,
        falloff: params.get(Params::Falloff)?.as_popup()?.value(),
        elasticity_strength: params.get(Params::ElasticityStrength)?.as_float_slider()?.value() as f32 / 100.0,
        min_spacing: params.get(Params::MinSpacing)?.as_float_slider()?.value() as f32 / 100.0,
        stretch_easing: params.get(Params::StretchEasing)?.as_float_slider()?.value() as f32 / 100.0,
        easing_distance: params.get(Params::EasingDistance)?.as_float_slider()?.value() as f32 / 100.0,
        wave_enabled: params.get(Params::WaveEnabled)?.as_checkbox()?.value() as i32,
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
        params.add_with_flags(Params::Columns, "Num Columns", ae::SliderDef::setup(|f| {
            f.set_valid_min(1);
            f.set_valid_max(50);
            f.set_slider_min(1);
            f.set_slider_max(50);
            f.set_default(4);
            f.set_value(f.default());
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;
        params.add_with_flags(Params::Rows, "Num Rows", ae::SliderDef::setup(|f| {
            f.set_valid_min(1);
            f.set_valid_max(50);
            f.set_slider_min(1);
            f.set_slider_max(50);
            f.set_default(4);
            f.set_value(f.default());
        }), ae::ParamFlag::SUPERVISE, ae::ParamUIFlags::empty())?;

        let mut grid_state_def = ae::ArbitraryDef::new();
        grid_state_def.set_default(GridArb::default())?;
        grid_state_def.set_refcon(GRID_REFCON as *mut c_void);
        params.add_customized(Params::GridState, "Grid Positions", grid_state_def, |param| {
            param.set_ui_flags(ae::ParamUIFlags::CONTROL);
            param.set_ui_width(240);
            param.set_ui_height(24);
            -1
        })?;

        params.add(Params::TensionRadius, "Tension Radius", ae::SliderDef::setup(|f| {
            f.set_valid_min(0);
            f.set_valid_max(20);
            f.set_slider_min(0);
            f.set_slider_max(20);
            f.set_default(3);
            f.set_value(f.default());
        }))?;
        params.add(Params::Falloff, "Falloff Profile", ae::PopupDef::setup(|f| {
            f.set_options(&["Smoothstep", "Gaussian", "Linear"]);
            f.set_default(1);
            f.set_value(f.default());
        }))?;
        params.add(Params::ElasticityStrength, "Elasticity Strength", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 200.0), (0.0, 200.0), 100.0, 1, true);
        }))?;
        params.add(Params::MinSpacing, "Min Line Spacing", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 25.0), (0.0, 25.0), 0.5, 2, true);
        }))?;
        params.add(Params::StretchEasing, "Stretch Easing", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 100.0), (0.0, 100.0), 50.0, 1, true);
        }))?;
        params.add(Params::EasingDistance, "Easing Distance", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 100.0), (0.0, 100.0), 25.0, 1, true);
        }))?;

        params.add(Params::WaveEnabled, "Wave Animation", ae::CheckBoxDef::setup(|f| {
            f.set_label("Enable");
            f.set_default(false);
            f.set_value(f.default());
        }))?;
        params.add(Params::WaveAmplitude, "Wave Amplitude", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 100.0), (0.0, 100.0), 0.0, 1, true);
        }))?;
        params.add(Params::WaveFrequency, "Wave Frequency", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (0.0, 10.0), (0.0, 10.0), 1.0, 2, false);
        }))?;
        params.add(Params::WavePhase, "Wave Phase", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (-360.0, 360.0), (-360.0, 360.0), 0.0, 2, false);
        }))?;
        params.add(Params::WaveSpeed, "Wave Speed", ae::FloatSliderDef::setup(|f| {
            setup_float(f, (-10.0, 10.0), (-10.0, 10.0), 0.0, 2, false);
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
            f.set_options(&["Draft (Bilinear)", "Better (Bicubic)"]);
            f.set_default(1);
            f.set_value(f.default());
        }))?;

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
        match cmd {
            ae::Command::About => {
                out_data.set_return_msg("ElasticGrid FX v0.9\rfinal hardening build");
            }
            ae::Command::UserChangedParam { param_index } => {
                if params.index(Params::Columns) == Some(param_index)
                    || params.index(Params::Rows) == Some(param_index)
                {
                    sync_grid_topology(params)?;
                }
            }
            ae::Command::QueryDynamicFlags => {
                // Wave animation uses current_time even when no parameter has a
                // keyframe. Tell AE only when that time dependency is active,
                // so caching stays correct without disabling useful cache hits
                // for static warps. The PiPL/global setup advertises the flag
                // because AE requires dynamically-cleared flags to start set.
                let enabled = params.checkout(Params::WaveEnabled)?.as_checkbox()?.value();
                let amplitude = params.checkout(Params::WaveAmplitude)?.as_float_slider()?.value();
                let speed = params.checkout(Params::WaveSpeed)?.as_float_slider()?.value();
                out_data.set_out_flag(
                    ae::OutFlags::NonParamVary,
                    wave_is_time_varying(enabled, amplitude, speed),
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
                    _ => {}
                }
            }
            ae::Command::Render { in_layer, mut out_layer } => {
                let grid = grid_snapshot(params)?;
                let mut p = evaluated_params(params, in_data, &grid)?;
                apply_spatial_context(in_data, &in_layer, &out_layer, &mut p);
                render(&in_layer, &mut out_layer, &p)?;
            }
            ae::Command::SmartPreRender { mut extra } => {
                let output_request = extra.output_request();
                let output_rect = EgRectI32 {
                    left: output_request.rect.left,
                    top: output_request.rect.top,
                    right: output_request.rect.right,
                    bottom: output_request.rect.bottom,
                };
                let (cw, ch) = rendered_canvas(in_data);
                let grid = grid_snapshot(params)?;
                let mut p = evaluated_params(params, in_data, &grid)?;
                p.canvas_width = cw;
                p.canvas_height = ch;
                p.input_origin_x = 0;
                p.input_origin_y = 0;
                p.output_origin_x = output_rect.left;
                p.output_origin_y = output_rect.top;

                let mut source_rect = EgRectI32 { left: 0, top: 0, right: cw, bottom: ch };
                let rc = unsafe { eg_required_source_rect(cw, ch, output_rect, &p, &mut source_rect) };
                if rc != 0 {
                    // Conservative fallback: correctness first.
                    source_rect = EgRectI32 { left: 0, top: 0, right: cw, bottom: ch };
                }
                let mut request = output_request;
                request.rect.left = source_rect.left;
                request.rect.top = source_rect.top;
                request.rect.right = source_rect.right;
                request.rect.bottom = source_rect.bottom;
                let input = extra.callbacks().checkout_layer(
                    0, 0, &request,
                    in_data.current_time(), in_data.time_step(), in_data.time_scale(),
                )?;

                // A grid warp can redistribute content anywhere inside the fixed
                // layer canvas, so result bounds must not be inherited from the
                // smaller input alpha bounds. Keep current result conservative and
                // max bounds stable across render requests.
                extra.set_result_rect(output_request.rect.into());
                let mut max_rect = ae::Rect { left: 0, top: 0, right: cw, bottom: ch };
                let input_max: ae::Rect = input.max_result_rect.into();
                max_rect.union(&input_max);
                extra.set_max_result_rect(max_rect);
                #[cfg(target_os = "macos")]
                extra.set_gpu_render_possible(extra.what_gpu() == ae::GpuFramework::Metal && extra.bit_depth() == 32);
            }
            ae::Command::SmartRender { extra } => {
                let cb = extra.callbacks();
                let Some(input) = cb.checkout_layer_pixels(0)? else {
                    return Ok(());
                };
                // Never leak a SmartFX checkout on an error path. The closure
                // captures all fallible work; checkin is performed unconditionally.
                let result = (|| -> Result<(), ae::Error> {
                    if let Some(mut output) = cb.checkout_output()? {
                        let grid = grid_snapshot(params)?;
                        let mut p = evaluated_params(params, in_data, &grid)?;
                        apply_spatial_context(in_data, &input, &output, &mut p);
                        render(&input, &mut output, &p)?;
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
                        let grid = grid_snapshot(params)?;
                        let mut p = evaluated_params(params, in_data, &grid)?;
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
        legacy.columns = GRID_WIRE_MARKER | (3u16 << 8) | 4u16;
        let bytes = bincode::serde::encode_to_vec(&legacy, bincode::config::legacy()).unwrap();
        let decoded = bincode::serde::decode_from_slice::<GridArb, _>(&bytes, bincode::config::legacy());
        assert!(decoded.is_err());
    }

    #[test]
    fn grid_resize_preserves_monotonic_shape() {
        let mut g = GridArb::uniform(4, 4);
        g.column_lines[1] = 0.18;
        g.column_lines[2] = 0.62;
        g.column_lines[3] = 0.84;
        let r = g.resized(9, 7);
        assert_eq!(r.column_lines.len(), 11);
        assert_eq!(r.row_lines.len(), 9);
        assert!(r.column_lines.windows(2).all(|w| w[1] > w[0]));
        assert!(r.row_lines.windows(2).all(|w| w[1] > w[0]));
        assert_eq!(r.column_lines[0], 0.0);
        assert_eq!(*r.column_lines.last().unwrap(), 1.0);
    }

    #[test]
    fn arbitrary_interpolation_blends_same_topology() {
        let a = GridArb::uniform(4, 4);
        let mut b = a.clone();
        b.column_lines[2] = 0.7;
        let m = a.interpolate(&b, 0.5);
        assert!((m.column_lines[2] - 0.6).abs() < 1.0e-6);
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
    fn ffi_layout_is_frozen_on_64_bit_hosts() {
        assert_eq!(std::mem::size_of::<usize>(), 8);
        assert_eq!(std::mem::size_of::<EgRenderParams>(), 160);
        assert_eq!(std::mem::offset_of!(EgRenderParams, abort_fn), 144);
        assert_eq!(std::mem::offset_of!(EgRenderParams, abort_refcon), 152);
        assert_eq!(std::mem::size_of::<EgElasticParams>(), 16);
        assert_eq!(std::mem::size_of::<EgRectI32>(), 16);
    }
}
