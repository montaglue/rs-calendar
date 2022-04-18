use user_interface::{manager::UIManager, console::ConsoleUI, UI};

pub mod combinatorics_engine;
pub mod world;
pub mod data;
pub mod prediction;
pub mod user_interface;

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
