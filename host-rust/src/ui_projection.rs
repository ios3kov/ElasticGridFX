//! UI-only projection. Never consulted by render callbacks or cached across events.
use super::*;
type Mat<const N: usize> = [[f64; N]; N];

fn inverse<const N: usize>(mut a: Mat<N>) -> Option<Mat<N>> {
    let mut b = [[0.0; N]; N];
    for (i, row) in b.iter_mut().enumerate() { row[i] = 1.0; }
    if !a.iter().flatten().all(|v| v.is_finite()) { return None; }
    for col in 0..N {
        let pivot = (col..N).max_by(|&x,&y| a[x][col].abs().total_cmp(&a[y][col].abs()))?;
        if a[pivot][col].abs() < 1e-12 { return None; }
        a.swap(col,pivot); b.swap(col,pivot);
        let divisor=a[col][col];
        for j in 0..N { a[col][j]/=divisor; b[col][j]/=divisor; }
        for i in 0..N { if i!=col {
            let factor=a[i][col];
            for j in 0..N { a[i][j]-=factor*a[col][j]; b[i][j]-=factor*b[col][j]; }
        }}
    }
    b.iter().flatten().all(|v|v.is_finite()).then_some(b)
}

pub(crate) struct Projection { forward: Mat<3>, backward: Mat<3> }
impl Projection {
    fn new(layer: Mat<4>, camera: Mat<4>, distance:f64, width:f64, height:f64) -> Option<Self> {
        if ![distance,width,height].iter().all(|v|v.is_finite()&&*v>0.0) {return None;}
        let view=inverse(camera)?;
        let mut model=[[0.0;4];4];
        for i in 0..4 {for j in 0..4 {for k in 0..4 {model[i][j]+=layer[i][k]*view[k][j];}}}
        // AE uses row vectors. Restrict layer z=0, then perspective divide.
        let mut h=[[0.0;3];3];
        for (i,row) in [0,1,3].into_iter().enumerate() {
            h[0][i]=distance*model[row][0]+width*0.5*model[row][2];
            h[1][i]=distance*model[row][1]+height*0.5*model[row][2];
            h[2][i]=model[row][2];
        }
        Some(Self{forward:h,backward:inverse(h)?})
    }
    fn apply(h:Mat<3>,x:f64,y:f64)->Option<(f64,f64)> {
        let w=h[2][0]*x+h[2][1]*y+h[2][2];
        if !w.is_finite()||w.abs()<1e-9 {return None;}
        let a=(h[0][0]*x+h[0][1]*y+h[0][2])/w;
        let b=(h[1][0]*x+h[1][1]*y+h[1][2])/w;
        (a.is_finite()&&b.is_finite()).then_some((a,b))
    }
    pub fn forward(&self,x:f64,y:f64)->Option<(f64,f64)> {
        if self.forward[2][0]*x+self.forward[2][1]*y+self.forward[2][2]<=1e-9 {return None;}
        Self::apply(self.forward,x,y)
    }
    pub fn backward(&self,x:f64,y:f64)->Option<(f64,f64)> {
        let p=Self::apply(self.backward,x,y)?;
        self.forward(p.0,p.1)?;
        Some(p)
    }
}

pub(crate) fn read(in_data:&ae::InData,event:&ae::EventExtra)->Result<Option<Projection>,ae::Error> {
    if event.window_type()!=ae::WindowType::Comp {return Ok(None);}
    let interface=ae::aegp::suites::PFInterface::new()?;
    let layers=ae::aegp::suites::Layer::new()?;
    let layer=interface.effect_layer(in_data.effect_ref())?;
    if !layers.is_layer_3d(layer)? {return Ok(None);}
    // Non-square projection must be verified separately; never draw a false plane.
    let par=in_data.pixel_aspect_ratio();
    if i64::from(par.num)!=i64::from(par.den) {return Err(ae::Error::BadCallbackParameter);}
    let comp=layers.layer_parent_comp(layer)?;
    let item=ae::aegp::suites::Comp::new()?.item_from_comp(comp)?;
    let comp_par=ae::aegp::suites::Item::new()?.item_pixel_aspect_ratio(item)?;
    if i64::from(comp_par.num)!=i64::from(comp_par.den) {return Err(ae::Error::BadCallbackParameter);}
    let time=interface.convert_effect_to_comp_time(in_data.effect_ref(),in_data.current_time(),in_data.time_scale())?;
    let world:ae::sys::A_Matrix4=layers.layer_to_world_xform(layer,time)?.into();
    let (camera,distance,width,height)=interface.effect_camera_matrix(in_data.effect_ref(),time)?;
    let camera:ae::sys::A_Matrix4=camera.into();
    Projection::new(world.mat,camera.mat,distance,width as f64,height as f64)
        .map(Some).ok_or(ae::Error::BadCallbackParameter)
}

#[cfg(test)] mod tests {
    use super::*;
    fn unit()->Mat<4>{[[1.,0.,0.,0.],[0.,1.,0.,0.],[0.,0.,1.,0.],[0.,0.,0.,1.]]}
    #[test] fn camera_plane_roundtrip_and_layer_motion(){
        let mut camera=unit();camera[3]=[320.,240.,-800.,1.];
        let p=Projection::new(unit(),camera,800.,640.,480.).unwrap();
        assert_eq!(p.forward(10.,20.),Some((10.,20.)));
        let mut layer=unit();let angle=0.6_f64;
        layer[0]=[angle.cos(),0.,-angle.sin(),0.];layer[2]=[angle.sin(),0.,angle.cos(),0.];
        layer[3]=[80.,-40.,100.,1.];
        let p=Projection::new(layer,camera,800.,640.,480.).unwrap();
        for x in [0.,100.,500.] {for y in [0.,100.,400.] {
            let q=p.forward(x,y).unwrap();let r=p.backward(q.0,q.1).unwrap();
            assert!((r.0-x).abs()<1e-8&&(r.1-y).abs()<1e-8);
            assert!((q.0-x).abs()>1.);
        }}
    }
    #[test] fn reject_singular_nonfinite_and_behind_camera(){
        assert!(Projection::new(unit(),[[0.;4];4],800.,640.,480.).is_none());
        let mut camera=unit();camera[3][2]=800.;
        let p=Projection::new(unit(),camera,800.,640.,480.).unwrap();
        assert!(p.forward(0.,0.).is_none());assert!(p.backward(0.,0.).is_none());
        assert!(Projection::new(unit(),camera,f64::NAN,640.,480.).is_none());
    }
}
