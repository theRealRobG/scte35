use self::{
    private_command::PrivateCommand, splice_insert::SpliceInsert, splice_schedule::SpliceSchedule,
    time_signal::TimeSignal,
};

pub mod private_command;
pub mod splice_insert;
pub mod splice_schedule;
pub mod time_signal;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SpliceCommandType {
    SpliceNull,
    SpliceSchedule,
    SpliceInsert,
    TimeSignal,
    BandwidthReservation,
    PrivateCommand,
}

impl SpliceCommandType {
    pub fn value(&self) -> u8 {
        match *self {
            SpliceCommandType::SpliceNull => 0x00,
            SpliceCommandType::SpliceSchedule => 0x04,
            SpliceCommandType::SpliceInsert => 0x05,
            SpliceCommandType::TimeSignal => 0x06,
            SpliceCommandType::BandwidthReservation => 0x07,
            SpliceCommandType::PrivateCommand => 0xff,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum SpliceCommand {
    /// The `SpliceNull` command is provided for extensibility of the standard. The `SpliceNull`
    /// command allows a `SpliceInfoTable` to be sent that can carry descriptors without having to
    /// send one of the other defined commands. This command may also be used as a "heartbeat
    /// message" for monitoring cue injection equipment integrity and link integrity.
    SpliceNull,
    /// The `SpliceSchedule` command is provided to allow a schedule of splice events to be
    /// conveyed in advance.
    SpliceSchedule(SpliceSchedule),
    /// The `SpliceInsert` command shall be sent at least once for every splice event.
    SpliceInsert(SpliceInsert),
    /// The `TimeSignal` provides a time synchronized data delivery mechanism. The syntax of the
    /// `TimeSignal` allows for the synchronization of the information carried in this message with
    /// the system time clock (STC). The unique payload of the message is carried in the
    /// descriptor, however the syntax an transport capabilities afforded to `SpliceInsert`
    /// messages are also afforded to the `TimeSignal`. The carriage however can be in a different
    /// PID than that carrying the other cue messages used for signalling splice points.
    TimeSignal(TimeSignal),
    /// The `BandwidthReservation` command is provided for reserving bandwidth in a multiplex. A
    /// typical usage would be in a satellite delivery system that requires packets of a certain
    /// PID to always be present at the intended repetition rate to guarantee a certain bandwidth
    /// for that PID. This message differs from a `SpliceNull` command so that it can easily be
    /// handled in a unique way by receiving equipment (i.e., removed from the multiplex by a
    /// satellite receiver). If a descriptor is sent with this command, it cannot be expected that
    /// it will be carried through the entire transmission chain and it should be a private
    /// descriptor that is utilized only by the bandwidth reservation process.
    BandwidthReservation,
    /// The `PrivateCommand` structure provides a means to distribute user-defined commands using
    /// the SCTE 35 protocol. The first bit field in each user-defined command is a 32-bit
    /// identifier, unique for each participating vendor. Receiving equipment should skip any
    /// `SpliceInfoSection` messages containing `PrivateCommand` structures with unknown
    /// identifiers.
    PrivateCommand(PrivateCommand),
}

impl SpliceCommand {
    pub fn command_type(&self) -> SpliceCommandType {
        match *self {
            SpliceCommand::SpliceNull => SpliceCommandType::SpliceNull,
            SpliceCommand::SpliceSchedule(_) => SpliceCommandType::SpliceSchedule,
            SpliceCommand::SpliceInsert(_) => SpliceCommandType::SpliceInsert,
            SpliceCommand::TimeSignal(_) => SpliceCommandType::TimeSignal,
            SpliceCommand::BandwidthReservation => SpliceCommandType::BandwidthReservation,
            SpliceCommand::PrivateCommand(_) => SpliceCommandType::PrivateCommand,
        }
    }
}
