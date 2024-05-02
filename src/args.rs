use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SoCalArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// Manage connected accounts
    Account(AccountCommand),
}

#[derive(Debug, Args)]
pub struct AccountCommand {
    #[clap(subcommand)]
    pub command: AccountSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AccountSubcommand {
    /// Link a new calendar account
    New,
}
