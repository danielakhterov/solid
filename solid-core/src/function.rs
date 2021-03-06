use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
    Error,
};
use std::{
    borrow::Cow,
    convert::{
        TryFrom,
        TryInto,
    },
};

/// Simple wrapper for the Solidity type `function`
pub struct Function(pub [u8; 32]);

impl TryFrom<&str> for Function {
    type Error = Error;

    fn try_from(value: &str) -> Result<Function, Self::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match value.len() {
            48 => value,
            50 => value.split_at(2).1,
            _ => value,
        };

        let slice = hex::decode(&s)?;
        let slice: [u8; 24] = slice.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[0..24].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryFrom<&[u8]> for Function {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = value.try_into()?;
        let mut buf = [0u8; 32];
        buf[0..24].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryFrom<&Vec<u8>> for Function {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = value.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[0..24].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryFrom<Vec<u8>> for Function {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = value.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[0..24].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl Encode for Function {
    fn encode(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl<'a> Decode<'a> for Function {
    fn decode(buf: &'a [u8]) -> Self {
        Function(buf[0..32].try_into().unwrap())
    }
}

impl IntoType for Function {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("function")
    }
}
