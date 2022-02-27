use std::fmt::Display;

use crate::data::task::Task;

pub struct WorldModel {
    process: String,
}

impl WorldModel {
    pub fn new(task: &Task) -> WorldModel {
        WorldModel {
            process: task.name().clone()
        }
    } 
}

impl Display for WorldModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Now I should do: {}", &self.process)?;
        Ok(())
    }
}