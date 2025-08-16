use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

pub fn to_bytes<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    Ok(bincode::serialize(v)?)
}

pub fn from_bytes<T: DeserializeOwned>(buf: &[u8]) -> Result<T> {
    Ok(bincode::deserialize(buf)?)
}
