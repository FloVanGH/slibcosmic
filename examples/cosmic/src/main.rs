#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

fn main() {
    CosmicWindow::new().unwrap().run().unwrap();
}
