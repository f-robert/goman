use clap::{Parser, Subcommand};

use goman::{
    cli::{InstallArgs, ListArgs, UninstallArgs, UseArgs},
    cmd::{install::install, list::list, r#use::r#use, uninstall::uninstall},
};

fn main() {
    let cli = CLI::parse();

    let result = match cli.cmd {
        Commands::Install(args) => install(&args),
        Commands::List(args) => list(&args),
        Commands::Uninstall(args) => uninstall(&args),
        Commands::Use(args) => r#use(&args),
    };

    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    };
}

#[derive(Debug, Parser)]
/// A Version Manager for GO
struct CLI {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install(InstallArgs),
    List(ListArgs),
    Uninstall(UninstallArgs),
    Use(UseArgs),
}
