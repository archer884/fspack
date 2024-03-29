use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use std::path::Path;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Layout {
    content: Vec<Content>,
}

impl Layout {
    pub fn new(content: impl IntoIterator<Item = Content>) -> Self {
        Self {
            content: content.into_iter().collect(),
        }
    }

    pub fn package_size(&self) -> u64 {
        self.content.iter().map(|x| x.size).sum()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content {
    path: String,
    size: u64,
    date: i64,
}

impl Content {
    pub fn new(base: &Path, path: &Path, meta: Metadata) -> Self {
        use std::time::SystemTime;

        let time = DateTime::from(meta.modified().unwrap_or_else(|_| SystemTime::now()));
        let path = path.strip_prefix(base).unwrap_or(path);

        Self {
            path: format_path(path),
            size: meta.len(),
            date: from_epoch(time),
        }
    }
}

// The number of ticks from the Windows filetime epoch.
// butwhy.gif
// Because man is fallen, that's why.
fn from_epoch(time: DateTime) -> i64 {
    const TICKS_PER_SECOND: i64 = 10_000_000;
    const NANOS_PER_TICK: i64 = 100;

    // You only WISH this were a constant.
    let epoch = chrono::Utc.with_ymd_and_hms(1, 1, 1, 1, 1, 1).unwrap();

    let elapsed = time.signed_duration_since(epoch);
    let a = elapsed.num_seconds() * TICKS_PER_SECOND;
    let b = elapsed.num_nanoseconds().unwrap_or(0) / NANOS_PER_TICK;
    a + b
}

fn format_path(path: &Path) -> String {
    let path = path.display().to_string();
    path.replace('\\', "/")
}
