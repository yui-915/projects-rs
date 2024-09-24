use crate::prelude::*;

pub struct Socket<T: Serialize + DeserializeOwned, S: Read + Write> {
    stream: S,
    _msg_type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned, S: Read + Write> Socket<T, S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            _msg_type: PhantomData,
        }
    }

    pub fn send(&mut self, msg: T) -> Result<()> {
        let serialized = bincode::serialize(&msg)?;
        let len = serialized.len() as u32;
        self.stream.write_all(&len.to_ne_bytes())?;
        self.stream.write_all(&serialized)?;
        Ok(())
    }

    pub fn recv(&mut self) -> Result<T> {
        let mut len_bytes = [0u8; 4];
        self.stream.read_exact(&mut len_bytes)?;
        let len = u32::from_ne_bytes(len_bytes);

        let mut buf = vec![0u8; len as usize];
        self.stream.read_exact(&mut buf)?;
        let msg = bincode::deserialize(&buf)?;
        Ok(msg)
    }
}
