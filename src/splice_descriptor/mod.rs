use self::{
    audio_descriptor::AudioDescriptor, avail_descriptor::AvailDescriptor,
    dtmf_descriptor::DTMFDescriptor, segmentation_descriptor::SegmentationDescriptor,
    time_descriptor::TimeDescriptor,
};
use crate::{bit_reader::Bits, error::ParseError};

pub mod audio_descriptor;
pub mod avail_descriptor;
pub mod dtmf_descriptor;
pub mod segmentation_descriptor;
pub mod time_descriptor;

/// The `SpliceDescriptor` is a prototype for adding new fields to the `SpliceInfoSection`. All
/// descriptors included use the same syntax for the first six bytes. In order to allow private
/// information to be added we have included the `identifier` code. This removes the need for a
/// registration descriptor in the descriptor loop.
///
/// Splice descriptors may exist in the `SpliceInfoSection` for extensions specific to the various
/// commands.
///
/// **Implementers note:** Multiple descriptors of the same or different types in a single command
/// are allowed and may be common. The only limit on the number of descriptors is the
/// `section_length` in `SpliceInfoSection`, although there may be other practical or
/// implementation limits.
/**
```text
splice_descriptor() {
  splice_descriptor_tag  8 uimsbf
  descriptor_length      8 uimsbf
  identifier            32 uimsbf
  for(i=0; i<N; i++) {
    private_byte         8 uimsbf
  }
}
```
*/
#[derive(PartialEq, Eq, Debug)]
pub enum SpliceDescriptor {
    /// The `AvailDescriptor` provides an optional extension to the `SpliceInsert` command that
    /// allows an authorization identifier to be sent for an avail. Multiple copies of this
    /// descriptor may be included by using the loop mechanism provided. This identifier is
    /// intended to replicate the functionality of the cue tone system used in analogue systems for
    /// ad insertion. This descriptor is intended only for use with a `SpliceInsert` command,
    /// within a `SpliceInfoSection`.
    AvailDescriptor(AvailDescriptor),
    /// The `DTMFDescriptor` provides an optional extension to the `SpliceInsert` command that
    /// allows a receiver device to generate a legacy analogue DTMF sequence based on a
    /// `SpliceInfoSection` being received.
    DTMFDescriptor(DTMFDescriptor),
    /// The `segmentation_descriptor` provides an optional extension to the `TimeSignal` and
    /// `SpliceInsert` commands that allows for segmentation messages to be sent in a time/video
    /// accurate method. This descriptor shall only be used with the `TimeSignal`, `SpliceInsert`
    /// and the `SpliceNull` commands. The `TimeSignal` or `SpliceInsert` message should be sent at
    /// least once a minimum of 4 seconds in advance of the signaled `SpliceTime` to permit the
    /// insertion device to place the `SpliceInfoSection` accurately. Devices that do not recognize
    /// a value in any field shall ignore the message and take no action.
    SegmentationDescriptor(SegmentationDescriptor),
    /// The `TimeDescriptor` provides an optional extension to the `SpliceInsert`, `SpliceNull` and
    /// `TimeSignal` commands that allows a programmer’s wall clock time to be sent to a client.
    /// For the highest accuracy, this descriptor should be used with a `TimeSignal` or
    /// `SpliceInsert` command that has a `pts_time` defined.
    TimeDescriptor(TimeDescriptor),
    /// The `AudioDescriptor` should be used when programmers and/or MVPDs do not support dynamic
    /// signaling (e.g., signaling of audio language changes) and with legacy audio formats that do
    /// not support dynamic signaling. As discussed in Section 9.1.5 of the SCTE Operational
    /// Practice on Multiple Audio Signaling [SCTE 248], since most MVPD head-ends do not change
    /// the PAT/PMT to signal changed audio streams, this descriptor in SCTE 35 should be used to
    /// signal such changes. This descriptor is an implementation of a `SpliceDescriptor`. It
    /// provides the ability to dynamically signal the audios actually in use in the stream. This
    /// descriptor shall only be used with a `TimeSignal` command and a segmentation descriptor
    /// with the type `program_start` or `program_overlap_start`.
    AudioDescriptor(AudioDescriptor),
}
impl SpliceDescriptor {
    /// This 8 bit number defines the syntax for the private bytes that make up the body of this
    /// descriptor. The descriptor tags are defined by the owner of the descriptor, as registered
    /// using the identifier.
    pub fn tag(&self) -> SpliceDescriptorTag {
        match self {
            Self::AvailDescriptor(_) => SpliceDescriptorTag::AvailDescriptor,
            Self::DTMFDescriptor(_) => SpliceDescriptorTag::DTMFDescriptor,
            Self::SegmentationDescriptor(_) => SpliceDescriptorTag::SegmentationDescriptor,
            Self::TimeDescriptor(_) => SpliceDescriptorTag::TimeDescriptor,
            Self::AudioDescriptor(_) => SpliceDescriptorTag::AudioDescriptor,
        }
    }

    /// This 32 bit number is used to identify the owner of the descriptor.
    ///
    /// The identifier is a 32-bit field as defined in [ITU-T H.222.0]. Refer to clauses 2.6.8 and
    /// 2.6.9 of [ITU-T H.222.0] for a description of registration descriptor and the semantic
    /// definition of fields in the registration descriptor. Only identifier values registered and
    /// recognized by SMPTE registration authority, LLC should be used. Its use in this descriptor
    /// shall scope and identify only the private information contained within this descriptor. The
    /// code 0x43554549 (ASCII "CUEI") for descriptors defined in this specification has been
    /// registered with SMPTE.
    pub fn identifier(&self) -> u32 {
        match self {
            Self::AvailDescriptor(descriptor) => descriptor.identifier,
            Self::DTMFDescriptor(descriptor) => descriptor.identifier,
            Self::SegmentationDescriptor(descriptor) => descriptor.identifier,
            Self::TimeDescriptor(descriptor) => descriptor.identifier,
            Self::AudioDescriptor(descriptor) => descriptor.identifier,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SpliceDescriptorTag {
    AvailDescriptor,
    DTMFDescriptor,
    SegmentationDescriptor,
    TimeDescriptor,
    AudioDescriptor,
}

impl TryFrom<u8> for SpliceDescriptorTag {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SpliceDescriptorTag::AvailDescriptor),
            0x01 => Ok(SpliceDescriptorTag::DTMFDescriptor),
            0x02 => Ok(SpliceDescriptorTag::SegmentationDescriptor),
            0x03 => Ok(SpliceDescriptorTag::TimeDescriptor),
            0x04 => Ok(SpliceDescriptorTag::AudioDescriptor),
            _ => Err(ParseError::UnrecognisedSpliceDescriptorTag(value)),
        }
    }
}

impl SpliceDescriptorTag {
    pub fn value(&self) -> u8 {
        match *self {
            SpliceDescriptorTag::AvailDescriptor => 0x00,
            SpliceDescriptorTag::DTMFDescriptor => 0x01,
            SpliceDescriptorTag::SegmentationDescriptor => 0x02,
            SpliceDescriptorTag::TimeDescriptor => 0x03,
            SpliceDescriptorTag::AudioDescriptor => 0x04,
        }
    }
}

struct DescriptorLengthExpectation {
    descriptor_bits_length: u32,
    bits_remaining_before_descriptor: isize,
    expected_bits_remaining_after_descriptor: isize,
}

impl DescriptorLengthExpectation {
    fn try_from(bits: &mut Bits, validation_description: &'static str) -> Result<Self, ParseError> {
        let descriptor_bits_length = bits.u32(8) * 8;
        bits.validate(descriptor_bits_length, validation_description)?;
        let bits_remaining_before_descriptor = bits.bits_remaining() as isize;
        let expected_bits_remaining_after_descriptor =
            bits_remaining_before_descriptor - (descriptor_bits_length as isize);

        Ok(Self {
            descriptor_bits_length,
            bits_remaining_before_descriptor,
            expected_bits_remaining_after_descriptor,
        })
    }

    fn validate_non_fatal(&self, bits: &mut Bits, splice_descriptor_tag: SpliceDescriptorTag) {
        let bits_remaining = bits.bits_remaining();
        if self.expected_bits_remaining_after_descriptor != (bits_remaining as isize) {
            bits.push_non_fatal_error(ParseError::UnexpectedSpliceDescriptorLength {
                declared_splice_descriptor_length_in_bits: self.descriptor_bits_length,
                actual_splice_descriptor_length_in_bits: (self.bits_remaining_before_descriptor
                    as usize)
                    - bits_remaining,
                splice_descriptor_tag,
            })
        }
    }
}

pub fn try_splice_descriptors_from(
    bits: &mut Bits,
    descriptor_loop_length: u32,
) -> Result<Vec<SpliceDescriptor>, ParseError> {
    let mut splice_descriptors = vec![];
    bits.validate(descriptor_loop_length * 8, "SpliceDescriptor; reading loop")?;
    let bits_remaining_before_loop = bits.bits_remaining();
    let expected_end = bits_remaining_before_loop - ((descriptor_loop_length as usize) * 8);
    while bits.bits_remaining() > expected_end {
        splice_descriptors.push(SpliceDescriptor::try_from(bits)?);
    }
    Ok(splice_descriptors)
}

impl SpliceDescriptor {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        match SpliceDescriptorTag::try_from(bits.byte())? {
            SpliceDescriptorTag::AvailDescriptor => {
                Ok(Self::AvailDescriptor(AvailDescriptor::try_from(bits)?))
            }
            SpliceDescriptorTag::DTMFDescriptor => {
                Ok(Self::DTMFDescriptor(DTMFDescriptor::try_from(bits)?))
            }
            SpliceDescriptorTag::SegmentationDescriptor => Ok(Self::SegmentationDescriptor(
                SegmentationDescriptor::try_from(bits)?,
            )),
            SpliceDescriptorTag::TimeDescriptor => {
                Ok(Self::TimeDescriptor(TimeDescriptor::try_from(bits)?))
            }
            SpliceDescriptorTag::AudioDescriptor => {
                Ok(Self::AudioDescriptor(AudioDescriptor::try_from(bits)?))
            }
        }
    }
}
