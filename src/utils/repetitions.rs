use super::time::Time;

pub trait Repetitions: Iterator<Item = Time> {}

impl Repetitions for Box<dyn Repetitions> {}