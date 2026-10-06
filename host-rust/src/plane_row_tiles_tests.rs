use super::*;

fn concurrent(count:i32,callback:&(dyn Fn(i32)->Result<(),ae::Error>+Sync))->Result<(),ae::Error> {
    std::thread::scope(|scope| {
        let tasks:Vec<_>=(0..count).rev().map(|i|scope.spawn(move||callback(i))).collect();
        for task in tasks {task.join().expect("callback must contain panic")?;}
        Ok(())
    })
}

trait Pixel:Copy+Default {const DEPTH:i32;fn sample(i:usize)->Self;}
impl Pixel for u8 {const DEPTH:i32=8;fn sample(i:usize)->Self {(i*31%255) as Self}}
impl Pixel for u16 {const DEPTH:i32=16;fn sample(i:usize)->Self {(i*73%32769) as Self}}
impl Pixel for f32 {
    const DEPTH:i32=32;
    fn sample(i:usize)->Self {
        match i%97 {0=>f32::NAN,1=>f32::INFINITY,2=>f32::NEG_INFINITY,3=>-0.0,
            _=>(i%73) as f32/13.0-2.0}
    }
}
fn bytes<T:Copy>(data:&[T])->&[u8] {
    // SAFETY: read-only initialized scalar storage, size includes row padding.
    unsafe {std::slice::from_raw_parts(data.as_ptr().cast(),std::mem::size_of_val(data))}
}

fn matrix<T:Pixel>() {
    const W:usize=83;const H:usize=137;const PITCH:usize=W*4+12;
    let target=[8.0,10.0,70.0,3.0,63.0,110.0,0.0,97.0];
    let source=[0.0,0.0,82.0,0.0,82.0,136.0,0.0,136.0];
    let columns=[0.0,0.17,0.67,1.0];let rows=[0.0,0.39,0.61,1.0];
    let mut input:Vec<T>=(0..PITCH*H).map(T::sample).collect();
    for kind in [RenderKind::Region,RenderKind::Layer,RenderKind::Comp,RenderKind::Perspective] {
        for quality in 0..=1 {for edge in 0..=3 {for sparse in 0..=2 {for invalid in [false,true] {
            let corners=if invalid {[0.0;8]} else {target};
            let state=State {corners:Some(corners),source_corners:Some(source),render_kind:kind,
                editable_corners:true,..State::default()};
            let mut serial=vec![T::sample(11);PITCH*H];let mut tiled=serial.clone();
            let image=|pixels:*mut T,width:i32,height:i32|Image {pixels:pixels.cast(),width,height,
                row_bytes:(PITCH*std::mem::size_of::<T>()) as isize};
            let src=match sparse {
                0=>image(input.as_mut_ptr(),W as i32,H as i32),
                1=>image(input.as_mut_ptr(),31,27),
                _=>Image {pixels:std::ptr::null_mut(),row_bytes:0,width:0,height:0},
            };
            let dst=image(serial.as_mut_ptr(),W as i32,H as i32);
            let tile_dst=image(tiled.as_mut_ptr(),W as i32,H as i32);
            let frame=Frame {corners,columns:columns.as_ptr(),rows:rows.as_ptr(),column_count:4,row_count:4,
                surface_units_x:1.0,surface_units_y:1.0,canvas_width:W as i32,canvas_height:H as i32,
                source_x:if sparse==1 {12}else{0},source_y:if sparse==1 {9}else{0},
                output_x:-17,output_y:-13,easing:0.72,easing_distance:0.42,
                abort_fn:None,abort_refcon:std::ptr::null_mut()};
            let expected=render_result(dispatch_render(&src,&dst,T::DEPTH,&frame,&mut Report::default(),
                (quality,edge),&state));
            // SAFETY: distinct owned scalar buffers/axes live through joined tasks.
            let actual=unsafe {Job::new(src,tile_dst,T::DEPTH,frame,(quality,edge),&state)}.run(||Ok(()),concurrent);
            assert_eq!(actual,expected,"depth{} kind{kind:?} q{quality} e{edge} sparse{sparse} invalid{invalid}",T::DEPTH);
            assert_eq!(bytes(&tiled),bytes(&serial),"depth{} kind{kind:?} q{quality} e{edge} sparse{sparse} invalid{invalid}",T::DEPTH);
        }}}}
    }
}

#[test] fn scheduled_rows_preserve_exact_pixels_padding_sparse_and_invalid_fallback() {
    matrix::<u8>();matrix::<u16>();matrix::<f32>();
}

#[test] fn independent_frames_can_schedule_concurrently() {
    std::thread::scope(|scope| {
        let a=scope.spawn(matrix::<u8>);let b=scope.spawn(matrix::<u16>);
        a.join().unwrap();b.join().unwrap();
    });
}

#[test] fn cancellation_and_scheduler_errors_stop_without_retry_or_worker_abort() {
    const W:usize=4;const H:usize=145;
    let mut input=vec![91u8;W*H*4];let mut output=vec![33u8;input.len()];
    let axis=[0.0,0.5,1.0];let corners=[0.0,0.0,3.0,0.0,3.0,144.0,0.0,144.0];
    let state=State {corners:Some(corners),editable_corners:true,..State::default()};
    let src=Image {pixels:input.as_mut_ptr().cast(),row_bytes:(W*4) as isize,width:W as i32,height:H as i32};
    let dst=Image {pixels:output.as_mut_ptr().cast(),..src};
    static WORKER_ABORTS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
    unsafe extern "C" fn forbidden(_: *mut std::ffi::c_void)->i32 {
        WORKER_ABORTS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);1
    }
    let frame=Frame {corners,columns:axis.as_ptr(),rows:axis.as_ptr(),column_count:3,row_count:3,
        surface_units_x:1.0,surface_units_y:1.0,canvas_width:W as i32,canvas_height:H as i32,
        source_x:0,source_y:0,output_x:0,output_y:0,easing:0.0,easing_distance:0.25,
        abort_fn:Some(forbidden),abort_refcon:std::ptr::null_mut()};
    // SAFETY: separate owned buffers and axes; bounded joined scheduling below.
    let job=unsafe {Job::new(src,dst,8,frame,(1,0),&state)};
    assert!(job.frame.abort_fn.is_none());assert!(job.frame.abort_refcon.is_null());
    let mut scheduled=0;
    assert_eq!(job.run(||Err(ae::Error::InterruptCancel),|_,_| {scheduled+=1;Ok(())}),Err(ae::Error::InterruptCancel));
    assert_eq!(scheduled,0);assert!(output.iter().all(|v|*v==33));
    let caller=std::thread::current().id();let mut polls=0;
    assert_eq!(job.run(|| {
        assert_eq!(std::thread::current().id(),caller);polls+=1;
        if polls==2 {Err(ae::Error::InterruptCancel)}else{Ok(())}
    },concurrent),Err(ae::Error::InterruptCancel));
    assert!(output[..64*W*4].iter().all(|v|*v==91));
    assert!(output[64*W*4..].iter().all(|v|*v==33));
    output.fill(33);
    assert_eq!(job.run(||Ok(()),|_,callback| {
        scheduled+=1;callback(0)?;Err(ae::Error::OutOfMemory)
    }),Err(ae::Error::OutOfMemory));
    assert_eq!(scheduled,1);assert!(output[16*W*4..].iter().all(|v|*v==33));
    assert_eq!(job.strip(0,-1,4),Err(ae::Error::BadCallbackParameter));
    assert_eq!(job.strip(0,4,4),Err(ae::Error::BadCallbackParameter));
    assert_eq!(contain(||panic!("contained test panic")),Err(ae::Error::InternalStructDamaged));
    assert_eq!(WORKER_ABORTS.load(std::sync::atomic::Ordering::Relaxed),0);
}

#[test] fn eligibility_preserves_small_separable_overlap_and_stride_validation() {
    let mut src_pixels=vec![0u8;1024*512*4];let mut dst_pixels=src_pixels.clone();
    let src=Image {pixels:src_pixels.as_mut_ptr().cast(),row_bytes:4096,width:1024,height:512};
    let mut dst=Image {pixels:dst_pixels.as_mut_ptr().cast(),..src};
    let axis=[0.0,0.5,1.0];let mut frame=Frame {corners:[0.0,0.0,1023.0,0.0,1023.0,511.0,0.0,511.0],
        columns:axis.as_ptr(),rows:axis.as_ptr(),column_count:3,row_count:3,
        surface_units_x:1.0,surface_units_y:1.0,canvas_width:1024,canvas_height:512,
        source_x:0,source_y:0,output_x:0,output_y:0,easing:0.0,easing_distance:0.25,
        abort_fn:None,abort_refcon:std::ptr::null_mut()};
    let mut state=State::default();
    assert!(!eligible(&src,&dst,8,&frame,&state)); // separable cache
    frame.corners[1]=7.0;assert!(eligible(&src,&dst,8,&frame,&state));
    assert!(!eligible(&src,&src,8,&frame,&state)); // overlap
    dst.row_bytes=7;assert!(!eligible(&src,&dst,8,&frame,&state));dst.row_bytes=4096;
    dst.height=1;assert!(!eligible(&src,&dst,8,&frame,&state));dst.height=512;
    frame.output_y=i32::MAX;assert!(!eligible(&src,&dst,8,&frame,&state));frame.output_y=0;
    assert!(!eligible(&src,&dst,24,&frame,&state));
    state.render_kind=RenderKind::Perspective;frame.corners[1]=0.0;
    assert!(eligible(&src,&dst,8,&frame,&state));
    let empty=Image {pixels:std::ptr::null_mut(),row_bytes:0,width:0,height:0};
    assert!(eligible(&empty,&dst,8,&frame,&state));
}
