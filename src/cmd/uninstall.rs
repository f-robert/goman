use std::fs::remove_dir_all;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::{
    cli::UninstallArgs,
    dirs::{candidates_dir, current_dir_name, unlink_current_dir},
};

pub fn uninstall(args: &UninstallArgs) -> Result<()> {
    let candidate_path = candidates_dir()?.join(&args.version);

    check_installed(&args.version, &candidate_path)?;

    if current_dir_name()?
        .filter(|version| version == &args.version)
        .is_some()
    {
        unlink_current_dir()?;
    }


    remove_dir_all(&candidate_path)?;
    println!("Uninstalled {}", &args.version);

    Ok(())
}

fn check_installed(version: &String, candidate_path: &PathBuf) -> Result<()> {
    if !candidate_path.is_dir() {
        Err(anyhow!("{} is not installed on your system", version))
    } else {
        Ok(())
    }
}
