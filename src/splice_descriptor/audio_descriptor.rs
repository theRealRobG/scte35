use super::DescriptorLengthExpectation;
use crate::{
    atsc::{AudioCodingMode, BitStreamMode},
    bit_reader::Bits,
    error::ParseError,
};

/// The `AudioDescriptor` should be used when programmers and/or MVPDs do not support dynamic
/// signaling (e.g., signaling of audio language changes) and with legacy audio formats that do not
/// support dynamic signaling. As discussed in Section 9.1.5 of the SCTE Operational Practice on
/// Multiple Audio Signaling [SCTE 248], since most MVPD head-ends do not change the PAT/PMT to
/// signal changed audio streams, this descriptor in SCTE 35 should be used to signal such changes.
/// This descriptor is an implementation of a `SpliceDescriptor`. It provides the ability to
/// dynamically signal the audios actually in use in the stream. This descriptor shall only be used
/// with a `TimeSignal` command and a segmentation descriptor with the type `program_start` or
/// `program_overlap_start`.
/**
```
// {
//   splice_descriptor_tag            8 uimsbf
//   descriptor_length                8 uimsbf
//   identifier                      32 uimsbf
//   audio_count                      4 uimsbf
//   reserved                         4 bslbf
//   for (i=0; i<audio_count; i++) {
//     component_tag                  8 uimsbf
//     ISO_code                      24 uimsbf
//     Bit_Stream_Mode                3 uimsbf
//     Num_Channels                   4 uimsbf
//     Full_Srvc_Audio                1 uimsbf
//   }
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct AudioDescriptor {
    /// This 32-bit number is used to identify the owner of the descriptor. The identifier shall
    /// have a value of 0x43554549 (ASCII “CUEI”).
    pub identifier: u32,
    /// The audio PIDs in the program.
    pub components: Vec<Component>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Component {
    /// An optional 8-bit value that identifies the elementary PID stream containing the audio
    /// channel that follows. If used, the value shall be the same as the value used in the
    /// `stream_identifier_descriptor()` to identify that elementary PID stream. If this is not
    /// used, the value shall be 0xFF and the stream order shall be inferred from the PMT audio
    /// order.
    pub component_tag: u8,
    /// This field is a 3-byte language code defining the language of this audio service which
    /// shall correspond to a registered language code contained in the Code column of the
    /// [ISO 639-2] registry.
    pub iso_code: u32,
    /// This is a 3-bit field that is set to the same value as the bsmod field in the AC-3
    /// elementary stream.
    pub bit_stream_mode: BitStreamMode,
    /// This is a 4-bit field that indicates the number of channels in the AC-3 elementary stream.
    /// When the MSB is 0, the lower 3 bits are set to the same value as the acmod field in the
    /// AC-3 elementary stream. When the MSB field is 1, the lower 3 bits indicate the maximum
    /// number of encoded audio channels (counting the lfe channel as 1).
    pub num_channels: NumChannels,
    /// This is a 1-bit field that indicates if this audio service is a full service suitable for
    /// presentation, or a partial service which should be combined with another audio service
    /// before presentation. This bit should be set to `true` if this audio service is sufficiently
    /// complete to be presented to the listener without being combined with another audio service
    /// (for example, a visually impaired service which contains all elements of the Program;
    /// music, effects, dialogue, and the visual content descriptive narrative). This bit should be
    /// set to `false` if the service is not sufficiently complete to be presented without being
    /// combined with another audio service (e.g., a visually impaired service which only contains
    /// a narrative description of the visual Program content and which needs to be combined with
    /// another audio service which contains music, effects, and dialogue).”
    pub full_srvc_audio: bool,
}

/// This is a 4-bit field that indicates the number of channels in the AC-3 elementary stream. When
/// the MSB is 0, the lower 3 bits are set to the same value as the acmod field in the AC-3
/// elementary stream. When the MSB field is 1, the lower 3 bits indicate the maximum number of
/// encoded audio channels (counting the lfe channel as 1).
#[derive(PartialEq, Eq, Debug)]
pub enum NumChannels {
    AudioCodingMode(AudioCodingMode),
    MaxNumberOfEncodedChannels(MaxNumberOfEncodedChannels),
}

/// Indicates the maximum number of encoded audio channels (counting the lfe channel as 1).
#[derive(PartialEq, Eq, Debug)]
pub enum MaxNumberOfEncodedChannels {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Unknown(u8),
}
impl MaxNumberOfEncodedChannels {
    fn new(value: u8) -> Self {
        match value {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            x => Self::Unknown(x),
        }
    }
}

impl AudioDescriptor {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let expectation = DescriptorLengthExpectation::try_from(bits, "AudioDescriptor")?;

        let identifier = bits.u32(32);
        let audio_count = bits.u8(4);
        bits.consume(4);
        let mut components = vec![];
        for _ in 0..audio_count {
            components.push(Component::try_from(bits)?);
        }

        expectation.validate_non_fatal(bits, super::SpliceDescriptorTag::AudioDescriptor);

        Ok(Self {
            identifier,
            components,
        })
    }
}

impl Component {
    fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let component_tag = bits.byte();
        let iso_code = bits.u32(24);
        let bsmod = bits.u8(3);
        if bits.bool() {
            let acmod = bits.u8(3);
            let audio_coding_mode = AudioCodingMode::try_from(acmod)?;
            let bit_stream_mode = BitStreamMode::try_from(bsmod, Some(acmod))?;
            let num_channels = NumChannels::AudioCodingMode(audio_coding_mode);
            let full_srvc_audio = bits.bool();
            Ok(Self {
                component_tag,
                iso_code,
                bit_stream_mode,
                num_channels,
                full_srvc_audio,
            })
        } else {
            let max_number_of_encoded_channels = MaxNumberOfEncodedChannels::new(bits.u8(3));
            let bit_stream_mode = BitStreamMode::try_from(bsmod, None)?;
            let num_channels =
                NumChannels::MaxNumberOfEncodedChannels(max_number_of_encoded_channels);
            let full_srvc_audio = bits.bool();
            Ok(Self {
                component_tag,
                iso_code,
                bit_stream_mode,
                num_channels,
                full_srvc_audio,
            })
        }
    }
}
