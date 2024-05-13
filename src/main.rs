use color_eyre::eyre::Result;
use socal::{configuration::Application, run};

fn main() -> Result<()> {
    color_eyre::install()?;

    let application = Application::setup()?;

    run(application)?;

    Ok(())
}
