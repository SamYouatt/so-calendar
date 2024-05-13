use color_eyre::eyre::Result;
use args::SoCalArgs;
use clap::Parser;
use configuration::Application;

use crate::features::list_accounts::handle_list_accounts;
use crate::features::new_account::handle_new_account::handle_new_account;

mod args;
mod features;
pub mod configuration;

pub fn run(application: Application) -> Result<()> {
    let args = SoCalArgs::parse();

    match args.entity {
        args::Entity::Account(account_command) => {
            match account_command.command {
                args::AccountSubcommand::New => handle_new_account(&application)?,
                args::AccountSubcommand::List => handle_list_accounts(&application)?,
            }
        },
    };

    Ok(())
}
