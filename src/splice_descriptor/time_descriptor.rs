use super::DescriptorLengthExpectation;
use crate::{bit_reader::Bits, error::ParseError};

/// The `TimeDescriptor` is an implementation of a `SpliceDescriptor`. It provides an optional
/// extension to the `SpliceInsert`, `SpliceNull` and `TimeSignal` commands that allows a
/// programmer’s wall clock time to be sent to a client. For the highest accuracy, this descriptor
/// should be used with a `TimeSignal` or `SpliceInsert` command that has a `pts_time` defined. The
/// repetition rate of this descriptor should be at least once every 5 seconds. When it is the only
/// descriptor present in the `TimeSignal` or `SpliceNull` command, then the encoder should not
/// insert a key frame.
///
/// This command may be used to synchronize time based external metadata with video and the party
/// responsible for the metadata and the time value used should ensure that they are properly
/// synchronized and have the desired level of accuracy required for their application.
///
/// The `TimeDescriptor` uses the time format defined for the Precision Time Protocol [PTP]. [PTP]
/// is based upon an international time scale called International Atomic Time (TAI), unlike NTP
/// [RFC5905] which is based upon UTC. [PTP] is being used in a/v bridging and broadcast
/// synchronization protocols and likely to be available in a studio environment. Other time
/// sources, such as NTP or GPS, are readily convertible to PTP format.
///
/// TAI does not have "leap" seconds like UTC. When UTC was introduced (January 1, 1972) it was
/// determined there should be a difference of 10 seconds between the two time scales. Since then
/// an additional 27 leap seconds (including one in December 2016) have been added to UTC to put
/// the current difference between the two timescales at 37 seconds (as of June 2018) The [PTP]
/// protocol communicates the current offset between TAI and UTC to enable conversion. By default
/// [PTP] uses the same "epoch" (i.e. origination or reference start time and date of the
/// timescale) as Unix time, of 00:00, January 1, 1970. Readers are advised to consult IERS
/// Bulletin C for the current value of leap seconds
/// [https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html].
/**
```
// {
//   splice_descriptor_tag  8 uimsbf
//   descriptor_length      8 uimsbf
//   identifier            32 uimsbf
//   TAI_seconds           48 uimsbf
//   TAI_ns                32 uimsbf
//   UTC_offset            16 uimsbf
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct TimeDescriptor {
    /// This 32-bit number is used to identify the owner of the descriptor. The identifier shall
    /// have a value of 0x43554549 (ASCII “CUEI”).
    pub identifier: u32,
    /// This 48-bit number is the TAI seconds value.
    pub tai_seconds: u64,
    /// This 32-bit number is the TAI nanoseconds value.
    pub tai_ns: u32,
    /// This 16-bit number shall be used in the conversion from TAI time to UTC or NTP time per the
    /// following equations.
    /// ```
    /// // UTC seconds = TAI seconds - UTC_offset
    /// // NTP seconds = TAI seconds - UTC_offset + 2,208,988,800
    /// ```
    pub utc_offset: u16,
}

impl TimeDescriptor {
    // NOTE: It is assumed that the splice_descriptor_tag has already been read.
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let expectation = DescriptorLengthExpectation::try_from(bits, "TimeDescriptor")?;

        let identifier = bits.u32(32);
        let tai_seconds = bits.u64(48);
        let tai_ns = bits.u32(32);
        let utc_offset = bits.u16(16);

        expectation.validate_non_fatal(bits, super::SpliceDescriptorTag::TimeDescriptor);

        Ok(Self {
            identifier,
            tai_seconds,
            tai_ns,
            utc_offset,
        })
    }
}
