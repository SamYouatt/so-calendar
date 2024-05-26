use std::{io::stdout, panic};

use color_eyre::eyre::Result;
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen}, ExecutableCommand};
use socal::{configuration::Application, run};

// Ensure terminal is reset properly upon panic
fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    install_panic_hook();
    color_eyre::install()?;

    let application = Application::setup().await?;

    run(application).await?;

    Ok(())
}
