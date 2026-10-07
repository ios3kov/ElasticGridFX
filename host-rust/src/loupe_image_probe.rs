//! Validation-only numeric census. No images, host pointers or host calls.
//! Bounded to16 local records; ordinary builds contain no journal.
use std::{cell::RefCell,io::Write};
#[derive(Default)]struct Journal{file:Option<std::fs::File>,records:usize,failed:bool}
thread_local!{static LOG:RefCell<Journal>=RefCell::new(Journal::default());}
pub(crate) fn record(kind:u8,data:&[f64]){
    LOG.with_borrow_mut(|log|{
        if log.failed||log.records>=16||data.len()>24||data.iter().any(|x|!x.is_finite()){return;}
        if log.file.is_none(){
            let nonce=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
            let path=std::env::temp_dir().join(format!("FSTR-loupe-image-{}-{nonce}.jsonl",std::process::id()));
            let mut options=std::fs::OpenOptions::new();options.create_new(true).write(true);
            #[cfg(unix)]{use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
            match options.open(path){Ok(file)=>log.file=Some(file),Err(_)=>{log.failed=true;return;}}
        }
        let row=format!("{{\"schema\":1,\"build\":\"{}\",\"kind\":{kind},\"data\":{data:?}}}\n",super::build_identity::DIAGNOSTIC.split('\r').find_map(|s|s.strip_prefix("ElasticGridBuildID=")).unwrap_or("unknown"));
        log.records+=1;
        if let Some(file)=log.file.as_mut(){if file.write_all(row.as_bytes()).is_err(){log.failed=true;}}
    });
}
