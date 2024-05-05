use std::path::PathBuf;

use socal::{configuration::Application, run};

fn main() {
    let data_dir: PathBuf = match std::env::var("OVERRIDE_DATA_DIR").ok() {
        Some(override_dir) => override_dir.into(),
        None => dirs_next::data_dir()
            .expect("Unable to find data directory")
            .join("so-calendar"),
    };

    let application = Application::setup(data_dir).unwrap();

    run(application);
}
