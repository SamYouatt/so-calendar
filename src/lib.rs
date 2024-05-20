use args::SoCalArgs;
use clap::Parser;
use color_eyre::eyre::Result;
use configuration::Application;

use crate::features::new_account::handle_new_account::handle_new_account;

mod args;
pub mod configuration;
mod features;
mod tui;

pub async fn run(application: Application) -> Result<()> {
    tui::run_tui(application).await

    // let args = SoCalArgs::parse();

    // match args.entity {
    //     args::Entity::Account(account_command) => match account_command.command {
    //         args::AccountSubcommand::New => handle_new_account(&application)?,
    //         args::AccountSubcommand::List => handle_list_accounts(&application)?,
    //     },
    // };

    // Ok(())
}
