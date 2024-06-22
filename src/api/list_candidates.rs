use anyhow::Result;
use serde::Deserialize;

use crate::api::API_BASE_URL;

const API_LIST_QUERY: &str = "?mode=json&include=all";
const ARCHIVE_KIND: &str = "archive";

#[derive(Debug)]
pub struct Candidate {
    pub version: String,
    pub filename: String,
    pub size: u32,
    pub sha256: String,
}

pub fn list_candidates(all: bool, os: &str, arch: &str) -> Result<Vec<Candidate>> {
    let url = &format!("{}/{}", API_BASE_URL, API_LIST_QUERY);
    let response = reqwest::blocking::get(url)?;
    let releases = response.json::<Vec<Release>>()?;

    let candidates = releases
        .into_iter()
        .filter(|r| all || r.stable)
        .flat_map(|r| r.files.into_iter())
        .filter(|f| f.kind == ARCHIVE_KIND && f.os == os && f.arch == arch)
        .map(|f| Candidate::from(f))
        .collect();

    Ok(candidates)
}

#[derive(Debug, Deserialize)]
struct Release {
    stable: bool,
    files: Vec<File>,
}

#[derive(Debug, Deserialize)]
struct File {
    version: String,
    filename: String,
    os: String,
    arch: String,
    size: u32,
    sha256: String,
    kind: String,
}

impl From<File> for Candidate {
    fn from(file: File) -> Self {
        Candidate {
            version: file.version,
            filename: file.filename,
            size: file.size,
            sha256: file.sha256,
        }
    }
}
