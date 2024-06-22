pub mod api;
pub mod cli;
pub mod cmd;

pub mod dirs {
    use std::{
        env,
        fs::{create_dir, read_dir, read_link, symlink_metadata},
        path::PathBuf,
    };

    use anyhow::{anyhow, Result};
    use dirs::home_dir;
    use symlink::{remove_symlink_dir, symlink_dir};

    const GOMAN_DEFAULT_DIR: &str = ".goman";
    const GOMAN_HOME_ENV: &str = "goman";

    const CANDIDATES_DIR: &str = "candidates";
    const CURRENT_DIR: &str = "current";
    const TEMP_DIR: &str = "tmp";

    pub fn candidates_dir() -> Result<PathBuf> {
        let candidates_dir = goman_dir()?.join(CANDIDATES_DIR);

        if !candidates_dir.exists() {
            create_dir(&candidates_dir)?;
        }

        Ok(candidates_dir)
    }

    pub fn local_candidates() -> Result<Vec<String>> {
        let local_candidates = read_dir(candidates_dir()?)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let metadata = entry.metadata().ok()?;

                if metadata.is_dir() {
                    entry.file_name().into_string().ok()
                } else {
                    None
                }
            })
            .collect();

        Ok(local_candidates)
    }

    pub fn temp_dir() -> Result<PathBuf> {
        let temp_dir = goman_dir()?.join(TEMP_DIR);

        if !temp_dir.exists() {
            create_dir(&temp_dir)?;
        }

        Ok(temp_dir)
    }

    pub fn link_current_dir(version: &String) -> Result<()> {
        let src = candidates_dir()?.join(version);
        let dest = current_dir()?;

        symlink_dir(&src, &dest)?;

        Ok(())
    }

    pub fn unlink_current_dir() -> Result<()> {
        let current_dir = current_dir()?;

        if current_dir.exists() {
            remove_symlink_dir(&current_dir)?;
        }

        Ok(())
    }

    pub fn current_dir() -> Result<PathBuf> {
        let current_dir = goman_dir()?.join(CURRENT_DIR);

        Ok(current_dir)
    }

    pub fn current_dir_name() -> Result<Option<String>> {
        let current_dir = goman_dir()?.join(CURRENT_DIR);

        if let Ok(metadata) = symlink_metadata(&current_dir) {
            if metadata.file_type().is_symlink() {
                if let Ok(target_path) = read_link(&current_dir) {
                    if target_path.is_dir() {
                        let name = target_path
                            .file_name()
                            .and_then(|name| name.to_str().map(|s| s.to_string()))
                            .ok_or_else(|| anyhow!("Failed to get current directory name"))?;

                        return Ok(Some(name));
                    }
                }
            }
        }

        Ok(None)
    }

    fn goman_dir() -> Result<PathBuf> {
        let goman_dir = match env::var(GOMAN_HOME_ENV) {
            Ok(path) => PathBuf::from(path),
            Err(_) => default_goman_dir()?,
        };

        if !goman_dir.exists() {
            create_dir(&goman_dir)?;
        }

        Ok(goman_dir)
    }

    fn default_goman_dir() -> Result<PathBuf> {
        let home_dir = home_dir().ok_or_else(|| anyhow!("Could not get home directory"))?;

        Ok(home_dir.join(GOMAN_DEFAULT_DIR))
    }
}

pub mod helpers {
    use std::env::consts;

    pub fn get_arch<'a>() -> &'a str {
        match consts::ARCH {
            "x86" => "386",
            "x86_64" => "amd64",
            "arm" => "armv6l",
            "aarch64" => "arm64",
            _ => "unknown",
        }
    }

    pub fn get_os<'a>() -> &'a str {
        consts::OS
    }
}
