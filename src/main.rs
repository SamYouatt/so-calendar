use socal::{configuration::Application, run};

fn main() {
    let application = Application::new();
    // TODO: handle this error
    let _ = application.setup();

    run(application);
}
