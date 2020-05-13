#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    OutOfMIDIRange,
    OutOfInstrumentRange,
}
