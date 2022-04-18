use crate::{data::task::{Task, Stage}, combinatorics_engine::bounds::TimeBound};

use super::{Time, state::WorldState};

pub struct WorldModel {
    pub(super) tasks: Vec<Task>,
}

impl WorldModel {
    pub fn predict(&self, moment: &Time) -> WorldState {
        let mut open_tasks: Vec<_> = self
            .tasks
            .iter()
            .filter(|task| {
                if task.stage == Stage::Planed || task.stage == Stage::InProgres {
                    for bound in task.bounds.time.iter() {
                        let ok = match bound {
                            TimeBound::Begin(begin) => begin <= moment,
                            TimeBound::BeginEnd(begin, end) 
                                => begin <= moment && moment <= end,
                            TimeBound::Deadline(deadline) => moment < deadline,
                            TimeBound::Day(day) => day.date() == moment.date(),
                            TimeBound::Duration(_) => true,
                        };
                        if !ok {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            })
            .collect();

        open_tasks.sort_by(|a, b| a.importance.cmp(&b.importance));

        open_tasks.pop().map(Task::state).unwrap_or_default()
    }
}
