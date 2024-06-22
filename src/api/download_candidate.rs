use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs::{remove_file, File};
use std::io::{ Read, Write};
use std::path::PathBuf;

use crate::{
    api::API_BASE_URL, dirs::temp_dir
};

use super::list_candidates::Candidate;

pub fn download_candidate(candidate: &Candidate) -> Result<PathBuf> {
    println!("Downloading {}...", &candidate.version);
    println!();

    let url = &format!("{}/{}", API_BASE_URL, candidate.filename);
    let mut response = reqwest::blocking::get(url)?;
    let total_size = response
        .content_length()
        .context("Failed to get content length")?;

    let dest_path = temp_dir()?.join(&candidate.filename);

    if dest_path.exists() {
        remove_file(&dest_path)?;
    }

    let mut dest_file = File::create(&dest_path)?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )?   
            .progress_chars("#>-")
    );

    let mut downloaded = 0;
    let mut buffer = [0; 4096];
    let mut hasher = Sha256::new();

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }

        dest_file.write_all(&buffer[..n])?;
        hasher.update(&buffer[..n]);

        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    println!();
    println!("Download complete");

    let hash_result = hasher.finalize();
    let hash_result = hex::encode(hash_result);

    if hash_result != candidate.sha256 {
        return Err(anyhow!(
            "Invalid SHA-256 signature {}, expected was {}",
            hash_result, candidate.sha256
        ));
    }

    Ok(dest_path)
}
