use crate::time::{BreakDuration, SpliceTime};

/// The `SpliceInsert` command shall be sent at least once for every splice event.
/**
```
// splice_insert() {
//   splice_event_id                                                      32 uimsbf
//   splice_event_cancel_indicator                                         1 bslbf
//   reserved                                                              7 bslbf
//   if(splice_event_cancel_indicator == '0') {
//     out_of_network_indicator                                            1 bslbf
//     program_splice_flag                                                 1 bslbf
//     duration_flag                                                       1 bslbf
//     splice_immediate_flag                                               1 bslbf
//     reserved                                                            4 bslbf
//     if((program_splice_flag == '1') && (splice_immediate_flag == '0'))
//       splice_time()
//     if(program_splice_flag == '0') {
//       component_count                                                   8 uimsbf
//       for(i=0;i<component_count;i++) {
//         component_tag                                                   8 uimsbf
//         if(splice_immediate_flag == '0')
//           splice_time()
//       }
//     }
//     if(duration_flag == '1')
//       break_duration()
//     unique_program_id                                                  16 uimsbf
//     avail_num                                                           8 uimsbf
//     avails_expected                                                     8 uimsbf
//   }
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct SpliceInsert {
    /// A 32-bit unique splice event identifier.
    pub event_id: u32,
    /// Information on the scheduled event. If this value is `None` it indicates that a previously
    /// sent splice event, identified by `event_id`, has been cancelled.
    pub scheduled_event: Option<ScheduledEvent>,
}
impl SpliceInsert {
    /// When set to `true` indicates that a previously sent splice event, identified by `event_id`,
    /// has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.scheduled_event == None
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ScheduledEvent {
    /// When set to `true`, indicates that the splice event is an opportunity to exit from the
    /// network feed and that the value of `splice_time`, as modified by `pts_adjustment`, shall
    /// refer to an intended out point or program out point. When set to `false`, the flag
    /// indicates that the splice event is an opportunity to return to the network feed and that
    /// the value of `splice_time`, as modified by `pts_adjustment`, shall refer to an intended in
    /// point or program in point.
    pub out_of_network_indicator: bool,
    /// When this flag is `true`, it indicates the absence of the `splice_time` field and that the
    /// splice mode shall be the Splice Immediate Mode, whereby the splicing device shall choose
    /// the nearest opportunity in the stream, relative to the splice information packet, to
    /// splice. When this flag is `false`, it indicates the presence of the `splice_time` field in
    /// at least one location within the `SpliceInsert` command.
    ///
    /// In this specific scenario, a value of `true` indicates that all `splice_time` values within
    /// the `splice_mode` enum will be `None`, and the converse for a value of `false`.
    pub is_immediate_splice: bool,
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
    /// The `SpliceTime` structure, when modified by `pts_adjustment`, specifies the time of the
    /// splice event.
    pub splice_time: Option<SpliceTime>,
}

/// Indicates that the mode is the Component Splice Mode whereby each component that is intended to
/// be spliced will be listed separately by the syntax that follows.
#[derive(PartialEq, Eq, Debug)]
pub struct ComponentMode {
    /// An 8-bit value that identifies the elementary PID stream containing the Splice Point
    /// specified by the value of `splice_time` that follows. The value shall be the same as the
    /// value used in the stream_identification_descriptor() to identify that elementary PID
    /// stream.
    pub component_tag: u8,
    /// The `SpliceTime` structure, when modified by `pts_adjustment`, specifies the time of the
    /// splice event.
    pub splice_time: Option<SpliceTime>,
}
