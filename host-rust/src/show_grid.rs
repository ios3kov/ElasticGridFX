//! Optional pixel visualization. No host UI calls, shared state or project writes.
use super::*;

#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    enabled: bool,
    counts: (usize, usize),
    layout: control_layout::State,
}
impl State {
    pub(crate) fn read(params: &ae::Parameters<Params>, checkout: bool) -> Result<Self, ae::Error> {
        Self::read_enabled(params, checkout, None)
    }
    pub(crate) fn read_demo(params: &ae::Parameters<Params>, checkout: bool) -> Result<Self, ae::Error> {
        if !demo::ENABLED {return Ok(Self::default());}
        Self::read_enabled(params, checkout, Some(true))
    }
    fn read_enabled(params: &ae::Parameters<Params>, checkout: bool, forced: Option<bool>) -> Result<Self, ae::Error> {
        let enabled = if let Some(value) = forced {value} else if checkout {params.checkout(Params::ShowGrid)?.as_checkbox()?.value()}
            else {params.get(Params::ShowGrid)?.as_checkbox()?.value()};
        if !enabled {return Ok(Self::default());}
        let (counts, layout) = if checkout {
            ((params.checkout(Params::Columns)?.as_slider()?.value(),
              params.checkout(Params::Rows)?.as_slider()?.value()),
             params.checkout(Params::ControlLayout)?.as_arbitrary()?.value::<control_layout::State>()?.clone())
        } else {
            ((params.get(Params::Columns)?.as_slider()?.value(), params.get(Params::Rows)?.as_slider()?.value()),
             params.get(Params::ControlLayout)?.as_arbitrary()?.value::<control_layout::State>()?.clone())
        };
        if !layout.valid() {return Err(ae::Error::BadCallbackParameter);}
        Ok(Self {enabled, counts: (counts.0.clamp(1, MAX_GUIDES as i32) as usize,
                                  counts.1.clamp(1, MAX_GUIDES as i32) as usize), layout})
    }
    pub(crate) fn render(&self, output: &mut ae::Layer, p: &EgRenderParams,
                         saved: &GridArb, plane: &plane::State) -> Result<(), ae::Error> {
        self.render_kind(output, p, saved, plane, false)
    }
    pub(crate) fn render_demo(&self, output: &mut ae::Layer, p: &EgRenderParams,
                              saved: &GridArb, plane: &plane::State) -> Result<(), ae::Error> {
        if !demo::ENABLED {return Ok(());}
        self.render_kind(output, p, saved, plane, true)
    }
    fn render_kind(&self, output: &mut ae::Layer, p: &EgRenderParams,
                   saved: &GridArb, plane: &plane::State, watermark: bool) -> Result<(), ae::Error> {
        if !self.enabled {return Ok(());}
        let mut evaluated = saved.clone();
        (evaluated.column_lines, evaluated.row_lines) = plane::evaluated_axes(p)?;
        let mut view = control_grid::view(&evaluated, self.counts, p.stretch_easing, p.easing_distance)?;
        control_grid::apply_layout(&mut view, &evaluated, self.counts, &self.layout,
                                   p.stretch_easing, p.easing_distance)?;
        // Invalid Four Corners already renders pass-through: do not invent a quad.
        let geometry = plane.geometry();
        if plane.corners.is_some() && geometry.is_none() {return Ok(());}
        let map = |x: f64, y: f64| -> Option<(f64, f64)> {
            if let Some(g) = &geometry {g.map(false, x, y)}
            else {Some((x * f64::from(p.canvas_width), y * f64::from(p.canvas_height)))}
        };
        let mut segments = Vec::with_capacity(self.counts.0 + self.counts.1);
        if watermark {
            for cell_y in view.grid.row_lines.windows(2) {
                for cell_x in view.grid.column_lines.windows(2) {
                    for (a,b) in demo::strokes([f64::from(cell_x[0]),f64::from(cell_y[0]),
                                              f64::from(cell_x[1]),f64::from(cell_y[1])]) {
                        if let (Some(a),Some(b))=(map(a.0,a.1),map(b.0,b.1)) {segments.push((a,b));}
                    }
                }
            }
        } else {
        for &x in &view.grid.column_lines[1..view.grid.column_lines.len()-1] {
            if let (Some(a), Some(b)) = (map(f64::from(x), 0.0), map(f64::from(x), 1.0)) {segments.push((a,b));}
        }
        for &y in &view.grid.row_lines[1..view.grid.row_lines.len()-1] {
            if let (Some(a), Some(b)) = (map(0.0, f64::from(y)), map(1.0, f64::from(y))) {segments.push((a,b));}
        }
        }
        let (width, height, stride, depth) = (output.width(), output.height(), output.row_bytes(), output.bit_depth());
        if width == 0 || height == 0 || stride == 0 {return Err(ae::Error::BadCallbackParameter);}
        let mut canvas = Canvas::new(output.buffer_mut(), width, height, stride, depth,
                                    (p.output_origin_x, p.output_origin_y))?;
        if watermark {canvas.color=[1.0,1.0,1.0,1.0];}
        for (a,b) in segments {
            canvas.stroke(a,b, &mut || {
                if let Some(abort) = p.abort_fn {
                    // SAFETY: synchronous callback/refcon owned by the current render.
                    if unsafe {abort(p.abort_refcon)} != 0 {return Err(ae::Error::InterruptCancel);}
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}

struct Canvas<'a> {bytes: &'a mut [u8], width: usize, height: usize, stride: usize,
                   reversed: bool, component: usize, origin: (i32,i32), color: [f64;4]}
impl<'a> Canvas<'a> {
    fn new(bytes: &'a mut [u8], width: usize, height: usize, stride: isize, depth: i16,
           origin: (i32,i32)) -> Result<Self,ae::Error> {
        let component = match depth {8=>1,16=>2,32=>4,_=>return Err(ae::Error::BadCallbackParameter)};
        let pitch = stride.checked_abs().ok_or(ae::Error::BadCallbackParameter)? as usize;
        if width==0 || height==0 || width.checked_mul(component*4).is_none_or(|v|v>pitch) ||
            pitch.checked_mul(height).is_none_or(|v|v>bytes.len()) {return Err(ae::Error::BadCallbackParameter);}
        Ok(Self {bytes,width,height,stride:pitch,reversed:stride<0,component,origin,color:[1.0,0.0,0.35,1.0]})
    }
    fn blend(&mut self, x: usize, y: usize, coverage: f64) {
        let row = if self.reversed {self.height-1-y} else {y};
        let start = row*self.stride + x*self.component*4;
        // Premultiplied source-over, coverage includes antialiasing.
        for (i, color) in self.color.into_iter().enumerate() {
            let offset = start+i*self.component;
            let old = match self.component {
                1=>f64::from(self.bytes[offset])/255.0,
                2=>f64::from(u16::from_ne_bytes(self.bytes[offset..offset+2].try_into().unwrap()))/32768.0,
                _=>f64::from(f32::from_ne_bytes(self.bytes[offset..offset+4].try_into().unwrap())),
            };
            let value = color*coverage + old*(1.0-coverage);
            match self.component {
                1=>self.bytes[offset]=(value*255.0).round().clamp(0.0,255.0) as u8,
                2=>self.bytes[offset..offset+2].copy_from_slice(&((value*32768.0).round().clamp(0.0,32768.0) as u16).to_ne_bytes()),
                _=>self.bytes[offset..offset+4].copy_from_slice(&(value as f32).to_ne_bytes()),
            }
        }
    }
    fn stroke(&mut self, a: (f64,f64), b: (f64,f64), abort: &mut impl FnMut()->Result<(),ae::Error>)
        -> Result<(),ae::Error> {
        abort()?;
        let a=(a.0-f64::from(self.origin.0),a.1-f64::from(self.origin.1));
        let b=(b.0-f64::from(self.origin.0),b.1-f64::from(self.origin.1));
        let dx=b.0-a.0; let dy=b.1-a.1; let length2=dx*dx+dy*dy;
        if ![a.0,a.1,b.0,b.1,length2].iter().all(|v|v.is_finite()) || length2<=0.0 {
            return Err(ae::Error::BadCallbackParameter);
        }
        // Iterate the major axis and only a narrow band around the segment.
        // Work grows with guide length, not with the entire frame per guide.
        let x_major=dx.abs()>=dy.abs();
        let (am,bm,an,dmajor,dminor,extent,other)=if x_major {
            (a.0,b.0,a.1,dx,dy,self.width,self.height)
        } else {(a.1,b.1,a.0,dy,dx,self.height,self.width)};
        let start=(am.min(bm)-1.0).floor().max(0.0) as usize;
        let end=((am.max(bm)+1.0).ceil().max(0.0) as usize).min(extent);
        for major in start.min(extent)..end {
            if major%1024==0 {abort()?;}
            let t=((major as f64-am)/dmajor).clamp(0.0,1.0);
            let minor=an+t*dminor;
            let lo=(minor-2.0).floor().max(0.0) as usize;
            let hi=((minor+2.0).ceil().max(0.0) as usize).min(other);
            for n in lo.min(other)..hi {
                let (x,y)=if x_major {(major,n)} else {(n,major)};
                let px=x as f64-a.0; let py=y as f64-a.1;
                let q=((px*dx+py*dy)/length2).clamp(0.0,1.0);
                let distance=((px-q*dx).powi(2)+(py-q*dy).powi(2)).sqrt();
                let coverage=(1.0-distance).clamp(0.0,1.0);
                if coverage>0.0 {self.blend(x,y,coverage);}
            }
        }
        Ok(())
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn disabled_bypasses_all_geometry_and_preserves_exact_bytes() {
        let mut bytes = vec![17u8; 64];
        // Zeroed C world/params are deliberately unusable for rendering. The
        // disabled path must not inspect them, call any host suite, or touch pixels.
        let mut world: ae::sys::PF_LayerDef = unsafe {std::mem::zeroed()};
        world.data = bytes.as_mut_ptr().cast(); world.width=4; world.height=4; world.rowbytes=16;
        let mut layer = ae::Layer::from_raw(&mut world, std::ptr::null::<ae::sys::PF_InData>(), None);
        let p: EgRenderParams = unsafe {std::mem::zeroed()};
        State::default().render(&mut layer, &p, &GridArb::uniform(4,4), &plane::State::default()).unwrap();
        // Even an enabled snapshot cannot bypass the OFF rollout gate.
        State {enabled:true, ..State::default()}.render_demo(&mut layer, &p, &GridArb::uniform(4,4), &plane::State::default()).unwrap();
        assert_eq!(bytes, vec![17u8;64]);
    }
    #[test] fn dormant_renderer_uses_evaluated_grid_and_does_not_write_saved_state() {
        let grid=GridArb::uniform(1,1);let retained=grid.clone();
        let mut p: EgRenderParams=unsafe {std::mem::zeroed()};
        p.columns=1;p.rows=1;p.column_lines=grid.column_lines.as_ptr();
        p.row_lines=grid.row_lines.as_ptr();p.column_pins=grid.column_pins.as_ptr();
        p.row_pins=grid.row_pins.as_ptr();p.column_line_count=3;p.row_line_count=3;
        p.canvas_width=100;p.canvas_height=100;p.falloff=2;
        p.tension_radius=3.;p.elasticity_strength=1.;p.easing_distance=0.25;
        p.min_spacing=0.005;p.quality=2;p.edge_mode=1;
        let mut bytes=vec![0u8;800*100];
        let mut world: ae::sys::PF_LayerDef=unsafe {std::mem::zeroed()};
        world.data=bytes.as_mut_ptr().cast();world.width=100;world.height=100;world.rowbytes=800;
        // Deep world flags resolve depth without a live PF_WorldSuite.
        world.world_flags=ae::sys::PF_WorldFlag_DEEP as _;
        let in_data: ae::sys::PF_InData=unsafe {std::mem::zeroed()};
        let mut layer=ae::Layer::from_raw(&mut world,&in_data as *const ae::sys::PF_InData,None);
        State {enabled:true,counts:(1,1),layout:control_layout::State::default()}
            .render_kind(&mut layer,&p,&grid,&plane::State::default(),true).unwrap();
        for y in 0..2 {for x in 0..2 {
            assert!((y*50..(y+1)*50).any(|yy| bytes[yy*800+x*400..yy*800+(x+1)*400].iter().any(|&v|v!=0)));
        }}
        assert_eq!(grid.column_lines,retained.column_lines);assert_eq!(grid.row_lines,retained.row_lines);
    }
    #[test] fn procedural_letters_render_each_cell_with_depth_padding_and_tile_parity() {
        // Exercise dormant rasterizer directly; production rollout remains OFF.
        let geometry=plane::State {corners:Some([-10.,-8.,90.,-4.,84.,86.,-6.,80.]), ..plane::State::default()}.geometry().unwrap();
        let mut lines=Vec::new();
        for y in 0..2 {for x in 0..2 {
            for (a,b) in demo::strokes([x as f64/2.,y as f64/2.,(x+1) as f64/2.,(y+1) as f64/2.]) {
                lines.push((geometry.map(false,a.0,a.1).unwrap(),geometry.map(false,b.0,b.1).unwrap()));
            }
        }}
        assert_eq!(lines.len(),80);
        for depth in [8,16,32] {
            let component=(depth/8) as usize; let stride=100*4*component+8;
            let mut full=vec![0;stride*100];let tile_pitch=35*4*component+8;
            let mut tile=vec![0;tile_pitch*31];
            for (bytes,w,h,pitch,origin) in [(&mut full,100,100,stride,(-12,-12)),
                                            (&mut tile,35,31,tile_pitch,(8,10))] {
                let mut canvas=Canvas::new(bytes,w,h,pitch as isize,depth,origin).unwrap();
                canvas.color=[1.;4];
                for &(a,b) in &lines {canvas.stroke(a,b,&mut ||Ok(())).unwrap();}
            }
            for y in 0..31 {
                assert_eq!(&tile[y*tile_pitch..y*tile_pitch+35*4*component],
                           &full[(y+22)*stride+20*4*component..(y+22)*stride+55*4*component]);
            }
            for row in full.chunks_exact(stride) {assert!(row[100*4*component..].iter().all(|&v|v==0));}
            // Every mapped cell has visible lettering, not merely one global mark.
            for y in 0..2 {for x in 0..2 {
                let center=geometry.map(false,(x as f64+0.5)/2.,(y as f64+0.5)/2.).unwrap();
                let cx=(center.0+12.) as usize;let cy=(center.1+12.) as usize;
                assert!((cy-10..cy+10).any(|yy| full[yy*stride+(cx-20)*4*component..yy*stride+(cx+20)*4*component].iter().any(|&v|v!=0)));
            }}
        }
    }
    #[test] fn transparent_stroke_and_hdr_use_premultiplied_source_over() {
        let mut bytes=vec![0u8;4];
        Canvas::new(&mut bytes,1,1,4,8,(0,0)).unwrap().blend(0,0,0.5);
        assert_eq!(bytes, [128,0,45,128]);
        let mut hdr: Vec<u8>=[1.0f32,4.0,-2.0,0.0].into_iter().flat_map(f32::to_ne_bytes).collect();
        Canvas::new(&mut hdr,1,1,16,32,(0,0)).unwrap().blend(0,0,0.5);
        let values: Vec<f32>=hdr.chunks_exact(4).map(|v|f32::from_ne_bytes(v.try_into().unwrap())).collect();
        assert_eq!(values, [1.0,2.0,-0.825,0.5]);
    }
    #[test] fn alpha_depth_padding_and_negative_stride() {
        for depth in [8,16,32] {
            let c=(depth/8) as usize; let stride=5*4*c+8;
            let mut bytes=vec![0;stride*5];
            Canvas::new(&mut bytes,5,5,-(stride as isize),depth,(0,0)).unwrap()
                .stroke((2.0,0.0),(2.0,4.0),&mut ||Ok(())).unwrap();
            assert!(bytes[stride*2+2*4*c..stride*2+3*4*c].iter().any(|&v|v!=0));
            for row in bytes.chunks_exact(stride) {assert!(row[5*4*c..].iter().all(|&v|v==0));}
        }
    }
    #[test] fn compact_tile_matches_full_frame() {
        let mut full=vec![0;16*16*4];let mut tile=vec![0;6*7*4];
        let a=(1.2,2.3);let b=(15.0,13.0);
        Canvas::new(&mut full,16,16,64,8,(0,0)).unwrap().stroke(a,b,&mut ||Ok(())).unwrap();
        Canvas::new(&mut tile,6,7,24,8,(4,5)).unwrap().stroke(a,b,&mut ||Ok(())).unwrap();
        for y in 0..7 {assert_eq!(&tile[y*24..(y+1)*24],&full[(y+5)*64+16..(y+5)*64+40]);}
    }
    #[test] fn cancel_and_invalid_storage_reject_without_writes() {
        let mut bytes=vec![0;64];
        assert!(Canvas::new(&mut bytes,5,4,16,8,(0,0)).is_err());
        let mut c=Canvas::new(&mut bytes,4,4,16,8,(0,0)).unwrap();
        assert!(c.stroke((0.0,0.0),(3.0,3.0),&mut ||Err(ae::Error::InterruptCancel)).is_err());
        assert!(bytes.iter().all(|&v|v==0));
    }
}
