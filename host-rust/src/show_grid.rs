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
        let enabled = if checkout {params.checkout(Params::ShowGrid)?.as_checkbox()?.value()}
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
        for &x in &view.grid.column_lines[1..view.grid.column_lines.len()-1] {
            if let (Some(a), Some(b)) = (map(f64::from(x), 0.0), map(f64::from(x), 1.0)) {segments.push((a,b));}
        }
        for &y in &view.grid.row_lines[1..view.grid.row_lines.len()-1] {
            if let (Some(a), Some(b)) = (map(0.0, f64::from(y)), map(1.0, f64::from(y))) {segments.push((a,b));}
        }
        let (width, height, stride, depth) = (output.width(), output.height(), output.row_bytes(), output.bit_depth());
        if width == 0 || height == 0 || stride == 0 {return Err(ae::Error::BadCallbackParameter);}
        let mut canvas = Canvas::new(output.buffer_mut(), width, height, stride, depth,
                                    (p.output_origin_x, p.output_origin_y))?;
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
                   reversed: bool, component: usize, origin: (i32,i32)}
impl<'a> Canvas<'a> {
    fn new(bytes: &'a mut [u8], width: usize, height: usize, stride: isize, depth: i16,
           origin: (i32,i32)) -> Result<Self,ae::Error> {
        let component = match depth {8=>1,16=>2,32=>4,_=>return Err(ae::Error::BadCallbackParameter)};
        let pitch = stride.checked_abs().ok_or(ae::Error::BadCallbackParameter)? as usize;
        if width==0 || height==0 || width.checked_mul(component*4).is_none_or(|v|v>pitch) ||
            pitch.checked_mul(height).is_none_or(|v|v>bytes.len()) {return Err(ae::Error::BadCallbackParameter);}
        Ok(Self {bytes,width,height,stride:pitch,reversed:stride<0,component,origin})
    }
    fn blend(&mut self, x: usize, y: usize, coverage: f64) {
        let row = if self.reversed {self.height-1-y} else {y};
        let start = row*self.stride + x*self.component*4;
        // Premultiplied source-over, fixed blue, coverage includes antialiasing.
        for (i, color) in [1.0,0.0,0.35,1.0].into_iter().enumerate() {
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
        assert_eq!(bytes, vec![17u8;64]);
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
