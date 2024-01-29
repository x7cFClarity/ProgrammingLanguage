//! This module is used for storing key value pairs of objects and retrieving them later.

pub mod z0;

pub trait DataMap<Record> {
    type Key;

    fn get_record(&mut self, label: Self::Key) -> &Option<Record>;
    fn set_record(&mut self, label: Self::Key, record: Record);
}