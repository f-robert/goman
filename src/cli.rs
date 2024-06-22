use clap::Args;

#[derive(Debug, Args)]
/// Install a candidate version
pub struct InstallArgs {
    /// The candidate version
    pub version: String,
}

#[derive(Debug, Args)]
/// List available candidate versions
pub struct ListArgs {
    #[clap(long)]
    /// Return all version, including unstable ones
    pub all: bool,
}

#[derive(Debug, Args)]
/// Uninstall a candidate version
pub struct UninstallArgs {
    /// The candidate version
    pub version: String,
}

#[derive(Debug, Args)]
/// Use a candidate version
pub struct UseArgs {
    /// The candidate version
    pub version: String,
}
