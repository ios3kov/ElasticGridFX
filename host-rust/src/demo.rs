//! Dormant procedural watermark; no assets, font files, entitlement or host calls.
//! Rollout remains OFF. Marketplace entitlement must be integrated before enabling.
pub(crate) const ENABLED: bool = false;
type Point = (f64, f64);
type Segment = (Point, Point);

// Original single-line vector lettering: FSTR FX. Coordinates are geometry,
// not an encrypted asset or a claim of resistance to binary modification.
const F: &[Segment] = &[((0.,1.),(0.,0.)),((0.,0.),(1.,0.)),((0.,0.5),(0.8,0.5))];
const S: &[Segment] = &[((1.,0.),(0.,0.)),((0.,0.),(0.,0.5)),((0.,0.5),(1.,0.5)),((1.,0.5),(1.,1.)),((1.,1.),(0.,1.))];
const T: &[Segment] = &[((0.,0.),(1.,0.)),((0.5,0.),(0.5,1.))];
const R: &[Segment] = &[((0.,1.),(0.,0.)),((0.,0.),(1.,0.)),((1.,0.),(1.,0.5)),((1.,0.5),(0.,0.5)),((0.,0.5),(1.,1.))];
const X: &[Segment] = &[((0.,0.),(1.,1.)),((1.,0.),(0.,1.))];

pub(crate) fn strokes(cell: [f64;4]) -> Vec<Segment> {
    let [left,top,right,bottom]=cell;
    if !cell.iter().all(|v|v.is_finite()) || right<=left || bottom<=top {return Vec::new();}
    // Centered, 70% of cell width and 24% of height; all strokes stay in cell.
    let width=(right-left)*0.7; let height=(bottom-top)*0.24;
    let x=left+(right-left-width)*0.5; let y=top+(bottom-top-height)*0.5;
    let unit=width/9.5;
    let mut result=Vec::with_capacity(20);
    for (offset,glyph) in [(0.,F),(1.5,S),(3.,T),(4.5,R),(7.,F),(8.5,X)] {
        for &(a,b) in glyph {
            result.push(((x+(offset+a.0)*unit,y+a.1*height),
                         (x+(offset+b.0)*unit,y+b.1*height)));
        }
    }
    result
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn lettering_is_centered_bounded_and_rejects_bad_cells() {
        let lines=strokes([-3.,2.,7.,8.]);
        assert_eq!(lines.len(),20);
        for (a,b) in lines {for (x,y) in [a,b] {assert!((-1.5..=5.5).contains(&x));assert!((4.28-1e-12..=5.72+1e-12).contains(&y));}}
        assert!(strokes([0.,0.,0.,1.]).is_empty());
        assert!(strokes([0.,0.,f64::NAN,1.]).is_empty());
    }
}
