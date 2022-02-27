use std::error::Error;

pub mod console;
pub mod manager;

pub trait UI {
    fn run(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}
