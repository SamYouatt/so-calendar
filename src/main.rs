use std::panic;

use color_eyre::eyre::Result;
use socal::{configuration::Application, run, tui::restore_terminal};

// Ensure terminal is reset properly upon panic
fn install_panic_hook() -> Result<()> {
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal().expect("Failed to restore terminal. Run `reset` or restart terminal.");
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        restore_terminal().expect("Failed to restore terminal. Run `reset` or restart terminal.");
        eyre_hook(error)
    }))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    install_panic_hook()?;

    let application = Application::setup().await?;

    run(application).await?;

    restore_terminal()?;

    Ok(())
}
