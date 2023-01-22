use crate::{bit_reader::Bits, error::ParseError};

/// The `BreakDuration` structure specifies the duration of the commercial break(s). It may
/// be used to give the splicer an indication of when the break will be over and when the
/// network in point will occur.
/**
```
// break_duration() {
//   auto_return       1 bslbf
//   reserved          6 bslbf
//   duration         33 uimsbf
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct BreakDuration {
    /// A flag that, when set to `true`, denotes that the `duration` shall be used by the splicing
    /// device to know when the return to the network feed (end of break) is to take place. A
    /// `SpliceInsert` command with `out_of_network_indicator` set to `false` is not intended to be
    /// sent to end this break. When this flag is `false`, the `duration` field, if present, is not
    /// required to end the break because a new `SpliceInsert` command will be sent to end the
    /// break. In this case, the presence of the `BreakDuration` field acts as a safety mechanism
    /// in the event that a `SpliceInsert` command is lost at the end of a break.
    pub auto_return: bool,
    /// A 33-bit field that indicates elapsed time in terms of ticks of the program's 90 kHz clock.
    pub duration: u64,
}

impl BreakDuration {
    pub fn try_from(bits: &mut Bits) -> Result<BreakDuration, ParseError> {
        bits.validate(40, "BreakDuration")?;
        let auto_return = bits.bool();
        bits.consume(6);
        let duration = bits.u64(33);
        Ok(Self {
            auto_return,
            duration,
        })
    }
}

/// The `SpliceTime` structure, when modified by `pts_adjustment`, specifies the time of the splice
/// event.
/**
```
// splice_time() {
//   time_specified_flag             1 bslbf
//   if(time_specified_flag == 1) {
//     reserved                      6 bslbf
//     pts_time                     33 uimsbf
//   } else
//     reserved                      7 bslbf
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct SpliceTime {
    /// A 33-bit field that indicates time in terms of ticks of the program's 90 kHz clock. This
    /// field, when modified by `pts_adjustment`, represents the time of the intended splice point.
    pub pts_time: Option<u64>,
}

impl SpliceTime {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        bits.validate(1, "SpliceTime; reading timeSpecifiedFlag")?;
        let time_specified_flag = bits.bool();
        if time_specified_flag {
            bits.validate(39, "SpliceTime; timeSpecifiedFlag == 1")?;
            bits.consume(6);
            let pts_time = bits.u64(33);
            Ok(Self {
                pts_time: Some(pts_time),
            })
        } else {
            bits.validate(7, "SpliceTime; timeSpecifiedFlag == 0")?;
            bits.consume(7);
            Ok(Self { pts_time: None })
        }
    }
}
