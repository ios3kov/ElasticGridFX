//! Optional test-only callback observations; no host API or render decisions.
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const MAX_RECORDS: u64 = 4096;
const HEADER: &str = "sequence,selector,time_numerator,time_scale,width,height,depth,path,has_output,completed,parameters_ns,input_ns,output_ns,sampling_ns,checkin_ns,total_ns,start_ns\n";
static WRITER: OnceLock<Option<Mutex<Writer>>> = OnceLock::new();
static ORIGIN: OnceLock<Instant> = OnceLock::new();

struct Writer {
    file: File,
    records: u64,
    failed: bool,
}
impl Writer {
    fn create() -> io::Result<Self> {
        let folder =
            std::env::temp_dir().join(format!("egfx-render-diagnostics-{}", std::process::id()));
        Self::create_at(&folder)
    }
    fn create_at(folder: &Path) -> io::Result<Self> {
        // Atomic create refuses existing directories/symlinks. Never reuse logs.
        DirBuilder::new().mode(0o700).create(folder)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(folder.join("render.csv"))?;
        writeln!(
            file,
            "# schema=1,build_id={}",
            super::build_identity::BUILD_ID
        )?;
        file.write_all(HEADER.as_bytes())?;
        Ok(Self {
            file,
            records: 0,
            failed: false,
        })
    }
    fn append(&mut self, record: &Record) {
        if self.failed || self.records >= MAX_RECORDS {
            return;
        }
        self.records += 1;
        if self
            .file
            .write_all(record.encode(self.records).as_bytes())
            .is_err()
        {
            self.failed = true;
        }
        // A full-cap file is conservatively rejected by observation consumers.
    }
}

#[derive(Default)]
struct Record {
    selector: &'static str,
    numerator: i32,
    scale: u32,
    width: i32,
    height: i32,
    depth: i16,
    path: &'static str,
    has_output: bool,
    completed: bool,
    endpoints: [Option<u64>; 5],
    total: u64,
    start: u64,
}
impl Record {
    fn encode(&self, sequence: u64) -> String {
        let points = self
            .endpoints
            .map(|v| v.map(|n| n.to_string()).unwrap_or_default());
        format!(
            "{sequence},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.selector,
            self.numerator,
            self.scale,
            self.width,
            self.height,
            self.depth,
            self.path,
            u8::from(self.has_output),
            u8::from(self.completed),
            points[0],
            points[1],
            points[2],
            points[3],
            points[4],
            self.total,
            self.start
        )
    }
}

pub(crate) enum Phase {
    Parameters,
    Input,
    Output,
    Sampling,
    Checkin,
}
pub(crate) struct Trace {
    started: Instant,
    record: Record,
}
impl Trace {
    pub(crate) fn new(selector: &'static str, numerator: i32, scale: u32) -> Self {
        let origin = ORIGIN.get_or_init(Instant::now);
        let started = Instant::now();
        Self {
            started,
            record: Record {
                selector,
                numerator,
                scale,
                path: "unknown",
                start: started
                    .duration_since(*origin)
                    .as_nanos()
                    .min(u64::MAX as u128) as u64,
                ..Record::default()
            },
        }
    }
    pub(crate) fn configure(&mut self, width: i32, height: i32, depth: i16, path: &'static str) {
        self.record.width = width;
        self.record.height = height;
        self.record.depth = depth;
        self.record.path = path;
    }
    fn elapsed(&self) -> u64 {
        self.started.elapsed().as_nanos().min(u64::MAX as u128) as u64
    }
    pub(crate) fn mark(&mut self, phase: Phase) {
        let index = match phase {
            Phase::Parameters => 0,
            Phase::Input => 1,
            Phase::Output => 2,
            Phase::Sampling => 3,
            Phase::Checkin => 4,
        };
        self.record.endpoints[index] = Some(self.elapsed());
    }
    pub(crate) fn output_present(&mut self) {
        self.record.has_output = true;
    }
    pub(crate) fn complete(&mut self) {
        self.record.completed = true;
    }
}
impl Drop for Trace {
    fn drop(&mut self) {
        self.record.total = self.elapsed();
        // Logging is after the observed endpoints and is deliberately not a
        // benchmark. No panic or logging error can alter the effect's result.
        if let Some(writer) = WRITER.get_or_init(|| Writer::create().ok().map(Mutex::new)) {
            if let Ok(mut writer) = writer.lock() {
                writer.append(&self.record);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    #[test]
    fn log_directory_is_private_exclusive_and_contains_pinned_identity() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let folder = std::env::temp_dir().join(format!(
            "egfx-trace-create-test-{}-{nonce}",
            std::process::id()
        ));
        let writer = Writer::create_at(&folder).unwrap();
        let path = folder.join("render.csv");
        assert_eq!(
            std::fs::metadata(&folder).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert!(
            std::fs::read_to_string(&path)
                .unwrap()
                .contains(super::super::build_identity::BUILD_ID)
        );
        assert!(Writer::create_at(&folder).is_err());
        drop(writer);
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }
    #[test]
    fn records_keep_missing_phases_and_unsuccessful_output_distinct() {
        let record = Record {
            selector: "smart",
            numerator: 3,
            scale: 30,
            width: 1920,
            height: 1080,
            depth: 32,
            path: "plane_region",
            endpoints: [Some(2), Some(5), None, None, None],
            total: 7,
            ..Record::default()
        };
        let fields: Vec<_> = record
            .encode(1)
            .trim_end()
            .split(',')
            .map(str::to_owned)
            .collect();
        assert_eq!(fields.len(), 17);
        assert_eq!(fields[8], "0");
        assert_eq!(fields[9], "0");
        assert_eq!(&fields[10..], &["2", "5", "", "", "", "7", "0"]);
    }
    #[test]
    fn output_and_success_are_explicit() {
        let record = Record {
            selector: "smart",
            path: "plane_layer",
            has_output: true,
            completed: true,
            total: 9,
            ..Record::default()
        };
        assert!(
            record
                .encode(4096)
                .starts_with("4096,smart,0,0,0,0,0,plane_layer,1,1,")
        );
        assert_eq!(HEADER.trim_end().split(',').count(), 17);
    }
    #[test]
    fn writer_stops_at_cap_without_creating_another_file() {
        // Exercise bounded append on an exclusive disposable file, no AE calls.
        let folder =
            std::env::temp_dir().join(format!("egfx-trace-cap-test-{}", std::process::id()));
        std::fs::create_dir(&folder).unwrap();
        let path = folder.join("cap.csv");
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let mut writer = Writer {
            file,
            records: MAX_RECORDS - 1,
            failed: false,
        };
        writer.append(&Record::default());
        let size = std::fs::metadata(&path).unwrap().len();
        writer.append(&Record::default());
        assert_eq!(std::fs::metadata(&path).unwrap().len(), size);
        assert_eq!(writer.records, MAX_RECORDS);
        drop(writer);
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }
}
