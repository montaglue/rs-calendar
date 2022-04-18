use std::collections::HashMap;

use crate::world::Time;

use self::task::Task;

pub mod task;

pub struct Data {
    id_counter: usize,
    current_flow_id: usize,
    tasks: HashMap<usize, Task>,
    flow_counter: HashMap<String, usize>,
}

impl Data {
    pub fn create_task(&mut self, name: String, description: String) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;

        let new_task = todo!();

        self.tasks.insert(id, new_task);

        id
    }

    pub fn current_task(&self, time: Time) -> Option<&Task> {
        todo!()
    }

    pub fn change_flow(&mut self, flow_name: String) {
        self.current_flow_id = todo!();
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
            current_flow_id: 0, 
            tasks: Default::default(), 
            flow_counter: Default::default(), 
        }
    }
}
