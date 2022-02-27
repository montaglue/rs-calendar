use std::error::Error;

use super::UI;

#[derive(Default)]
pub struct UIManager {
    interfaces: Vec<Box<dyn UI>>,
}

impl UIManager {
    pub fn push(&mut self, ui: Box<dyn UI>) {
        self.interfaces.push(ui);
    }    
}


impl UI for UIManager {
    fn run(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        for ui in self.interfaces {
            ui.run()?;
        }
        loop {}
    }
}
