use bitter::{BigEndianReader, BitReader};

use crate::{atsc::InvalidATSCContentIdentifierInUPIDInfo, hex::DecodeHexError};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    DecodeHexError(DecodeHexError),
    UnexpectedEndOfData(UnexpectedEndOfDataErrorInfo),
    InvalidATSCContentIdentifierInUPID(InvalidATSCContentIdentifierInUPIDInfo),
}

impl From<DecodeHexError> for ParseError {
    fn from(e: DecodeHexError) -> Self {
        ParseError::DecodeHexError(e)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ParseError::DecodeHexError(e) => e.fmt(f),
            ParseError::UnexpectedEndOfData(i) => {
                write!(
                    f,
                    "Expected at least {} bits left and instead was {} when parsing: {}.",
                    i.expected_minimum_bits_left, i.actual_bits_left, i.description
                )
            }
            ParseError::InvalidATSCContentIdentifierInUPID(i) => {
                write!(
                    f,
                    "UPID length defined as {}, and {} bytes are taken up by static fields, implying content_id has {} bytes left, which is invalid.",
                    i.upid_length,
                    i.static_bytes_length(),
                    i.calculated_content_id_byte_count()
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnexpectedEndOfDataErrorInfo {
    /// The expected minimum number of bits left in the data.
    pub expected_minimum_bits_left: usize,
    /// The actual number of bits left in the data.
    pub actual_bits_left: usize,
    /// A description of what was being attempted to be parsed that resulted in error.
    pub description: &'static str,
}

pub fn validate(
    bit_reader: &BigEndianReader,
    expected_minimum_bits_left: usize,
    description: &'static str,
) -> Result<(), ParseError> {
    match bit_reader.has_bits_remaining(expected_minimum_bits_left) {
        true => Ok(()),
        false => Err(ParseError::UnexpectedEndOfData(
            UnexpectedEndOfDataErrorInfo {
                expected_minimum_bits_left,
                actual_bits_left: bit_reader.bits_remaining().unwrap_or(0),
                description,
            },
        )),
    }
}
