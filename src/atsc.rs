use crate::{bit_reader::Bits, error::ParseError};

/// The ATSC Content Identifier is a structure that is composed of a TSID and a “house number” with
/// a period of uniqueness. A “house number” is any number that the holder of the TSID wishes as
/// constrained herein. Numbers are unique for each value of TSID.
/**
```text
{
  TSID       16 uimsbf
  reserved    2 bslbf
  end_of_day  5 uimsbf
  unique_for  9 uimsbf
  content_id
}
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct ATSCContentIdentifier {
    /// This 16 bit unsigned integer field shall contain a value of `transport_stream_id` per
    /// section 6.3.1 of A/65 \[3\]. Note: The assigning authority for these values for the United
    /// States is the FCC. Ranges for Mexico, Canada, and the United States have been
    /// established by formal agreement among these countries. Values in other regions are
    /// established by appropriate authorities.
    pub tsid: u16,
    /// This 5-bit unsigned integer shall be set to the hour of the day in UTC in which the
    /// broadcast day ends and the instant after which the `content_id` values may be re-used
    /// according to `unique_for`. The value of this field shall be in the range of 0–23. The
    /// values 24–31 are reserved. Note that the value of this field is expected to be static
    /// per broadcaster.
    pub end_of_day: u8,
    /// This 9-bit unsigned integer shall be set to the number of days, rounded up, measured
    /// relative to the hour indicated by `end_of_day`, during which the `content_id` value is not
    /// reassigned to different content. The value shall be in the range 1 to 511. The value
    /// zero shall be forbidden. The value 511 shall have the special meaning of “indefinitely”.
    /// Note that the value of this field is expected to be essentially static per broadcaster,
    /// only changing when the method of house numbering is changed. Note also that decoders can
    /// treat stored `content_id` values as unique until the `unique_for` fields expire, which can
    /// be implemented by decrementing all stored `unique_for` fields by one every day at the
    /// `end_of_day` until they reach zero.
    pub unique_for: u16,
    /// This variable length field shall be set to the value of the identifier according to the
    /// house number system or systems for the value of `tsid`. Each such value shall not be
    /// assigned to different content within the period of uniqueness set by the values in the
    /// `end_of_day` and `unique_for` fields. The identifier may be any combination of human
    /// readable and/or binary values and need not exactly match the form of a house number, not
    /// to exceed 242 bytes.
    pub content_id: String,
}

impl ATSCContentIdentifier {
    pub fn try_from(bits: &mut Bits, upid_length: u8) -> Result<ATSCContentIdentifier, ParseError> {
        let content_id_length = (upid_length as isize) - 4;
        if content_id_length < 0 {
            return Err(ParseError::InvalidATSCContentIdentifierInUPID { upid_length });
        }

        let tsid = bits.u16(16);
        bits.consume(2);
        let end_of_day = bits.u8(5);
        let unique_for = bits.u16(9);
        let content_id = bits.string(
            content_id_length as usize,
            "Reading content_id for ATSCContentIdentifier",
        )?;

        Ok(Self {
            tsid,
            end_of_day,
            unique_for,
            content_id,
        })
    }
}

/// ATSC A/52 Table 5.8 Audio Coding Mode.
///
/// This 3-bit code, shown in Table 5.8, indicates which of the main service channels are in use,
/// ranging from 3/2 to 1/0. If the msb of acmod is a 1, surround channels are in use and surmixlev
/// follows in the bit stream. If the msb of acmod is a ‘0’, the surround channels are not in use
/// and surmixlev does not follow in the bit stream. If the lsb of acmod is a ‘0’, the center
/// channel is not in use. If the lsb of acmod is a ‘1’, the center channel is in use. Note: The
/// state of acmod sets the number of fullbandwidth channels parameter, nfchans, (e.g., for 3/2
/// mode, nfchans = 5; for 2/1 mode, nfchans = 3; etc.). The total number of channels, nchans, is
/// equal to nfchans if the lfe channel is off, and is equal to 1 + nfchans if the lfe channel is
/// on. If acmod is 0, then two completely independent program channels (dual mono) are encoded
/// into the bit stream, and are referenced as Ch1, Ch2. In this case, a number of additional items
/// are present in BSI or audblk to fully describe Ch2. Table 5.8 also indicates the channel
/// ordering (the order in which the channels are processed) for each of the modes.
/**
```text
acmod Audio Coding Mode nfchans Channel Array Ordering
‘000’ 1+1               2       Ch1, Ch2
‘001’ 1/0               1       C
‘010’ 2/0               2       L, R
‘011’ 3/0               3       L, C, R
‘100’ 2/1               3       L, R, S
‘101’ 3/1               4       L, C, R, S
‘110’ 2/2               4       L, R, SL, SR
‘111’ 3/2               5       L, C, R, SL, SR
```
*/
#[derive(PartialEq, Eq, Debug)]
pub enum AudioCodingMode {
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘000’ 1+1               2       Ch1, Ch2
    ```
    */
    OneAndOne,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘001’ 1/0               1       C
    ```
    */
    OneZero,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘010’ 2/0               2       L, R
    ```
    */
    TwoZero,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘011’ 3/0               3       L, C, R
    ```
    */
    ThreeZero,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘100’ 2/1               3       L, R, S
    ```
    */
    TwoOne,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘101’ 3/1               4       L, C, R, S
    ```
    */
    ThreeOne,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘110’ 2/2               4       L, R, SL, SR
    ```
    */
    TwoTwo,
    /**
    ```text
    acmod Audio Coding Mode nfchans Channel Array Ordering
    ‘111’ 3/2               5       L, C, R, SL, SR
    ```
    */
    ThreeTwo,
}

impl TryFrom<u8> for AudioCodingMode {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AudioCodingMode::OneAndOne),
            1 => Ok(AudioCodingMode::OneZero),
            2 => Ok(AudioCodingMode::TwoZero),
            3 => Ok(AudioCodingMode::ThreeZero),
            4 => Ok(AudioCodingMode::TwoOne),
            5 => Ok(AudioCodingMode::ThreeOne),
            6 => Ok(AudioCodingMode::TwoTwo),
            7 => Ok(AudioCodingMode::ThreeTwo),
            _ => Err(ParseError::UnrecognisedAudioCodingMode(value)),
        }
    }
}

impl AudioCodingMode {
    pub fn value(&self) -> u8 {
        match *self {
            AudioCodingMode::OneAndOne => 0,
            AudioCodingMode::OneZero => 1,
            AudioCodingMode::TwoZero => 2,
            AudioCodingMode::ThreeZero => 3,
            AudioCodingMode::TwoOne => 4,
            AudioCodingMode::ThreeOne => 5,
            AudioCodingMode::TwoTwo => 6,
            AudioCodingMode::ThreeTwo => 7,
        }
    }
}

/// ATSC A/52 Table 5.7 Bit Stream Mode.
///
/// This 3-bit code indicates the type of service that the bit stream conveys.
/**
```text
bsmod acmod         Type of Service
‘000’ any           main audio service: complete main (CM)
‘001’ any           main audio service: music and effects (ME)
‘010’ any           associated service: visually impaired (VI)
‘011’ any           associated service: hearing impaired (HI)
‘100’ any           associated service: dialogue (D)
‘101’ any           associated service: commentary (C)
‘110’ any           associated service: emergency (E)
‘111’ ‘001’         associated service: voice over (VO)
‘111’ ‘010’ - ‘111’ main audio service: karaoke
```
*/
#[derive(PartialEq, Eq, Debug)]
pub enum BitStreamMode {
    CompleteMain,
    MusicAndEffects,
    VisuallyImpaired,
    HearingImpaired,
    Dialogue,
    Commentary,
    Emergeny,
    VoiceOver,
    Karaoke,
}

impl BitStreamMode {
    pub fn try_from(bsmod: u8, acmod: Option<u8>) -> Result<Self, ParseError> {
        match bsmod {
            0 => Ok(Self::CompleteMain),
            1 => Ok(Self::MusicAndEffects),
            2 => Ok(Self::VisuallyImpaired),
            3 => Ok(Self::HearingImpaired),
            4 => Ok(Self::Dialogue),
            5 => Ok(Self::Commentary),
            6 => Ok(Self::Emergeny),
            7 => match acmod {
                Some(1) => Ok(Self::VoiceOver),
                Some(2..=7) => Ok(Self::Karaoke),
                _ => Err(ParseError::InvalidBitStreamMode { bsmod, acmod }),
            },
            _ => Err(ParseError::InvalidBitStreamMode { bsmod, acmod }),
        }
    }
}
