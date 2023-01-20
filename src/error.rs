use bitter::{BigEndianReader, BitReader};

use crate::{atsc::InvalidATSCContentIdentifierInUPIDInfo, hex::DecodeHexError};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    DecodeHexError(DecodeHexError),
    UnexpectedEndOfData {
        /// The expected minimum number of bits left in the data.
        expected_minimum_bits_left: u32,
        /// The actual number of bits left in the data.
        actual_bits_left: u32,
        /// A description of what was being attempted to be parsed that resulted in error.
        description: &'static str,
    },
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
            ParseError::UnexpectedEndOfData {
                expected_minimum_bits_left,
                actual_bits_left,
                description,
            } => {
                write!(
                    f,
                    "Expected at least {} bits left and instead was {} when parsing: {}.",
                    expected_minimum_bits_left, actual_bits_left, description
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

pub fn validate(
    bit_reader: &mut BigEndianReader,
    expected_minimum_bits_left: u32,
    description: &'static str,
) -> Result<(), ParseError> {
    let actual_bits_left = bit_reader.refill_lookahead();
    if actual_bits_left < expected_minimum_bits_left {
        Err(ParseError::UnexpectedEndOfData {
            expected_minimum_bits_left,
            actual_bits_left,
            description,
        })
    } else {
        Ok(())
    }
}
