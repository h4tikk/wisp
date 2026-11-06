use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wisp")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(alias = "s")]
    Search {
        query: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(alias = "inf")]
    Info {
        query: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(alias = "i")]
    Install {
        query: Vec<String>,
        #[arg(long)]
        noconfirm: bool,
    },
    #[command(alias = "r")]
    Remove {
        query: Vec<String>,
        #[arg(long)]
        noconfirm: bool,
    },
    #[command(alias = "u")]
    Upgrade,
    #[command(alias = "ic")]
    InstallConfig {
        #[arg(long)]
        dry_run: bool,
    },
    #[command(alias = "sy")]
    Sync {
        #[arg(long)]
        interactive: bool,
    },
    #[command(alias = "c")]
    Clean,
}
