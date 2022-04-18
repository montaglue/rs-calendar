use chrono::{DateTime, Utc};

use crate::{data::Data, world::state::WorldState};

pub fn predict(data: &mut Data, time: DateTime<Utc>) -> Option<WorldState> {
    todo!()
}
