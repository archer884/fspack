use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    dependencies: Vec<Dependency>,
    content_type: String,
    title: String,
    manufacturer: String,
    creator: String,
    package_version: String,
    minimum_game_version: String,
    release_notes: Option<Notes>,
    total_package_size: String,
}

impl Manifest {
    pub fn set_total_package_size(&mut self, size: u64) {
        self.total_package_size = format!("{:020}", size);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    name: String,
    package_version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Notes {
    neutral: Option<Neutral>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Neutral {
    #[serde(rename = "LastUpdate")]
    last_update: String,
    #[serde(rename = "OlderHistory")]
    older_history: String,
}
