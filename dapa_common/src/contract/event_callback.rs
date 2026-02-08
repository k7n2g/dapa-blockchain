use crate::serializer::*;

// Represents an event callback registration
// chunk_id identifies which function chunk to call on the listener contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventCallbackRegistration {
    // Chunk ID to invoke on the listener contract
    pub chunk_id: u16,
    // max_gas is the maximum gas that can be used for this callback
    // it is already paid/reserved at the time of registration
    pub max_gas: u64,
}

impl Serializer for EventCallbackRegistration {
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.chunk_id);
        writer.write_u64(self.max_gas);
    }

    fn read(reader: &mut Reader) -> Result<Self, ReaderError> {
        let chunk_id = reader.read_u16()?;
        let max_gas = reader.read_u64()?;
        Ok(EventCallbackRegistration { chunk_id, max_gas })
    }

    fn size(&self) -> usize {
        10 // u16 + u64
    }
}