use color_eyre::eyre::Result;
use configuration::Application;

mod args;
pub mod configuration;
pub mod domain;
pub mod features;
pub mod tui;
pub mod util;

pub async fn run(application: Application) -> Result<()> {
    tui::run_tui(application).await
}
