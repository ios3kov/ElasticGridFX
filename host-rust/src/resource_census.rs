//! Nondefault, passive census. Workers only update scalar atomics. UI writes a
//! bounded private record; no host request, quality change or retained pointer.
//! World bytes describe borrowed storage, not AE's allocation/cache lifetime.
use super::ae;
use std::sync::atomic::{AtomicU64,Ordering};
use std::cell::RefCell;
use std::io::Write;
use std::time::{Instant,SystemTime,UNIX_EPOCH};

#[derive(Clone,Copy,Debug)]
pub(crate) enum Kind {Snapshot,Loupe,Input,Output,UiBitmap,DrawImage}
struct Counter {created:AtomicU64,dropped:AtomicU64,live:AtomicU64,peak:AtomicU64,largest:AtomicU64,dimensions:[AtomicU64;3]}
impl Counter {
    const fn new()->Self {Self{created:AtomicU64::new(0),dropped:AtomicU64::new(0),
        live:AtomicU64::new(0),peak:AtomicU64::new(0),largest:AtomicU64::new(0),dimensions:[const{AtomicU64::new(0)};3]}}
    fn start(&self,bytes:u64) {
        self.created.fetch_add(1,Ordering::Relaxed);
        self.largest.fetch_max(bytes,Ordering::Relaxed);
        let live=self.live.fetch_add(bytes,Ordering::Relaxed)+bytes;
        self.peak.fetch_max(live,Ordering::Relaxed);
    }
    fn finish(&self,bytes:u64) {self.live.fetch_sub(bytes,Ordering::Relaxed);self.dropped.fetch_add(1,Ordering::Relaxed);}
    fn read(&self)->[u64;8] {[&self.created,&self.dropped,&self.live,&self.peak,&self.largest,
        &self.dimensions[0],&self.dimensions[1],&self.dimensions[2]].map(|v|v.load(Ordering::Relaxed))}
}
static COUNTS:[Counter;6]=[const{Counter::new()};6];
// Pixel checkin ok/error, loupe checkin ok/error, async polls/nonnull receipts.
static CHECKINS:[AtomicU64;6]=[const{AtomicU64::new(0)};6];
// Attempted async calls: warm / active gesture. Preparation errors include
// failures before a host call and must not be mislabeled checkout failures.
static REQUESTS:[AtomicU64;3]=[const{AtomicU64::new(0)};3];
pub(crate) fn request(gesture:bool) {REQUESTS[usize::from(gesture)].fetch_add(1,Ordering::Relaxed);}
pub(crate) fn preparation_error() {REQUESTS[2].fetch_add(1,Ordering::Relaxed);}
#[derive(Debug)]
pub(crate) struct Token {kind:Kind,bytes:u64}
impl Token {
    pub(crate) fn new(kind:Kind,bytes:u64)->Self {COUNTS[kind as usize].start(bytes);Self{kind,bytes}}
}
impl Clone for Token {fn clone(&self)->Self {Self::new(self.kind,self.bytes)}}
impl Drop for Token {fn drop(&mut self){COUNTS[self.kind as usize].finish(self.bytes);}}
fn storage_bytes(stride:i64,height:u64)->Option<u64> {
    u64::try_from(stride).ok()?.checked_mul(height)
}
pub(crate) fn world(kind:Kind,layer:&ae::Layer)->Token {
    for (slot,value) in COUNTS[kind as usize].dimensions.iter().zip([layer.width() as u64,layer.height() as u64,layer.bit_depth() as u64]) {slot.store(value,Ordering::Relaxed);}
    Token::new(kind,storage_bytes(layer.row_bytes() as i64,layer.height() as u64).unwrap_or(0))
}
pub(crate) fn checkin(loupe:bool,success:bool) {
    CHECKINS[usize::from(loupe)*2+usize::from(!success)].fetch_add(1,Ordering::Relaxed);
}
pub(crate) fn receipt(nonnull:bool) {
    CHECKINS[4].fetch_add(1,Ordering::Relaxed);
    if nonnull {CHECKINS[5].fetch_add(1,Ordering::Relaxed);}
}
type Sample=([[u64;8];6],[u64;6],[u64;3]);
#[derive(Default)]
struct Ui {file:Option<std::fs::File>,last:Option<Instant>,previous:Option<Sample>,records:usize,failed:bool}
thread_local!{static UI:RefCell<Ui>=RefCell::new(Ui::default());}
pub(crate) fn record(force:bool) {
    UI.with(|ui|{
        let mut ui=ui.borrow_mut();
        if ui.failed||ui.records>=1024||(!force&&ui.last.is_some_and(|t|t.elapsed().as_millis()<1000)){return;}
        ui.last=Some(Instant::now());
        let counts=COUNTS.each_ref().map(Counter::read);
        let checkins=CHECKINS.each_ref().map(|v|v.load(Ordering::Relaxed));
        let requests=REQUESTS.each_ref().map(|v|v.load(Ordering::Relaxed));
        if !force&&ui.previous==Some((counts,checkins,requests)){return;}
        ui.previous=Some((counts,checkins,requests));
        if ui.file.is_none(){
            let nonce=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
            let path=std::env::temp_dir().join(format!("FSTR-resources-{}-{nonce}.jsonl",std::process::id()));
            let mut options=std::fs::OpenOptions::new();options.create_new(true).write(true);
            #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
            match options.open(path){Ok(f)=>ui.file=Some(f),Err(_)=>{ui.failed=true;return;}}
        }
        let build=super::build_identity::DIAGNOSTIC.split('\r').find_map(|s|s.strip_prefix("ElasticGridBuildID=")).unwrap_or("unknown");
        let now=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        let row=format!("{{\"schema\":2,\"build\":\"{build}\",\"pid\":{},\"unix_ms\":{now},\"sequence\":{},\"resources\":{counts:?},\"checkins\":{checkins:?},\"requests\":{requests:?}}}\n",std::process::id(),ui.records);
        ui.records+=1;
        if let Some(f)=ui.file.as_mut(){if f.write_all(row.as_bytes()).is_err(){ui.failed=true;}}
    });
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn concurrent_start_finish_and_high_water_are_bounded() {
        let count=Counter::new();
        std::thread::scope(|s|{for _ in 0..4 {let count=&count;s.spawn(move||{
            for _ in 0..10000 {count.start(4096);count.finish(4096);}
        });}});
        let [created,dropped,live,peak,largest,_,_,_]=count.read();
        assert_eq!((created,dropped,live,largest),(40000,40000,0,4096));
        assert!((4096..=4*4096).contains(&peak));
    }
    #[test]fn transient_ui_bitmap_tokens_balance_across_clone_and_early_error() {
        let count=&COUNTS[Kind::UiBitmap as usize];
        let before=count.read();
        let fail=||->Result<(),()> {
            let token=Token::new(Kind::UiBitmap,129*129*4);
            let _copy=token.clone();
            Err(())
        };
        assert!(fail().is_err());
        let after=count.read();
        assert_eq!(after[0]-before[0],2);
        assert_eq!(after[1]-before[1],2);
        assert_eq!(after[2],before[2]);
    }
    #[test]fn invalid_storage_sizes_are_not_cast_to_huge_allocations() {
        assert_eq!(storage_bytes(1920*16,1080),Some(33_177_600));
        assert_eq!(storage_bytes(-1,1080),None);
        assert_eq!(storage_bytes(i64::MAX,u64::MAX),None);
        assert_eq!(storage_bytes(0,0),Some(0));
    }
}
