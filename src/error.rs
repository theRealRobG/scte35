use crate::{
    hex::DecodeHexError,
    splice_command::SpliceCommandType,
    splice_descriptor::{segmentation_descriptor::SegmentationUPIDType, SpliceDescriptorTag},
};
use std::{
    fmt::{Display, Formatter},
    str::Utf8Error,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEndOfData {
        /// The expected minimum number of bits left in the data.
        expected_minimum_bits_left: u32,
        /// The actual number of bits left in the data.
        actual_bits_left: u32,
        /// A description of what was being attempted to be parsed that resulted in error.
        description: &'static str,
    },
    DecodeHexError(DecodeHexError),
    InvalidSectionSyntaxIndicator,
    InvalidPrivateIndicator,
    UnrecognisedSpliceCommandType(u8),
    UnrecognisedSegmentationUPIDType(u8),
    UnexpectedSegmentationUPIDLength {
        /// This is the number of bytes that the UPID was expected to have as declared via
        /// `segmentation_upid_length`.
        declared_segmentation_upid_length: u8,
        /// This is the number of bytes that the UPID was expected to have as defined by the
        /// specification for the given UPID type.
        expected_segmentation_upid_length: u8,
        /// This is the type of the UPID that failed to parse properly.
        segmentation_upid_type: SegmentationUPIDType,
    },
    InvalidUUIDInSegmentationUPID(&'static str),
    InvalidURLInSegmentationUPID(&'static str),
    UnrecognisedSegmentationTypeID(u8),
    InvalidSegmentationDescriptorIdentifier(u32),
    InvalidATSCContentIdentifierInUPID {
        upid_length: u8,
    },
    InvalidMPUInSegmentationUPID {
        upid_length: u8,
    },
    InvalidBitStreamMode {
        bsmod: u8,
        acmod: Option<u8>,
    },
    UnrecognisedAudioCodingMode(u8),
    UnrecognisedSpliceDescriptorTag(u8),
    EncryptedMessageNotSupported,
    UnexpectedSpliceCommandLength {
        /// This is the number of bits that the SpliceCommand was expected to have as declared via
        /// `splice_command_length`.
        declared_splice_command_length_in_bits: u32,
        /// This is the number of bits that the SpliceCommand actually had after parsing had
        /// completed.
        actual_splice_command_length_in_bits: usize,
        /// The type of the splice command
        splice_command_type: SpliceCommandType,
    },
    UnexpectedDescriptorLoopLength {
        /// This is the number of bits that the `SpliceDescriptor` array was expected to have as
        /// declared via `descriptor_loop_length`.
        declared_descriptor_loop_length_in_bits: u32,
        /// This is the number of bits that the `SpliceDescriptor` array actually had after parsing
        /// had completed.
        actual_descriptor_loop_length_in_bits: u32,
    },
    UnexpectedSpliceDescriptorLength {
        /// This is the number of bits that the `SpliceDescriptor` was expected to have as declared
        /// via `descriptor_length`.
        declared_splice_descriptor_length_in_bits: u32,
        /// This is the number of bits that the `SpliceDescriptor` actually had after parsing had
        /// completed.
        actual_splice_descriptor_length_in_bits: usize,
        /// The tag for the splice descriptor.
        splice_descriptor_tag: SpliceDescriptorTag,
    },
    Utf8ConversionError {
        error: Utf8Error,
        description: &'static str,
    },
}

impl From<DecodeHexError> for ParseError {
    fn from(e: DecodeHexError) -> Self {
        ParseError::DecodeHexError(e)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
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
            ParseError::DecodeHexError(e) => e.fmt(f),
            ParseError::InvalidSectionSyntaxIndicator => {
                "The 1-bit section syntax indicator was not 0.".fmt(f)
            }
            ParseError::InvalidPrivateIndicator => "The 1-bit private indicator was not 0.".fmt(f),
            ParseError::UnrecognisedSpliceCommandType(t) => {
                write!(f, "Value {} was obtained for splice command type and this does not match any known values.", t)
            }
            ParseError::UnrecognisedSegmentationUPIDType(t) => {
                write!(f, "Value {} was obtained for segmentation upid type and this does not match any known values.", t)
            }
            ParseError::UnexpectedSegmentationUPIDLength {
                declared_segmentation_upid_length,
                expected_segmentation_upid_length,
                segmentation_upid_type,
            } => {
                write!(
                    f,
                    "Declared upid length was {}; however, expected length for upid type {} is {}.",
                    declared_segmentation_upid_length,
                    segmentation_upid_type.value(),
                    expected_segmentation_upid_length
                )
            }
            ParseError::InvalidUUIDInSegmentationUPID(id) => {
                write!(f, "{} is not a valid UUID.", id)
            }
            ParseError::InvalidURLInSegmentationUPID(id) => write!(f, "{} is not a valid URL.", id),
            ParseError::UnrecognisedSegmentationTypeID(t) => {
                write!(f, "Value {} was obtained for segmentation type id and this does not match any known values.", t)
            }
            ParseError::InvalidSegmentationDescriptorIdentifier(v) => {
                write!(f, "Value {} was obtained for segmentation descriptor identifier but this should be 0x43554549.", v)
            }
            ParseError::InvalidATSCContentIdentifierInUPID { upid_length } => {
                write!(
                    f,
                    "UPID length defined as {}, and {} bytes are taken up by static fields, implying content_id has {} bytes left, which is invalid.",
                    upid_length,
                    STATIC_BYTES_LENGTH,
                    calculated_byte_count(*upid_length)
                )
            }
            ParseError::InvalidMPUInSegmentationUPID { upid_length } => {
                write!(
                    f,
                    "UPID length defined as {}, and {} bytes are taken up by static fields, implying private data has {} bytes left, which is invalid.",
                    upid_length,
                    STATIC_BYTES_LENGTH,
                    calculated_byte_count(*upid_length)
                )
            }
            ParseError::InvalidBitStreamMode { bsmod, acmod } => {
                write!(
                    f,
                    "Value {} was obtained for bit stream mode, and {} was obtained for audio coding mode, but this combination is not a valid BitStreamMode.",
                    bsmod,
                    acmod.map(|s| format!("{}", s)).unwrap_or_else(|| String::from("None"))
                )
            }
            ParseError::UnrecognisedAudioCodingMode(t) => {
                write!(f, "Value {} was obtained for audio coding mode and this does not match any known values.", t)
            }
            ParseError::UnrecognisedSpliceDescriptorTag(t) => {
                write!(f, "Value {} was obtained for splice descriptor tag and this does not match any known values.", t)
            }
            ParseError::EncryptedMessageNotSupported => {
                "The SpliceInfoSection was determined to be encrypted and this is not currently supported".fmt(f)
            }
            ParseError::UnexpectedSpliceCommandLength {
                declared_splice_command_length_in_bits,
                actual_splice_command_length_in_bits,
                splice_command_type,
            } => {
                write!(
                    f,
                    "Declared splice command ({}) length was {} bits; however, number of bits needed to parse the section was {}.",
                    splice_command_type.value(),
                    declared_splice_command_length_in_bits,
                    actual_splice_command_length_in_bits
                )
            }
            ParseError::UnexpectedDescriptorLoopLength {
                declared_descriptor_loop_length_in_bits,
                actual_descriptor_loop_length_in_bits,
            } => {
                write!(
                    f,
                    "Declared descriptor loop length was {} bits; however, number of bits needed to parse the array was {}.",
                    declared_descriptor_loop_length_in_bits,
                    actual_descriptor_loop_length_in_bits
                )
            }
            ParseError::UnexpectedSpliceDescriptorLength {
                declared_splice_descriptor_length_in_bits,
                actual_splice_descriptor_length_in_bits,
                splice_descriptor_tag,
            } => {
                write!(
                    f,
                    "Declared splice descriptor ({}) length was {} bits; however, number of bits needed to parse the descriptor was {}.",
                    splice_descriptor_tag.value(),
                    declared_splice_descriptor_length_in_bits,
                    actual_splice_descriptor_length_in_bits
                )
            }
            ParseError::Utf8ConversionError { error, description } => {
                write!(f, "Utf8Error: {} - {}", error, description)
            }
        }
    }
}

impl std::error::Error for ParseError {}

const STATIC_BYTES_LENGTH: isize = 4;

fn calculated_byte_count(upid_length: u8) -> isize {
    (upid_length as isize) - STATIC_BYTES_LENGTH
}
