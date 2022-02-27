use crate::{data::Data, utils::time::Time, world_model::WorldModel};



pub fn predict(data: &mut Data, time: Time) -> Option<WorldModel> {
    Some(WorldModel::new(data.current_task(time)?))
}
