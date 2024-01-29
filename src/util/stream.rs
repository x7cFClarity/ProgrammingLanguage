pub trait ReadStream<Part> {
    // Read the currently queued part.
    fn read_next(&mut self) -> Part;
}

pub trait WriteStream<Part> {
    // Write to the queue.
    fn write_next(&mut self, part: Part);
}