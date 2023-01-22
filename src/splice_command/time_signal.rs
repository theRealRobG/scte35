use crate::{bit_reader::Bits, error::ParseError, time::SpliceTime};

/// The `TimeSignal` provides a time synchronized data delivery mechanism. The syntax of the
/// `TimeSignal` allows for the synchronization of the information carried in this message with the
/// system time clock (STC). The unique payload of the message is carried in the descriptor,
/// however the syntax and transport capabilities afforded to `SpliceInsert` messages are also
/// afforded to the `TimeSignal`. The carriage however can be in a different PID than that carrying
/// the other cue messages used for signalling splice points.
///
/// If there is no `pts_time` in the message, then the command shall be interpreted as an immediate
/// command. It must be understood that using it in this manner will cause an unspecified amount of
/// accuracy error.
/**
```
// time_signal() {
//   splice_time()
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct TimeSignal {
    /// The `SpliceTime` structure, when modified by `pts_adjustment`, specifies the time of the
    /// splice event.
    pub splice_time: SpliceTime,
}
impl TimeSignal {
    pub fn is_immediate(&self) -> bool {
        self.splice_time.pts_time == None
    }
}

impl TimeSignal {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        Ok(Self {
            splice_time: SpliceTime::try_from(bits)?,
        })
    }
}
