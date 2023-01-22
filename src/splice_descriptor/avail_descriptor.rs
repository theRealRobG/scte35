use super::DescriptorLengthExpectation;
use crate::{bit_reader::Bits, error::ParseError};

/// The `AvailDescriptor` is an implementation of a `SpliceDescriptor`. It provides an optional
/// extension to the `SpliceInsert` command that allows an authorization identifier to be sent for
/// an avail. Multiple copies of this descriptor may be included by using the loop mechanism
/// provided. This identifier is intended to replicate the functionality of the cue tone system
/// used in analogue systems for ad insertion. This descriptor is intended only for use with a
/// `SpliceInsert` command, within a `SpliceInfoSection`.
/**
```
// avail_descriptor() {
//   splice_descriptor_tag  8 uimsbf
//   descriptor_length      8 uimsbf
//   identifier            32 uimsbf
//   provider_avail_id     32 uimsbf
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct AvailDescriptor {
    /// This 32-bit number is used to identify the owner of the descriptor. The identifier shall
    /// have a value of 0x43554549 (ASCII "CUEI").
    pub identifier: u32,
    /// This 32-bit number provides information that a receiving device may utilize to alter its
    /// behaviour during or outside of an avail. It may be used in a manner similar to analogue cue
    /// tones. An example would be a network directing an affiliate or a head-end to black out a
    /// sporting event.
    pub provider_avail_id: u32,
}

impl AvailDescriptor {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let expectation = DescriptorLengthExpectation::try_from(bits, "AvailDescriptor")?;

        let identifier = bits.u32(32);
        let provider_avail_id = bits.u32(32);

        expectation.validate_non_fatal(bits, super::SpliceDescriptorTag::AvailDescriptor);

        Ok(Self {
            identifier,
            provider_avail_id,
        })
    }
}
