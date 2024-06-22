use std::{
    fs::{create_dir_all, File},
    io::BufReader,
    path::PathBuf,
};

use crate::{
    api::{download_candidate::download_candidate, list_candidates::list_candidates},
    cli::InstallArgs,
    dirs::candidates_dir,
    helpers::{get_arch, get_os},
};

use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use tar::Archive;

pub fn install(args: &InstallArgs) -> Result<()> {
    check_installed(&args.version)?;

    let arch = get_arch();
    let os = get_os();

    let candidates = list_candidates(true, os, arch)?;
    let candidate = candidates
        .iter()
        .find(|candidate| candidate.version == args.version)
        .ok_or_else(|| {
            anyhow!(format!(
                "{} is not available for {} {}",
                args.version, os, arch
            ))
        })?;

    let archive_path = download_candidate(candidate)?;
    unpack(&args.version, archive_path)?;

    Ok(())
}

fn unpack(version: &String, path: PathBuf) -> Result<()> {
    let destination = candidates_dir()?.join(version);
    let file = File::open(&path)?;
    let buf_reader = BufReader::new(&file);

    let decoder = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decoder);
    let entries = archive.entries()?;

    for entry in entries {
        let mut entry = entry?;
        let entry_path = entry.path()?;

        let mut components = entry_path.components();
        components.next();

        let relative_path: PathBuf = components.collect();
        let mut dest_path = PathBuf::from(&destination);
        dest_path.push(relative_path);

        if let Some(parent) = dest_path.parent() {
            create_dir_all(parent)?;
        }

        entry.unpack(dest_path)?;
    }

    Ok(())
}

fn check_installed(version: &String) -> Result<()> {
    let candidate = candidates_dir()?.join(version);

    if candidate.is_dir() {
        Err(anyhow!("{} is already installed", version))
    } else {
        Ok(())
    }
}
