use std::{fs, path::PathBuf};

use args::SoCalArgs;
use clap::Parser;
use features::new_account::handle_new_account;

mod args;
mod features;

pub struct Application {
    data_dir: PathBuf,
    db_path: PathBuf,
}

impl Application {
    fn new() -> Self {
        let data_dir = dirs_next::data_dir()
            .expect("Unable to find data directory")
            .join("so-calendar");

        let db_path = data_dir.join("app.sqlite");

        Self { data_dir, db_path }
    }

    fn setup(&self) {
        fs::create_dir_all(&self.data_dir).expect("Failed to create data directory");
    }
}

fn main() {
    let application = Application::new();
    application.setup();

    let args = SoCalArgs::parse();

    match args.entity {
        args::Entity::Account(_) => handle_new_account(&application),
    }
}
