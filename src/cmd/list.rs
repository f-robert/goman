use anyhow::Result;

use crate::{
    api::list_candidates::{list_candidates, Candidate},
    cli::ListArgs,
    dirs::{current_dir_name, local_candidates},
    helpers::{get_arch, get_os},
};

pub fn list(args: &ListArgs) -> Result<()> {
    let arch = get_arch();
    let os = get_os();

    let candidates = list_candidates(args.all, os, arch)?;
    let local_candidates = local_candidates()?;
    let current_dir_name = current_dir_name()?;

    display(candidates, local_candidates, current_dir_name, arch, os);

    Ok(())
}

fn display(
    candidates: Vec<Candidate>,
    local_candidates: Vec<String>,
    current_dir_name: Option<String>,
    arch: &str,
    os: &str,
) {
    let used = match current_dir_name {
        Some(name) => name,
        None => "".to_string(),
    };

    let title = &format!("Available candidate versions for {} {}", os, arch);
    let title_len = title.len();
    let header = "=".repeat(title_len);
    let separator = "-".repeat(title_len);
    let to_console = |first: &str, second: &str, third: &str| {
        println!("{:<12} | {:<3} | {:<9}", first, second, third);
    };

    pager::Pager::new().setup();

    println!("{header}");
    println!("{title}");
    println!("{header}");
    to_console("Version", "Use", "Status");
    println!("{separator}");

    for candidate in candidates {
        let used = if candidate.version == used { "<<<" } else { "" };
        let status = if local_candidates.contains(&candidate.version) {
            "installed"
        } else {
            ""
        };

        to_console(&candidate.version, used, status);
    }
}
