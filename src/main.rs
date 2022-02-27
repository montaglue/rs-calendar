use user_interface::{manager::UIManager, console::ConsoleUI, UI};

pub mod world_model;
pub mod data;
pub mod prediction;
pub mod user_interface;
pub mod utils;

fn main() {
    let mut ui_manager = Box::new(UIManager::default());
    ui_manager.push(Box::new(ConsoleUI::default()));
    match ui_manager.run() {
        Ok(_) => unreachable!(),
        Err(err) => {
            println!("{:?}", err);
        }
    };
}
