use args::SoCalArgs;
use clap::Parser;
use features::new_account::handle_new_account;

mod args;
mod features;

fn main() {
    let args = SoCalArgs::parse();

    match args.entity {
        args::Entity::Account(_) => handle_new_account(),
    }
}
