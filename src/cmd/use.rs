use anyhow::{anyhow, Result};

use crate::{
    cli::UseArgs,
    dirs::{link_current_dir, local_candidates, unlink_current_dir},
};

pub fn r#use(args: &UseArgs) -> Result<()> {
    let local_candidates = local_candidates()?;

    if !local_candidates.contains(&args.version) {
        Err(anyhow!("{} is not installed on your system", &args.version))
    } else {
        unlink_current_dir()?;
        link_current_dir(&args.version)?;

        println!("{} is the current version", &args.version);

        Ok(())
    }
}
