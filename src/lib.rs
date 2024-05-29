use color_eyre::eyre::Result;
use configuration::Application;

mod args;
pub mod configuration;
mod features;
mod tui;

pub async fn run(application: Application) -> Result<()> {
    tui::run_tui(application).await
}
