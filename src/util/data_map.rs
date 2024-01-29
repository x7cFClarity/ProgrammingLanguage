pub trait RecordGetterByString<Record> {
    type Key;

    fn get_record(&mut self, label: Self::Key) -> &Option<Record>;
    fn set_record(&mut self, label: Self::Key, record: Record);
}