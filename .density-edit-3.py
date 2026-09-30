put('host-rust/src/lib.rs', [
    (7,0,r'''mod guide_density;
mod detail_map;
'''),
    (74,1,r'''#[derive(Clone, Debug, PartialEq, PartialOrd)]
'''),
    (76,1,r''''''),
    (87,0,r'''    pub(crate) column_detail: Vec<f32>,
    pub(crate) row_detail: Vec<f32>,
'''),
    (89,19,r'''// Preserve the exact v3 wire while no detail edit has occurred. A count change
// therefore cannot rewrite even the serialization version. v4 appends only
// actual geometry, never the visible counts. Older plugins reject v4 explicitly.
const GRID_DETAIL_WIRE_VERSION: u16 = 4;
impl Serialize for GridArb {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let detailed = !self.column_detail.is_empty() || !self.row_detail.is_empty();
        let version = if detailed { GRID_DETAIL_WIRE_VERSION } else { GRID_WIRE_VERSION };
        let wire = GRID_WIRE_MARKER | (version << 8) | self.columns;
        let mut tuple = serializer.serialize_tuple(if detailed {8} else {6})?;
        tuple.serialize_element(&wire)?;
        tuple.serialize_element(&self.rows)?;
        tuple.serialize_element(&self.column_lines)?;
        tuple.serialize_element(&self.row_lines)?;
        tuple.serialize_element(&self.column_pins)?;
        tuple.serialize_element(&self.row_pins)?;
        if detailed { tuple.serialize_element(&self.column_detail)?; tuple.serialize_element(&self.row_detail)?; }
        tuple.end()
'''),
    (109,9,r''''''),
    (175,0,r'''struct BoundedAxis(#[serde(deserialize_with = "deserialize_bounded_f32_vec")] Vec<f32>);
#[derive(Deserialize)]
struct BoundedPins(#[serde(deserialize_with = "deserialize_bounded_u8_vec")] Vec<u8>);
#[derive(Deserialize)]
struct BoundedDetail(#[serde(deserialize_with = "deserialize_detail_vec")] Vec<f32>);
fn deserialize_detail_vec<'de,D:Deserializer<'de>>(deserializer:D)->Result<Vec<f32>,D::Error> {
    struct DetailVisitor;
    impl<'de> Visitor<'de> for DetailVisitor {
        type Value=Vec<f32>;
        fn expecting(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {f.write_str("an empty or 257-sample detail map")}
        fn visit_seq<A:SeqAccess<'de>>(self,mut seq:A)->Result<Vec<f32>,A::Error> {
            if seq.size_hint().is_some_and(|n|n>detail_map::SAMPLES) {return Err(serde::de::Error::custom("oversized detail map"));}
            let mut values=Vec::new();
            while let Some(value)=seq.next_element::<f32>()? {
                if values.len()==detail_map::SAMPLES {return Err(serde::de::Error::custom("oversized detail map"));}
                values.push(value);
            }
            if !detail_map::valid(&values) {return Err(serde::de::Error::custom("invalid detail map"));}
            Ok(values)
        }
    }
    deserializer.deserialize_seq(DetailVisitor)
}
'''),
    (176,11,r'''    columns:u16, rows:u16, column_lines:Vec<f32>,row_lines:Vec<f32>,
    column_pins:Vec<u8>,row_pins:Vec<u8>,column_detail:Vec<f32>,row_detail:Vec<f32>,
}
impl<'de> Deserialize<'de> for GridArbWire {
    fn deserialize<D:Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
        struct WireVisitor;
        impl<'de> Visitor<'de> for WireVisitor {
            type Value=GridArbWire;
            fn expecting(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {f.write_str("a versioned ElasticGrid state")}
            fn visit_seq<A:SeqAccess<'de>>(self,mut seq:A)->Result<Self::Value,A::Error> {
                fn next<'de,T:Deserialize<'de>,A:SeqAccess<'de>>(seq:&mut A)->Result<T,A::Error> {
                    seq.next_element()?.ok_or_else(||serde::de::Error::custom("truncated ElasticGrid state"))
                }
                let wire:u16=next(&mut seq)?;
                let version=if wire&GRID_WIRE_MARKER==0 {0} else {(wire>>8)&0x7f};
                let columns=if version==0 {wire} else {wire&0xff};
                if version>GRID_DETAIL_WIRE_VERSION || !(1..=MAX_GUIDES as u16).contains(&columns) {
                    return Err(serde::de::Error::custom("unsupported grid state version/count"));
                }
                let rows:u16=next(&mut seq)?;
                if !(1..=MAX_GUIDES as u16).contains(&rows) {return Err(serde::de::Error::custom("invalid row count"));}
                let column_lines=next::<BoundedAxis,_>(&mut seq)?.0;
                let row_lines=next::<BoundedAxis,_>(&mut seq)?.0;
                let column_pins=next::<BoundedPins,_>(&mut seq)?.0;
                let row_pins=next::<BoundedPins,_>(&mut seq)?.0;
                let (column_detail,row_detail)=if version==GRID_DETAIL_WIRE_VERSION {
                    (next::<BoundedDetail,_>(&mut seq)?.0,next::<BoundedDetail,_>(&mut seq)?.0)
                } else {(Vec::new(),Vec::new())};
                Ok(GridArbWire {columns,rows,column_lines,row_lines,column_pins,row_pins,column_detail,row_detail})
            }
        }
        // Binary tuple visitor consumes the six legacy fields or eight v4
        // fields according to its leading version marker, never by EOF probing.
        deserializer.deserialize_tuple(8,WireVisitor)
    }
'''),
    (240,0,r'''            column_detail: wire.column_detail,
            row_detail: wire.row_detail,
'''),
    (282,0,r'''            column_detail: Vec::new(),
            row_detail: Vec::new(),
'''),
    (318,0,r'''            && detail_map::valid(&self.column_detail) && detail_map::valid(&self.row_detail)
'''),
    (346,9,r''''''),
    (356,24,r''''''),
    (391,0,r'''        if value <= 0.0 { return self.clone(); }
        if value >= 1.0 { return other.clone(); }
'''),
    (396,0,r'''        out.column_detail = detail_map::interpolate(&self.column_detail, &other.column_detail, value.clamp(0.0,1.0));
        out.row_detail = detail_map::interpolate(&self.row_detail, &other.row_detail, value.clamp(0.0,1.0));
'''),
    (509,5,r''''''),
    (531,2,r''''''),
    (536,1,r'''        guide_density::render_grid(&value)?
'''),
    (577,0,r'''    #[cfg(test)]
'''),
    (590,0,r'''    #[cfg(test)]
'''),
    (602,0,r'''
    fn eg_render_frame_detail(
        input:*const c_void,input_pitch:isize,iw:i32,ih:i32,
        output:*mut c_void,output_pitch:isize,ow:i32,oh:i32,depth:i32,
        params:*const EgRenderParams,detail:*const detail_map::Maps,sparse:i32,
    )->i32;
    fn eg_axis_coordinates(axis:*const f32,count:i32,queries:*const f32,output:*mut f32,
                           n:i32,easing:f32,distance:f32,forward:i32)->i32;
'''),
    (692,1,r'''fn guide_counts(params: &ae::Parameters<Params>) -> Result<(usize, usize), ae::Error> {
'''),
    (700,1,r''''''),
    (702,15,r'''    guide_density::render_grid(&grid)
'''),
    (817,0,r'''    grid: &GridArb,
'''),
    (823,1,r'''        eg_render_frame_detail(
'''),
    (833,1,r'''            p, &detail_map::Maps::from_grid(grid), 0,
'''),
    (849,0,r'''    grid: &GridArb,
'''),
    (866,1,r'''        eg_render_frame_detail(
'''),
    (876,1,r'''            p, &detail_map::Maps::from_grid(grid), 1,
'''),
    (1103,1,r'''                    // Display density only: never create, replace or migrate a
                    // Grid Positions key, including when standing on a key.
                    ui::release_cursor();
                    out_data.set_out_flag(ae::OutFlags::ForceRerender, true);
'''),
    (1149,2,r'''                    plane::render(Some(&in_layer), &mut out_layer, &p, &plane, &grid)?;
                } else { render(&in_layer, &mut out_layer, &p, &grid)?; }
'''),
    (1210,2,r'''                            plane::render(input.as_ref(), &mut output, &p, &snapshot.plane, &snapshot.grid)?;
                        } else { render_sparse(input.as_ref(), &mut output, &p, &snapshot.grid)?; }
'''),
    (1259,0,r'''                        if !snapshot.grid.column_detail.is_empty() || !snapshot.grid.row_detail.is_empty() {
                            // GPU dispatch is disabled; never silently drop a detail field.
                            return Err(ae::Error::BadCallbackParameter);
                        }
'''),
    (1388,1,r'''        legacy.columns = GRID_WIRE_MARKER | (5u16 << 8) | 4u16;
'''),
    (1420,16,r''''''),
])
put('host-rust/src/plane.rs', [
    (226,0,r'''    fn eg_render_plane_detail(source:*const Image,output:*const Image,depth:i32,
        frame:*const Frame,report:*mut Report,quality:i32,edge:i32,
        detail:*const detail_map::Maps,layer:i32)->i32;
'''),
    (247,1,r'''                     p: &EgRenderParams, state: &State, grid: &GridArb) -> Result<(), ae::Error> {
'''),
    (271,2,r'''    let rc = if grid.column_detail.is_empty() && grid.row_detail.is_empty() {
        unsafe {render_plane(&src, &dst, output.bit_depth() as i32,
                             &frame, &mut report, p.quality - 1, p.edge_mode - 1)}
    } else {
        unsafe {eg_render_plane_detail(&src,&dst,output.bit_depth() as i32,&frame,&mut report,
            p.quality-1,p.edge_mode-1,&detail_map::Maps::from_grid(grid),i32::from(state.comp_space && !state.editable_corners))}
    };
'''),
])
put('host-rust/src/ui.rs', [
    (56,1,r'''fn evaluated_control_base(in_data: &ae::InData, params: &mut ae::Parameters<Params>, plane: &ViewPlane)
'''),
    (59,4,r'''    let _ = plane;
    let p = evaluated_params(params, *in_data, &grid)?;
    (grid.column_lines,grid.row_lines) = plane::evaluated_axes(&p)?;
'''),
    (64,0,r'''}

fn mapping_settings(params:&ae::Parameters<Params>)->Result<guide_density::Mapping,ae::Error> {
    Ok(guide_density::Mapping {
        easing:params.get(Params::StretchEasing)?.as_float_slider()?.value() as f32 / 100.0,
        distance:params.get(Params::EasingDistance)?.as_float_slider()?.value() as f32 / 100.0,
    })
}
fn displayed_grid(in_data:&ae::InData,params:&mut ae::Parameters<Params>,plane:&ViewPlane)->Result<GridArb,ae::Error> {
    let base=evaluated_control_base(in_data,params,plane)?;
    let (columns,rows)=guide_counts(params)?;
    guide_density::view_grid_mapped(&base,columns,rows,mapping_settings(params)?)
'''),
    (429,0,r'''        event.set_continue_refcon(2, (grid.columns as usize * 64 + grid.rows as usize) as _);
        let canonical = grid_snapshot(params)?;
        event.set_continue_refcon(3, (canonical.columns as usize * 64 + canonical.rows as usize) as _);
'''),
    (485,0,r'''    let (columns, rows) = guide_counts(params)?;
    if event.continue_refcon(2) != (columns * 64 + rows) as isize
        || event.continue_refcon(3) != (grid.columns as usize * 64 + grid.rows as usize) as isize {
        // Count/time changes during a drag must not silently select a new line.
        event.set_send_drag(false);
        return Ok(());
    }
'''),
    (486,34,r'''    let base=evaluated_control_base(in_data,params,&plane)?;
    let mapping=mapping_settings(params)?;
    let request=guide_density::Drag {
        columns:axis==DRAG_COLUMNS,visible:if axis==DRAG_COLUMNS {columns} else {rows},
        index,target:if axis==DRAG_COLUMNS {local_x} else {local_y},mapping,
'''),
    (521,2,r'''    let changed=guide_density::drag_control(&mut grid,&base,request,&elastic)?;
    if changed {
'''),
    (524,5,r'''        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT
            | ae::EventOutFlags::ALWAYS_UPDATE | ae::EventOutFlags::UPDATE_NOW);
'''),
])
put('src/bridge/detail_ffi.h', [
    (0,0,r'''#pragma once
#include <cstdint>
#include <span>
#include "core/DetailMap.h"

// New opt-in entry points use this record. The old render/plane ABIs stay frozen.
struct EgDetailMaps {
    const float* columns;
    std::int32_t column_count;
    const float* rows;
    std::int32_t row_count;
};
static_assert(sizeof(EgDetailMaps) == 32);
inline bool eg_detail_valid(const EgDetailMaps* maps) noexcept {
    if (!maps) return true;
    auto valid = [](const float* values, std::int32_t count) {
        if (count == 0) return true;
        if (!values || count != static_cast<std::int32_t>(elasticgrid::kDetailSamples)) return false;
        return elasticgrid::validDetail({values, static_cast<std::size_t>(count)});
    };
    return valid(maps->columns, maps->column_count) && valid(maps->rows, maps->row_count);
}
inline std::span<const float> eg_detail_x(const EgDetailMaps* maps) noexcept {
    return maps && maps->column_count > 0 ? std::span<const float>{maps->columns, static_cast<std::size_t>(maps->column_count)} : std::span<const float>{};
}
inline std::span<const float> eg_detail_y(const EgDetailMaps* maps) noexcept {
    return maps && maps->row_count > 0 ? std::span<const float>{maps->rows, static_cast<std::size_t>(maps->row_count)} : std::span<const float>{};
}
'''),
])
