use crate::utils::{time::{Time, Day}, repetitions::Repetitions};

pub enum Stage {
    NotStarted,
    Planed,
    InProgres,
    Completed,
}

#[derive(Clone)]
pub enum TaskTime {
    Complete {
        begin: Time,
        duration: Time,
    },
    Begin(Time),
    Day(Day),
    None,
}

pub struct Task {
    pub(super) id: usize,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) flow: String,
    pub(super) flow_position: usize,
    pub(crate) stage: Stage,
}

impl Task {
    pub fn name(&self) -> &String {
        &self.name
    }
}
