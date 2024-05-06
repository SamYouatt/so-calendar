use socal::{configuration::Application, run};

fn main() {
    let application = Application::setup().unwrap();

    run(application);
}
