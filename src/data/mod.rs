use std::collections::HashMap;

use crate::utils::time::Time;

use self::{task::{Task, Stage, TaskTime}, calendar::Calendar};

pub mod task;
pub mod calendar;

pub struct Data {
    id_counter: usize,
    current_flow: String,
    calendar: Calendar,
    tasks: HashMap<usize, Task>,
    flow_counter: HashMap<String, usize>,
}

impl Data {
    pub fn create_task(&mut self, name: String, description: String) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;

        let new_task = Task {
            id,
            name,
            description,
            flow: self.current_flow.clone(),
            flow_position: *self.flow_counter.get(&self.current_flow).unwrap_or(&0),
            stage: Stage::NotStarted,
        };

        self.calendar.insert(TaskTime::None, id);

        self.tasks.insert(id, new_task);

        id
    }

    pub fn current_task(&self, time: Time) -> Option<&Task> {
        let current_tasks = Calendar::current_task(&self.calendar, time);
        for i in current_tasks {
            if let Some(task) = self.tasks.get(&i.1) {
                return Some(task);
            }
        }

        None
    }

    pub fn change_flow(&mut self, flow_name: String) {
        self.current_flow = flow_name;
    }

    pub fn change_task<F>(&mut self, id: usize, f: F)
    where
        F: FnOnce(&mut Task) -> ()  {
        self.tasks.get_mut(&id).map(f);
    }

    pub fn remove_task(&mut self, id: usize) {
        self.tasks.remove(&id);
    }
}


impl Default for Data {
    fn default() -> Self {
        Self { 
            id_counter: Default::default(), 
            current_flow: format!("other"), 
            calendar: Default::default(), 
            tasks: Default::default(), 
            flow_counter: Default::default(), 
        }
    }
}
