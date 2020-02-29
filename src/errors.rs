use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Base24Error {
    #[error("Input data length must be a multiple of 4 bytes (32 bits)")]
    EncodeInputLengthInvalid,
    #[error("Input data length must be a multiple of 7 chars")]
    DecodeInputLengthInvalid,
    #[error("Unsupported character in input: {0:?}")]
    DecodeUnsupportedCharacter(char),
}
