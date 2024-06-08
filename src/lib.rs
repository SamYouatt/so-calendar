use color_eyre::eyre::Result;
use configuration::Application;

mod args;
pub mod configuration;
pub mod features;
pub mod tui;
pub mod domain;

pub async fn run(application: Application) -> Result<()> {
    tui::run_tui(application).await
}
