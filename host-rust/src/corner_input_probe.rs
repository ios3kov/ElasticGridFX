//! Test-only input-route census. UI thread only; no host calls, frame requests,
//! retained pointers, project coordinates or worker logging. At most16 writes.
use std::cell::RefCell;
use std::io::Write;
use std::time::{SystemTime,UNIX_EPOCH};

pub(crate) enum Route {CustomClick,CustomDrag,NativeSupervision,
    #[cfg(feature="frozen-corner-probe")]Tentative,
    #[cfg(feature="frozen-corner-probe")]FinalCommit}
#[derive(Default)]
struct Journal {file:Option<std::fs::File>,counts:[u64;5],records:usize,failed:bool}
thread_local!{static UI:RefCell<Journal>=RefCell::new(Journal::default());}
pub(crate) fn record(route:Route,last:bool) {
    UI.with(|journal|{
        let mut journal=journal.borrow_mut();
        let index=route as usize;
        journal.counts[index]=journal.counts[index].saturating_add(1);
        // Retain the first native edit after scripted fixture initialization;
        // the old first-only policy could hide precisely that second change.
        let early_native=index==2&&journal.counts[index]<=3;
        if journal.failed || journal.records>=16 || (journal.counts[index]>1 && !last&&!early_native){return;}
        if journal.file.is_none(){
            let nonce=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
            let path=std::env::temp_dir().join(format!("FSTR-corner-route-{}-{nonce}.jsonl",std::process::id()));
            let mut options=std::fs::OpenOptions::new();options.create_new(true).write(true);
            #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
            match options.open(path){Ok(file)=>journal.file=Some(file),Err(_)=>{journal.failed=true;return;}}
        }
        let build=super::build_identity::DIAGNOSTIC.split('\r').find_map(|s|s.strip_prefix("ElasticGridBuildID=")).unwrap_or("unknown");
        let now=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        let row=format!("{{\"schema\":1,\"build\":\"{build}\",\"pid\":{},\"unix_ms\":{now},\"route\":{index},\"last\":{last},\"counts\":{:?}}}\n",std::process::id(),journal.counts);
        journal.records+=1;
        if let Some(file)=journal.file.as_mut(){if file.write_all(row.as_bytes()).is_err(){journal.failed=true;}}
    });
}
