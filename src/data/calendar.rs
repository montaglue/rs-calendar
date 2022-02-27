use std::collections::hash_map::Entry;
use std::collections::{HashMap, BTreeMap, HashSet};
use std::ops::Bound::{Included, Excluded};

use crate::utils::time::{Time, Day};

use super::task::TaskTime;

#[derive(Default)]
pub struct Calendar {
    not_dated: HashSet<usize>,
    days: HashMap<Day, HashSet<usize>>,
    time: BTreeMap<Time, usize>,
    times: HashMap<usize, TaskTime>,
}

impl Calendar {
    pub fn current_task(&self, time: Time) -> impl Iterator<Item = (Time, usize)> {
        let mut iter_time = self.time
            .range((Included(&time.clone().day_begin()), Excluded(&time.clone().next_day())))
            .map(|(time, id)| {
                (time.clone(), *id)
            })
            .collect::<Vec<_>>()
            .into_iter();

        if let Some((day, vec)) = self.days.get_key_value(&time.day()) {
            iter_time = iter_time
                .chain(vec.iter().map(|x| (day.time().clone(), *x)))
                .collect::<Vec<_>>()
                .into_iter();
        }

        iter_time
    }

    pub fn time(&self, id: usize) -> Option<&TaskTime> {
        self.times.get(&id)
    }

    pub fn not_dated(&self) -> &HashSet<usize> {
        &self.not_dated
    }

    pub fn insert(&mut self, time: TaskTime, id: usize) {
        self.times.insert(id, time.clone());

        match time {
            TaskTime::Complete { begin, duration } => {
                self.time.insert(begin.clone(), id);
                self.time.insert(begin.add(duration), id);
            },
            TaskTime::Begin(time) => {
                self.time.insert(time, id);
            }
            TaskTime::Day(day) => {
                match self.days.entry(day) {
                    Entry::Occupied(mut v) => {
                        v.get_mut().insert(id);
                    }
                    Entry::Vacant(v) => {
                        v.insert({
                            let mut res = HashSet::new();
                            res.insert(id);
                            res
                        });
                    }
                }
            }
            TaskTime::None => {
                self.not_dated.insert(id);
            }
        }
    }

    pub fn remove(&mut self, id: usize) -> Option<TaskTime> {
        let result = self.times.remove(&id);
        if let Some(time) =  result.clone() {
            match time {
                TaskTime::Complete { begin, duration } => {
                    self.time.remove(&begin);
                    self.time.remove(&begin.add(duration));
                }
                TaskTime::Begin(time) => {
                    self.time.remove(&time);
                }
                TaskTime::Day(day) => {
                    self.days.remove(&day);
                },
                TaskTime::None => {
                    self.not_dated.remove(&id);
                }
            }
        }

        result
    }
}
