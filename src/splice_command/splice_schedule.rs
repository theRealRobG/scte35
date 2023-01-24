use crate::{bit_reader::Bits, error::ParseError, time::BreakDuration};

/// The `SpliceSchedule` command is provided to allow a schedule of splice events to be conveyed
/// in advance.
/**
```text
splice_schedule() {
  splice_count                                   8 uimsbf
  for (i=0; i<splice_count; i++) {
    splice_event_id                             32 uimsbf
    splice_event_cancel_indicator                1 bslbf
    reserved                                     7 bslbf
    if (splice_event_cancel_indicator == '0') {
      out_of_network_indicator                   1 bslbf
      program_splice_flag                        1 bslbf
      duration_flag                              1 bslbf
      reserved                                   5 bslbf
      if (program_splice_flag == '1')
        utc_splice_time                         32 uimsbf
      if (program_splice_flag == '0') {
        component_count                          8 uimsbf
        for(j=0;j<component_count;j++) {
          component_tag                          8 uimsbf
          utc_splice_time                       32 uimsbf
        }
      }
      if (duration_flag)
        break_duration()
      unique_program_id                         16 uimsbf
      avail_num                                  8 uimsbf
      avails_expected                            8 uimsbf
    }
  }
}
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct SpliceSchedule {
    pub events: Vec<Event>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Event {
    /// A 32-bit unique splice event identifier.
    pub event_id: u32,
    /// Information on the scheduled event. If this value is `None` it indicates that a previously
    /// sent splice event, identified by `event_id`, has been cancelled.
    pub scheduled_event: Option<ScheduledEvent>,
}
impl Event {
    /// When set to `true` indicates that a previously sent splice event, identified by `event_id`,
    /// has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.scheduled_event == None
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ScheduledEvent {
    /// When set to `true`, indicates that the splice event is an opportunity to exit from the
    /// network feed and that the value of `utc_splice_time` shall refer to an intended out point
    /// or program out point. When set to `false`, the flag indicates that the splice event is an
    /// opportunity to return to the network feed and that the value of `utc_splice_time` shall
    /// refer to an intended in point or program in point.
    pub out_of_network_indicator: bool,
    /// Information on the type of splice message.
    pub splice_mode: SpliceMode,
    /// The `BreakDuration` structure specifies the duration of the commercial break(s). It may be
    /// used to give the splicer an indication of when the break will be over and when the network
    /// in point will occur.
    pub break_duration: Option<BreakDuration>,
    /// This value should provide a unique identification for a viewing event within the service.
    pub unique_program_id: u16,
    /// This field provides an identification for a specific avail within one `unique_program_id`.
    /// This value is expected to increment with each new avail within a viewing event. This value
    /// is expected to reset to one for the first avail in a new viewing event. This field is
    /// expected to increment for each new avail. It may optionally carry a zero value to indicate
    /// its non-usage.
    pub avail_num: u8,
    /// This field provides a count of the expected number of individual avails within the current
    /// viewing event. When this field is zero, it indicates that the `avail_num` field has no
    /// meaning.
    pub avails_expected: u8,
}

/// Information on the type of splice message.
#[derive(PartialEq, Eq, Debug)]
pub enum SpliceMode {
    /// Indicates that the message refers to a Program Splice Point and that the mode is the
    /// Program Splice Mode whereby all PIDs/components of the program are to be spliced.
    ProgramSpliceMode(ProgramMode),
    /// Indicates that the mode is the Component Splice Mode whereby each component that is
    /// intended to be spliced will be listed separately by the syntax that follows.
    ComponentSpliceMode(Vec<ComponentMode>),
}

/// Indicates that the message refers to a Program Splice Point and that the mode is the Program
/// Splice Mode whereby all PIDs/components of the program are to be spliced.
#[derive(PartialEq, Eq, Debug)]
pub struct ProgramMode {
    /// A 32-bit unsigned integer quantity representing the time of the signalled splice event as
    /// the number of seconds since 00 hours coordinated universal time (UTC), January 6th, 1980,
    /// with the count of intervening leap seconds included. The `utc_splice_time` may be converted
    /// to UTC without the use of the GPS_UTC_offset value provided by the System Time table. The
    /// `utc_splice_time` field is used only in the `SpliceSchedule` command.
    pub utc_splice_time: u32,
}

/// Indicates that the mode is the Component Splice Mode whereby each component that is intended to
/// be spliced will be listed separately by the syntax that follows.
#[derive(PartialEq, Eq, Debug)]
pub struct ComponentMode {
    /// An 8-bit value that identifies the elementary PID stream containing the Splice Point
    /// specified by the value of `utc_splice_time` that follows. The value shall be the same as
    /// the value used in the stream_identification_descriptor() to identify that elementary PID
    /// stream.
    pub component_tag: u8,
    /// A 32-bit unsigned integer quantity representing the time of the signalled splice event as
    /// the number of seconds since 00 hours coordinated universal time (UTC), January 6th, 1980,
    /// with the count of intervening leap seconds included. The `utc_splice_time` may be converted
    /// to UTC without the use of the GPS_UTC_offset value provided by the System Time table. The
    /// `utc_splice_time` field is used only in the `SpliceSchedule` command.
    pub utc_splice_time: u32,
}

impl SpliceSchedule {
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let splice_count = bits.byte();
        let mut events = vec![];
        for _ in 0..splice_count {
            events.push(Event::try_from(bits)?);
        }
        Ok(Self { events })
    }
}

impl Event {
    fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let event_id = bits.u32(32);
        let is_cancelled = bits.bool();
        bits.consume(7);
        if is_cancelled {
            Ok(Self {
                event_id,
                scheduled_event: None,
            })
        } else {
            Ok(Self {
                event_id,
                scheduled_event: Some(ScheduledEvent::try_from(bits)?),
            })
        }
    }
}

impl ScheduledEvent {
    fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let out_of_network_indicator = bits.bool();
        let program_splice_flag = bits.bool();
        let duration_flag = bits.bool();
        bits.consume(5);
        let splice_mode = if program_splice_flag {
            SpliceMode::ProgramSpliceMode(ProgramMode {
                utc_splice_time: bits.u32(32),
            })
        } else {
            let component_count = bits.byte();
            let mut components = vec![];
            for _ in 0..component_count {
                let component_tag = bits.byte();
                let utc_splice_time = bits.u32(32);
                components.push(ComponentMode {
                    component_tag,
                    utc_splice_time,
                });
            }
            SpliceMode::ComponentSpliceMode(components)
        };
        let break_duration = if duration_flag {
            Some(BreakDuration::try_from(bits)?)
        } else {
            None
        };
        let unique_program_id = bits.u16(16);
        let avail_num = bits.byte();
        let avails_expected = bits.byte();
        Ok(Self {
            out_of_network_indicator,
            splice_mode,
            break_duration,
            unique_program_id,
            avail_num,
            avails_expected,
        })
    }
}
