use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use std::path::PathBuf;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Layout {
    content: Vec<Content>,
}

impl Layout {
    pub fn set_content(&mut self, files: impl IntoIterator<Item = Content>) -> u64 {
        let mut len = 0;
        self.content = files.into_iter().inspect(|x| len += x.size).collect();
        len
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content {
    path: PathBuf,
    size: u64,
    date: i64,
}

impl Content {
    pub fn new(path: impl Into<PathBuf>, meta: Metadata) -> Self {
        use std::time::SystemTime;

        let time = DateTime::from(meta.modified().unwrap_or_else(|_| SystemTime::now()));
        Self {
            path: path.into(),
            size: meta.len(),
            date: from_epoch(time),
        }
    }
}

// The number of seconds from the Windows filetime epoch.
// butwhy.gif
// Because man is fallen, that's why.
fn from_epoch(time: DateTime) -> i64 {
    const TICKS_PER_SECOND: i64 = 10_000_000;
    const NANOS_PER_TICK: i64 = 100;

    // You only WISH this could be a constant.
    let epoch = chrono::Utc.ymd(1601, 1, 1).and_hms(0, 0, 0);

    let elapsed = time.signed_duration_since(epoch);
    let a = elapsed.num_seconds() as i64 * TICKS_PER_SECOND;
    let b = elapsed.num_nanoseconds().unwrap_or(0) as i64 / NANOS_PER_TICK;
    a + b
}
