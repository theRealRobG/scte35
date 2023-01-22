use super::DescriptorLengthExpectation;
use crate::{atsc::ATSCContentIdentifier, bit_reader::Bits, error::ParseError, hex::encode_hex};
use ::std::fmt::Write;

/// The `SegmentationDescriptor` is an implementation of a `SpliceDescriptor`. It provides an
/// optional extension to the `TimeSignal` and `SpliceInsert` commands that allows for segmentation
/// messages to be sent in a time/video accurate method. This descriptor shall only be used with
/// the `TimeSignal`, `SpliceInsert` and the `SpliceNull` commands. The `TimeSignal` or
/// `SpliceInsert` message should be sent at least once a minimum of 4 seconds in advance of the
/// signaled `SpliceTime` to permit the insertion device to place the `SpliceInfoSection`
/// accurately. Devices that do not recognize a value in any field shall ignore the message and
/// take no action.
/**
```
// segmentation_descriptor() {
//   splice_descriptor_tag                             8 uimsbf
//   descriptor_length                                 8 uimsbf
//   identifier                                       32 uimsbf
//   segmentation_event_id                            32 uimsbf
//   segmentation_event_cancel_indicator               1 bslbf
//   reserved                                          7 bslbf
//   if(segmentation_event_cancel_indicator == ‘0’) {
//     program_segmentation_flag                       1 bslbf
//     segmentation_duration_flag                      1 bslbf
//     delivery_not_restricted_flag                    1 bslbf
//     if(delivery_not_restricted_flag == ‘0’) {
//       web_delivery_allowed_flag                     1 bslbf
//       no_regional_blackout_flag                     1 bslbf
//       archive_allowed_flag                          1 bslbf
//       device_restrictions                           2 bslbf
//     } else {
//       reserved                                      5 bslbf
//     }
//     if(program_segmentation_flag == ‘0’) {
//       component_count                               8 uimsbf
//       for(i=0;i<component_count;i++) {
//         component_tag                               8 uimsbf
//         reserved                                    7 bslbf
//         pts_offset                                 33 uimsbf
//       }
//     }
//     if(segmentation_duration_flag == ‘1’)
//       segmentation_duration                        40 uimsbf
//     segmentation_upid_type                          8 uimsbf
//     segmentation_upid_length                        8 uimsbf
//     segmentation_upid()
//     segmentation_type_id                            8 uimsbf
//     segment_num                                     8 uimsbf
//     segments_expected                               8 uimsbf
//     if(segmentation_type_id == ‘0x34’ ||
//       segmentation_type_id == ‘0x36’ ||
//       segmentation_type_id == ‘0x38’ ||
//       segmentation_type_id == ‘0x3A’) {
//         sub_segment_num                             8 uimsbf
//         sub_segments_expected                       8 uimsbf
//     }
//   }
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct SegmentationDescriptor {
    /// This 32-bit number is used to identify the owner of the descriptor. The identifier shall
    /// have a value of 0x43554549 (ASCII “CUEI”).
    pub identifier: u32,
    /// A 32-bit unique segmentation event identifier.
    pub event_id: u32,
    /// Information on the scheduled event. If this value is `None` it indicates that a previously
    /// sent segmentation descriptor, identified by `event_id`, has been cancelled.
    pub scheduled_event: Option<ScheduledEvent>,
}
impl SegmentationDescriptor {
    /// When set to `true` indicates that a previously sent segmentation descriptor, identified by
    /// `event_id`, has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.scheduled_event == None
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ScheduledEvent {
    /// This is provided to facilitate implementations that use methods that are out of scope of
    /// this standard to process and manage this Segment.
    pub delivery_restrictions: Option<DeliveryRestrictions>,
    /// When not defined, indicates that the message refers to a Program Segmentation Point and
    /// that the mode is the Program Segmentation Mode whereby all PIDs/components of the program
    /// are to be segmented. When defined, this field indicates that the mode is the Component
    /// Segmentation Mode whereby each component that is intended to be segmented will be listed
    /// separately.
    pub component_segments: Option<Vec<ComponentSegmentation>>,
    /// A 40-bit unsigned integer that specifies the duration of the Segment in terms of ticks of
    /// the program’s 90 kHz clock. It may be used to give the splicer an indication of when the
    /// Segment will be over and when the next segmentation message will occur. Shall be `0` for
    /// end messages.
    pub segmentation_duration: Option<u64>,
    /// There are multiple types allowed to ensure that programmers will be able to use an id that
    /// their systems support. It is expected that the consumers of these ids will have an
    /// out-of-band method of collecting other data related to these numbers and therefore they do
    /// not need to be of identical types. These ids may be in other descriptors in the Program
    /// and, where the same identifier is used (ISAN for example), it shall match between Programs.
    pub segmentation_upid: SegmentationUPID,
    /// Designates the type of segmentation. All unused values are reserved. When the
    /// `SegmentationTypeID` is `0x01` (`ContentIdentification`), the value of
    /// `SegmentationUPIDType` shall be non-zero. If `segmentation_upid_length` is zero, then
    /// `SegmentationTypeID` shall be set to `0x00` for Not Indicated.
    pub segmentation_type_id: SegmentationTypeID,
    /// This field provides support for numbering segments within a given collection of Segments
    /// (such as Chapters or Advertisements). This value, when utilized, is expected to reset to
    /// one at the beginning of a collection of Segments. This field is expected to increment for
    /// each new Segment (such as a Chapter).
    pub segment_num: u8,
    /// This field provides a count of the expected number of individual Segments (such as
    /// Chapters) within a collection of Segments.
    pub segments_expected: u8,
    /// Provides information for a collection of sub-Segments.
    pub sub_segment: Option<SubSegment>,
}

/// This is provided to facilitate implementations that use methods that are out of scope of this
/// standard to process and manage this Segment.
#[derive(PartialEq, Eq, Debug)]
pub struct DeliveryRestrictions {
    /// This shall have the value of `true` when there are no restrictions with respect to web
    /// delivery of this Segment. This shall have the value of `false` to signal that restrictions
    /// related to web delivery of this Segment are asserted.
    pub web_delivery_allowed: bool,
    /// This shall have the value of `true` when there is no regional blackout of this Segment.
    /// This shall have the value of `false` when this Segment is restricted due to regional
    /// blackout rules.
    pub no_regional_blackout: bool,
    /// This shall have the value of `true` when there is no assertion about recording this
    /// Segment. This shall have the value of `false` to signal that restrictions related to
    /// recording this Segment are asserted.
    pub archive_allowed: bool,
    /// This field signals three pre-defined groups of devices. The population of each group is
    /// independent and the groups are non-hierarchical. The delivery and format of the messaging
    /// to define the devices contained in the groups is out of the scope of this standard.
    pub device_restrictions: DeviceRestrictions,
}

/// This field signals three pre-defined groups of devices. The population of each group is
/// independent and the groups are non-hierarchical. The delivery and format of the messaging to
/// define the devices contained in the groups is out of the scope of this standard.
#[derive(PartialEq, Eq, Debug)]
pub enum DeviceRestrictions {
    /// 00 - This Segment is restricted for a class of devices defined by an out of band message
    /// that describes which devices are excluded.
    RestrictGroup0,
    /// 01 - This Segment is restricted for a class of devices defined by an out of band message
    /// that describes which devices are excluded.
    RestrictGroup1,
    /// 10 - This Segment is restricted for a class of devices defined by an out of band message
    /// that describes which devices are excluded.
    RestrictGroup2,
    /// 11 - This Segment has no device restrictions.
    None,
}

impl TryFrom<u8> for DeviceRestrictions {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceRestrictions::RestrictGroup0),
            1 => Ok(DeviceRestrictions::RestrictGroup1),
            2 => Ok(DeviceRestrictions::RestrictGroup2),
            3 => Ok(DeviceRestrictions::None),
            _ => Err("Unexpected u8 for DeviceRestrictions"),
        }
    }
}

impl DeviceRestrictions {
    pub fn value(&self) -> u8 {
        match *self {
            DeviceRestrictions::RestrictGroup0 => 0,
            DeviceRestrictions::RestrictGroup1 => 1,
            DeviceRestrictions::RestrictGroup2 => 2,
            DeviceRestrictions::None => 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ComponentSegmentation {
    /// An 8-bit value that identifies the elementary PID stream containing the Segmentation Point
    /// specified by the value of `SpliceTime` that follows. The value shall be the same as the value
    /// used in the `stream_identifier_descriptor()` to identify that elementary PID stream. The
    /// presence of this field from the component loop denotes the presence of this component of
    /// the asset.
    pub component_tag: u8,
    /// A 33-bit unsigned integer that shall be used by a splicing device as an offset to be added
    /// to the `pts_time`, as modified by `pts_adjustment`, in the `TimeSignal` message to obtain
    /// the intended splice time(s). When this field has a zero value, then the `pts_time` field(s)
    /// shall be used without an offset. If `SpliceTime` has no `pts_time` or if the command this
    /// descriptor is carried with does not have a `SpliceTime` field, this field shall be used to
    /// offset the derived immediate splice time.
    pub pts_offset: u64,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SubSegment {
    /// If specified, this field provides identification for a specific sub-Segment within a
    /// collection of sub-Segments. This value, when utilized, is expected to be set to one for the
    /// first sub-Segment within a collection of sub-Segments. This field is expected to increment
    /// by one for each new sub-Segment within a given collection.
    pub sub_segment_num: u8,
    /// If specified, this field provides a count of the expected number of individual sub-Segments
    /// within the collection of sub-Segments.
    pub sub_segments_expected: u8,
}

/// Designates the type of segmentation. All unused values are reserved. When the
/// `SegmentationTypeID` is `0x01` (`ContentIdentification`), the value of `SegmentationUPIDType`
/// shall be non-zero. If `segmentation_upid_length` is zero, then `SegmentationTypeID` shall be
/// set to `0x00` for Not Indicated.
#[derive(PartialEq, Eq, Debug)]
pub enum SegmentationTypeID {
    /// 0x00
    NotIndicated,
    /// 0x01
    ContentIdentification,
    /// 0x10
    ProgramStart,
    /// 0x11
    ProgramEnd,
    /// 0x12
    ProgramEarlyTermination,
    /// 0x13
    ProgramBreakaway,
    /// 0x14
    ProgramResumption,
    /// 0x15
    ProgramRunoverPlanned,
    /// 0x16
    ProgramRunoverUnplanned,
    /// 0x17
    ProgramOverlapStart,
    /// 0x18
    ProgramBlackoutOverride,
    /// 0x19
    ProgramJoin,
    /// 0x20
    ChapterStart,
    /// 0x21
    ChapterEnd,
    /// 0x22
    BreakStart,
    /// 0x23
    BreakEnd,
    /// 0x24
    OpeningCreditStart,
    /// 0x25
    OpeningCreditEnd,
    /// 0x26
    ClosingCreditStart,
    /// 0x27
    ClosingCreditEnd,
    /// 0x30
    ProviderAdvertisementStart,
    /// 0x31
    ProviderAdvertisementEnd,
    /// 0x32
    DistributorAdvertisementStart,
    /// 0x33
    DistributorAdvertisementEnd,
    /// 0x34
    ProviderPlacementOpportunityStart,
    /// 0x35
    ProviderPlacementOpportunityEnd,
    /// 0x36
    DistributorPlacementOpportunityStart,
    /// 0x37
    DistributorPlacementOpportunityEnd,
    /// 0x38
    ProviderOverlayPlacementOpportunityStart,
    /// 0x39
    ProviderOverlayPlacementOpportunityEnd,
    /// 0x3A
    DistributorOverlayPlacementOpportunityStart,
    /// 0x3B
    DistributorOverlayPlacementOpportunityEnd,
    /// 0x3C
    ProviderPromoStart,
    /// 0x3D
    ProviderPromoEnd,
    /// 0x3E
    DistributorPromoStart,
    /// 0x3F
    DistributorPromoEnd,
    /// 0x40
    UnscheduledEventStart,
    /// 0x41
    UnscheduledEventEnd,
    /// 0x42
    AlternateContentOpportunityStart,
    /// 0x43
    AlternateContentOpportunityEnd,
    /// 0x44
    ProviderAdBlockStart,
    /// 0x45
    ProviderAdBlockEnd,
    /// 0x46
    DistributorAdBlockStart,
    /// 0x47
    DistributorAdBlockEnd,
    /// 0x50
    NetworkStart,
    /// 0x51
    NetworkEnd,
}

impl TryFrom<u8> for SegmentationTypeID {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SegmentationTypeID::NotIndicated),
            0x01 => Ok(SegmentationTypeID::ContentIdentification),
            0x10 => Ok(SegmentationTypeID::ProgramStart),
            0x11 => Ok(SegmentationTypeID::ProgramEnd),
            0x12 => Ok(SegmentationTypeID::ProgramEarlyTermination),
            0x13 => Ok(SegmentationTypeID::ProgramBreakaway),
            0x14 => Ok(SegmentationTypeID::ProgramResumption),
            0x15 => Ok(SegmentationTypeID::ProgramRunoverPlanned),
            0x16 => Ok(SegmentationTypeID::ProgramRunoverUnplanned),
            0x17 => Ok(SegmentationTypeID::ProgramOverlapStart),
            0x18 => Ok(SegmentationTypeID::ProgramBlackoutOverride),
            0x19 => Ok(SegmentationTypeID::ProgramJoin),
            0x20 => Ok(SegmentationTypeID::ChapterStart),
            0x21 => Ok(SegmentationTypeID::ChapterEnd),
            0x22 => Ok(SegmentationTypeID::BreakStart),
            0x23 => Ok(SegmentationTypeID::BreakEnd),
            0x24 => Ok(SegmentationTypeID::OpeningCreditStart),
            0x25 => Ok(SegmentationTypeID::OpeningCreditEnd),
            0x26 => Ok(SegmentationTypeID::ClosingCreditStart),
            0x27 => Ok(SegmentationTypeID::ClosingCreditEnd),
            0x30 => Ok(SegmentationTypeID::ProviderAdvertisementStart),
            0x31 => Ok(SegmentationTypeID::ProviderAdvertisementEnd),
            0x32 => Ok(SegmentationTypeID::DistributorAdvertisementStart),
            0x33 => Ok(SegmentationTypeID::DistributorAdvertisementEnd),
            0x34 => Ok(SegmentationTypeID::ProviderPlacementOpportunityStart),
            0x35 => Ok(SegmentationTypeID::ProviderPlacementOpportunityEnd),
            0x36 => Ok(SegmentationTypeID::DistributorPlacementOpportunityStart),
            0x37 => Ok(SegmentationTypeID::DistributorPlacementOpportunityEnd),
            0x38 => Ok(SegmentationTypeID::ProviderOverlayPlacementOpportunityStart),
            0x39 => Ok(SegmentationTypeID::ProviderOverlayPlacementOpportunityEnd),
            0x3A => Ok(SegmentationTypeID::DistributorOverlayPlacementOpportunityStart),
            0x3B => Ok(SegmentationTypeID::DistributorOverlayPlacementOpportunityEnd),
            0x3C => Ok(SegmentationTypeID::ProviderPromoStart),
            0x3D => Ok(SegmentationTypeID::ProviderPromoEnd),
            0x3E => Ok(SegmentationTypeID::DistributorPromoStart),
            0x3F => Ok(SegmentationTypeID::DistributorPromoEnd),
            0x40 => Ok(SegmentationTypeID::UnscheduledEventStart),
            0x41 => Ok(SegmentationTypeID::UnscheduledEventEnd),
            0x42 => Ok(SegmentationTypeID::AlternateContentOpportunityStart),
            0x43 => Ok(SegmentationTypeID::AlternateContentOpportunityEnd),
            0x44 => Ok(SegmentationTypeID::ProviderAdBlockStart),
            0x45 => Ok(SegmentationTypeID::ProviderAdBlockEnd),
            0x46 => Ok(SegmentationTypeID::DistributorAdBlockStart),
            0x47 => Ok(SegmentationTypeID::DistributorAdBlockEnd),
            0x50 => Ok(SegmentationTypeID::NetworkStart),
            0x51 => Ok(SegmentationTypeID::NetworkEnd),
            _ => Err(ParseError::UnrecognisedSegmentationTypeID(value)),
        }
    }
}

impl SegmentationTypeID {
    pub fn value(&self) -> u8 {
        match *self {
            SegmentationTypeID::NotIndicated => 0x00,
            SegmentationTypeID::ContentIdentification => 0x01,
            SegmentationTypeID::ProgramStart => 0x10,
            SegmentationTypeID::ProgramEnd => 0x11,
            SegmentationTypeID::ProgramEarlyTermination => 0x12,
            SegmentationTypeID::ProgramBreakaway => 0x13,
            SegmentationTypeID::ProgramResumption => 0x14,
            SegmentationTypeID::ProgramRunoverPlanned => 0x15,
            SegmentationTypeID::ProgramRunoverUnplanned => 0x16,
            SegmentationTypeID::ProgramOverlapStart => 0x17,
            SegmentationTypeID::ProgramBlackoutOverride => 0x18,
            SegmentationTypeID::ProgramJoin => 0x19,
            SegmentationTypeID::ChapterStart => 0x20,
            SegmentationTypeID::ChapterEnd => 0x21,
            SegmentationTypeID::BreakStart => 0x22,
            SegmentationTypeID::BreakEnd => 0x23,
            SegmentationTypeID::OpeningCreditStart => 0x24,
            SegmentationTypeID::OpeningCreditEnd => 0x25,
            SegmentationTypeID::ClosingCreditStart => 0x26,
            SegmentationTypeID::ClosingCreditEnd => 0x27,
            SegmentationTypeID::ProviderAdvertisementStart => 0x30,
            SegmentationTypeID::ProviderAdvertisementEnd => 0x31,
            SegmentationTypeID::DistributorAdvertisementStart => 0x32,
            SegmentationTypeID::DistributorAdvertisementEnd => 0x33,
            SegmentationTypeID::ProviderPlacementOpportunityStart => 0x34,
            SegmentationTypeID::ProviderPlacementOpportunityEnd => 0x35,
            SegmentationTypeID::DistributorPlacementOpportunityStart => 0x36,
            SegmentationTypeID::DistributorPlacementOpportunityEnd => 0x37,
            SegmentationTypeID::ProviderOverlayPlacementOpportunityStart => 0x38,
            SegmentationTypeID::ProviderOverlayPlacementOpportunityEnd => 0x39,
            SegmentationTypeID::DistributorOverlayPlacementOpportunityStart => 0x3A,
            SegmentationTypeID::DistributorOverlayPlacementOpportunityEnd => 0x3B,
            SegmentationTypeID::ProviderPromoStart => 0x3C,
            SegmentationTypeID::ProviderPromoEnd => 0x3D,
            SegmentationTypeID::DistributorPromoStart => 0x3E,
            SegmentationTypeID::DistributorPromoEnd => 0x3F,
            SegmentationTypeID::UnscheduledEventStart => 0x40,
            SegmentationTypeID::UnscheduledEventEnd => 0x41,
            SegmentationTypeID::AlternateContentOpportunityStart => 0x42,
            SegmentationTypeID::AlternateContentOpportunityEnd => 0x43,
            SegmentationTypeID::ProviderAdBlockStart => 0x44,
            SegmentationTypeID::ProviderAdBlockEnd => 0x45,
            SegmentationTypeID::DistributorAdBlockStart => 0x46,
            SegmentationTypeID::DistributorAdBlockEnd => 0x47,
            SegmentationTypeID::NetworkStart => 0x50,
            SegmentationTypeID::NetworkEnd => 0x51,
        }
    }
}

/// There are multiple types allowed to ensure that programmers will be able to use an id that
/// their systems support. It is expected that the consumers of these ids will have an out-of-band
/// method of collecting other data related to these numbers and therefore they do not need to be
/// of identical types. These ids may be in other descriptors in the Program and, where the same
/// identifier is used (ISAN for example), it shall match between Programs.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SegmentationUPIDType {
    NotUsed,
    UserDefined,
    ISCI,
    AdID,
    UMID,
    DeprecatedISAN,
    ISAN,
    TID,
    TI,
    ADI,
    EIDR,
    ATSCContentIdentifier,
    MPU,
    MID,
    ADSInformation,
    URI,
    UUID,
}

impl TryFrom<u8> for SegmentationUPIDType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SegmentationUPIDType::NotUsed),
            0x01 => Ok(SegmentationUPIDType::UserDefined),
            0x02 => Ok(SegmentationUPIDType::ISCI),
            0x03 => Ok(SegmentationUPIDType::AdID),
            0x04 => Ok(SegmentationUPIDType::UMID),
            0x05 => Ok(SegmentationUPIDType::DeprecatedISAN),
            0x06 => Ok(SegmentationUPIDType::ISAN),
            0x07 => Ok(SegmentationUPIDType::TID),
            0x08 => Ok(SegmentationUPIDType::TI),
            0x09 => Ok(SegmentationUPIDType::ADI),
            0x0A => Ok(SegmentationUPIDType::EIDR),
            0x0B => Ok(SegmentationUPIDType::ATSCContentIdentifier),
            0x0C => Ok(SegmentationUPIDType::MPU),
            0x0D => Ok(SegmentationUPIDType::MID),
            0x0E => Ok(SegmentationUPIDType::ADSInformation),
            0x0F => Ok(SegmentationUPIDType::URI),
            0x10 => Ok(SegmentationUPIDType::UUID),
            _ => Err(ParseError::UnrecognisedSegmentationUPIDType(value)),
        }
    }
}

impl SegmentationUPIDType {
    pub fn value(&self) -> u8 {
        match *self {
            SegmentationUPIDType::NotUsed => 0x00,
            SegmentationUPIDType::UserDefined => 0x01,
            SegmentationUPIDType::ISCI => 0x02,
            SegmentationUPIDType::AdID => 0x03,
            SegmentationUPIDType::UMID => 0x04,
            SegmentationUPIDType::DeprecatedISAN => 0x05,
            SegmentationUPIDType::ISAN => 0x06,
            SegmentationUPIDType::TID => 0x07,
            SegmentationUPIDType::TI => 0x08,
            SegmentationUPIDType::ADI => 0x09,
            SegmentationUPIDType::EIDR => 0x0A,
            SegmentationUPIDType::ATSCContentIdentifier => 0x0B,
            SegmentationUPIDType::MPU => 0x0C,
            SegmentationUPIDType::MID => 0x0D,
            SegmentationUPIDType::ADSInformation => 0x0E,
            SegmentationUPIDType::URI => 0x0F,
            SegmentationUPIDType::UUID => 0x10,
        }
    }
}

/// There are multiple types allowed to ensure that programmers will be able to use an id that
/// their systems support. It is expected that the consumers of these ids will have an out-of-band
/// method of collecting other data related to these numbers and therefore they do not need to be
/// of identical types. These ids may be in other descriptors in the Program and, where the same
/// identifier is used (ISAN for example), it shall match between Programs.
#[derive(PartialEq, Eq, Debug)]
pub enum SegmentationUPID {
    /// The `SegmentationUPID` is not defined and is not present in the descriptor.
    NotUsed,
    /// Deprecated: use type `0x0C`; The `SegmentationUPID` does not follow a standard naming
    /// scheme.
    UserDefined(String),
    /// Deprecated: use type `0x03`, 8 characters; 4 alpha characters followed by 4 numbers.
    ISCI(String),
    /// Defined by the Advertising Digital Identification, LLC group. 12 characters; 4 alpha
    /// characters (company identification prefix) followed by 8 alphanumeric characters. (See
    /// `AdID`)
    AdID(String),
    /// See [SMPTE 330]
    UMID(String),
    /// Deprecated: use type `0x06`, ISO 15706 binary encoding.
    DeprecatedISAN(String),
    /// Formerly known as V-ISAN. ISO 15706-2 binary encoding (“versioned” ISAN). See
    /// [ISO 15706-2].
    ISAN(String),
    /// Tribune Media Systems Program identifier. 12 characters; 2 alpha characters followed by 10
    /// numbers.
    TID(String),
    /// AiringID (Formerly Turner ID), used to indicate a specific airing of a Program that is
    /// unique within a network.
    TI(String),
    /// CableLabs metadata identifier.
    ///
    /// When the value of `SegmentationUPIDType` is `0x09` (ADI), it shall have the abbreviated
    /// syntax of `<element> : <identifier>`. The variable `<element>` shall take only the values
    /// `“PREVIEW”`, `“MPEG2HD”`, `“MPEG2SD”`, `“AVCHD”`, `“AVCSD”`, `“HEVCSD”`, `“HEVCHD”`,
    /// `“SIGNAL”`, `“PO”` (PlacementOpportunity), `“BLACKOUT”` and `“OTHER”`.
    ///
    /// For CableLabs Content metadata 1.1 the variable `<identifier>` shall take the form
    /// `<providerID>/<assetID>`, the variables `<providerID>` and `<assetID>` shall be as
    /// specified in [CLADI1-1] Sections 5.3.1 for Movie or 5.5.1 for Preview represented as 7-bit
    /// printable ASCII characters (values ranging from 0x20 (space) to 0x7E (tilde)).
    ///
    /// SCTE 2362 provides compatibility with this identifier model as described in [SCTE 236]
    /// Section 7.11.1.
    ADI(String),
    /// An EIDR (see [EIDR]) represented in Compact Binary encoding as defined in Section 2.1.1 in
    /// EIDR ID Format (see [EIDR ID FORMAT])
    EIDR(String),
    /// `ATSC_content_identifier()` structure as defined in [ATSC A/57B].
    ATSCContentIdentifier(ATSCContentIdentifier),
    /// Managed Private UPID structure.
    MPU(ManagedPrivateUPID),
    /// Multiple UPID types structure.
    MID(Vec<SegmentationUPID>),
    /// Advertising information. The specific usage is out of scope of this standard.
    ADSInformation(String),
    /// Universal Resource Identifier (see [RFC 3986]).
    URI(String),
    /// Universally Unique Identifier (see [RFC 4122]). This `SegmentationUPIDType` can be used
    /// instead of an URI if it is desired to transfer the UUID payload only.
    UUID(String),
}

impl SegmentationUPID {
    pub fn upid_type(&self) -> SegmentationUPIDType {
        match *self {
            SegmentationUPID::NotUsed => SegmentationUPIDType::NotUsed,
            SegmentationUPID::UserDefined(_) => SegmentationUPIDType::UserDefined,
            SegmentationUPID::ISCI(_) => SegmentationUPIDType::ISCI,
            SegmentationUPID::AdID(_) => SegmentationUPIDType::AdID,
            SegmentationUPID::UMID(_) => SegmentationUPIDType::UMID,
            SegmentationUPID::DeprecatedISAN(_) => SegmentationUPIDType::DeprecatedISAN,
            SegmentationUPID::ISAN(_) => SegmentationUPIDType::ISAN,
            SegmentationUPID::TID(_) => SegmentationUPIDType::TID,
            SegmentationUPID::TI(_) => SegmentationUPIDType::TI,
            SegmentationUPID::ADI(_) => SegmentationUPIDType::ADI,
            SegmentationUPID::EIDR(_) => SegmentationUPIDType::EIDR,
            SegmentationUPID::ATSCContentIdentifier(_) => {
                SegmentationUPIDType::ATSCContentIdentifier
            }
            SegmentationUPID::MPU(_) => SegmentationUPIDType::MPU,
            SegmentationUPID::MID(_) => SegmentationUPIDType::MID,
            SegmentationUPID::ADSInformation(_) => SegmentationUPIDType::ADSInformation,
            SegmentationUPID::URI(_) => SegmentationUPIDType::URI,
            SegmentationUPID::UUID(_) => SegmentationUPIDType::UUID,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ManagedPrivateUPID {
    pub format_specifier: String,
    pub private_data: Vec<u8>,
}

impl SegmentationDescriptor {
    // NOTE: It is assumed that the splice_descriptor_tag has already been read.
    pub fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let expectation = DescriptorLengthExpectation::try_from(bits, "SegmentationDescriptor")?;

        let identifier = bits.u32(32);
        if identifier != 1129661769 {
            return Err(ParseError::InvalidSegmentationDescriptorIdentifier(
                identifier,
            ));
        }
        let event_id = bits.u32(32);
        let segmentation_event_cancelled = bits.bool();
        bits.consume(7);
        let scheduled_event = if segmentation_event_cancelled {
            None
        } else {
            Some(ScheduledEvent::try_from(
                bits,
                expectation.expected_bits_remaining_after_descriptor as usize,
            )?)
        };

        expectation.validate_non_fatal(bits, super::SpliceDescriptorTag::SegmentationDescriptor);

        Ok(Self {
            identifier,
            event_id,
            scheduled_event,
        })
    }
}

impl ScheduledEvent {
    fn try_from(bits: &mut Bits, bits_left_after_descriptor: usize) -> Result<Self, ParseError> {
        let program_segmentation_flag = bits.bool();
        let segmentation_duration_flag = bits.bool();
        let delivery_not_restricted_flag = bits.bool();
        let delivery_restrictions = if delivery_not_restricted_flag {
            bits.consume(5);
            None
        } else {
            let web_delivery_allowed = bits.bool();
            let no_regional_blackout = bits.bool();
            let archive_allowed = bits.bool();
            let device_restrictions =
                DeviceRestrictions::try_from(bits.u8(2)).unwrap_or(DeviceRestrictions::None);
            Some(DeliveryRestrictions {
                web_delivery_allowed,
                no_regional_blackout,
                archive_allowed,
                device_restrictions,
            })
        };
        let component_segments = if program_segmentation_flag {
            None
        } else {
            let component_count = bits.byte();
            let mut components = vec![];
            for _ in 0..component_count {
                let component_tag = bits.byte();
                bits.consume(7);
                let pts_offset = bits.u64(33);
                components.push(ComponentSegmentation {
                    component_tag,
                    pts_offset,
                })
            }
            Some(components)
        };
        let segmentation_duration = if segmentation_duration_flag {
            Some(bits.u64(40))
        } else {
            None
        };
        let segmentation_upid = SegmentationUPID::try_from(bits)?;
        let segmentation_type_id = SegmentationTypeID::try_from(bits.byte())?;
        let segment_num = bits.byte();
        let segments_expected = bits.byte();
        let sub_segment =
            SubSegment::try_from(bits, &segmentation_type_id, bits_left_after_descriptor);
        Ok(Self {
            delivery_restrictions,
            component_segments,
            segmentation_duration,
            segmentation_upid,
            segmentation_type_id,
            segment_num,
            segments_expected,
            sub_segment,
        })
    }
}

impl SubSegment {
    fn try_from(
        bits: &mut Bits,
        segmentation_type_id: &SegmentationTypeID,
        bits_left_after_descriptor: usize,
    ) -> Option<Self> {
        let bits_left = bits.bits_remaining();
        if bits_left < 16 {
            return None;
        }
        if bits_left - 16 < bits_left_after_descriptor {
            return None;
        }
        match segmentation_type_id {
            SegmentationTypeID::ProviderPlacementOpportunityStart
            | SegmentationTypeID::DistributorPlacementOpportunityStart
            | SegmentationTypeID::ProviderOverlayPlacementOpportunityStart
            | SegmentationTypeID::DistributorOverlayPlacementOpportunityStart => {
                let sub_segment_num = bits.byte();
                let sub_segments_expected = bits.byte();
                Some(Self {
                    sub_segment_num,
                    sub_segments_expected,
                })
            }
            _ => None,
        }
    }
}

impl SegmentationUPID {
    fn try_from(bits: &mut Bits) -> Result<Self, ParseError> {
        let upid_type_raw_value = bits.byte();
        let upid_type = SegmentationUPIDType::try_from(upid_type_raw_value)?;
        let upid_length = bits.byte();
        bits.validate((upid_length as u32) * 8, "SegmentationUPID; reading loop")?;
        Self::try_from_with_type(bits, upid_type, upid_length)
    }

    fn try_from_with_type(
        bits: &mut Bits,
        upid_type: SegmentationUPIDType,
        upid_length: u8,
    ) -> Result<Self, ParseError> {
        match upid_type {
            SegmentationUPIDType::NotUsed => {
                validate(upid_length, 0, upid_type)?;
                Ok(Self::NotUsed)
            }
            SegmentationUPIDType::UserDefined => {
                let user_defined =
                    bits.string(upid_length as usize, "SegmentationUPIDType::UserDefined")?;
                Ok(Self::UserDefined(user_defined))
            }
            SegmentationUPIDType::ISCI => {
                validate(upid_length, 8, upid_type)?;
                let isci = bits.string(upid_length as usize, "SegmentationUPIDType::ISCI")?;
                Ok(Self::ISCI(isci))
            }
            SegmentationUPIDType::AdID => {
                validate(upid_length, 12, upid_type)?;
                let ad_id = bits.string(upid_length as usize, "SegmentationUPIDType::AdID")?;
                Ok(Self::AdID(ad_id))
            }
            SegmentationUPIDType::UMID => {
                validate(upid_length, 32, upid_type)?;
                let mut umid_vec = vec![];
                for _ in 0..8 {
                    let mut s = String::with_capacity(8);
                    write!(&mut s, "{:08x}", bits.u32(32)).unwrap();
                    umid_vec.push(s.to_uppercase());
                }
                Ok(Self::UMID(umid_vec.join(".")))
            }
            SegmentationUPIDType::DeprecatedISAN => {
                validate(upid_length, 8, upid_type)?;
                let check = HyphenSeparatedCheckedHex {
                    version: HyphenSeparatedCheckedHexVersion::DeprecatedISAN,
                };
                Ok(Self::DeprecatedISAN(check.read(bits)))
            }
            SegmentationUPIDType::ISAN => {
                validate(upid_length, 12, upid_type)?;
                let check = HyphenSeparatedCheckedHex {
                    version: HyphenSeparatedCheckedHexVersion::VersionedISAN,
                };
                Ok(Self::ISAN(check.read(bits)))
            }
            SegmentationUPIDType::TID => {
                validate(upid_length, 12, upid_type)?;
                let tid = bits.string(upid_length as usize, "SegmentationUPIDType::TID")?;
                Ok(Self::TID(tid))
            }
            SegmentationUPIDType::TI => {
                validate(upid_length, 8, upid_type)?;
                Ok(Self::TI(format!(
                    "0x{}",
                    encode_hex(&bits.bytes(8)).to_uppercase()
                )))
            }
            SegmentationUPIDType::ADI => {
                let adi = bits.string(upid_length as usize, "SegmentationUPIDType::ADI")?;
                Ok(Self::ADI(adi))
            }
            SegmentationUPIDType::EIDR => {
                validate(upid_length, 12, upid_type)?;
                let decimal = format!("10.{}", bits.u16(16));
                let check = HyphenSeparatedCheckedHex {
                    version: HyphenSeparatedCheckedHexVersion::Eidr,
                };
                let hex_components = check.read(bits);
                Ok(Self::EIDR(format!("{}/{}", decimal, hex_components)))
            }
            SegmentationUPIDType::ATSCContentIdentifier => {
                let atsc = ATSCContentIdentifier::try_from(bits, upid_length)?;
                Ok(Self::ATSCContentIdentifier(atsc))
            }
            SegmentationUPIDType::MPU => {
                let mpu = ManagedPrivateUPID::try_from(bits, upid_length)?;
                Ok(Self::MPU(mpu))
            }
            SegmentationUPIDType::MID => {
                let mut mid = vec![];
                let bits_remaining_after_upid =
                    bits.bits_remaining() - ((upid_length as usize) * 8);
                while bits.bits_remaining() > bits_remaining_after_upid {
                    mid.push(Self::try_from(bits)?);
                }
                Ok(Self::MID(mid))
            }
            SegmentationUPIDType::ADSInformation => {
                let ads =
                    bits.string(upid_length as usize, "SegmentationUPIDType::ADSInformation")?;
                Ok(Self::ADSInformation(ads))
            }
            SegmentationUPIDType::URI => {
                let uri = bits.string(upid_length as usize, "SegmentationUPIDType::URI")?;
                Ok(Self::URI(uri))
            }
            SegmentationUPIDType::UUID => {
                validate(upid_length, 16, upid_type)?;
                let uuid = bits.string(16, "SegmentationUPIDType::UUID")?;
                Ok(Self::UUID(uuid))
            }
        }
    }
}

fn validate(
    upid_length: u8,
    expected_length: u8,
    upid_type: SegmentationUPIDType,
) -> Result<(), ParseError> {
    if upid_length != expected_length {
        Err(ParseError::UnexpectedSegmentationUPIDLength {
            declared_segmentation_upid_length: upid_length,
            expected_segmentation_upid_length: expected_length,
            segmentation_upid_type: upid_type,
        })
    } else {
        Ok(())
    }
}

impl ManagedPrivateUPID {
    fn try_from(bits: &mut Bits, upid_length: u8) -> Result<Self, ParseError> {
        if upid_length < 4 {
            return Err(ParseError::InvalidMPUInSegmentationUPID { upid_length });
        }
        let private_data_length = upid_length - 4;
        let format_specifier = bits.string(4, "ManagedPrivateUPID")?;
        let mut private_data = vec![];
        for _ in 0..private_data_length {
            private_data.push(bits.byte());
        }
        Ok(Self {
            format_specifier,
            private_data,
        })
    }
}

enum HyphenSeparatedCheckedHexVersion {
    DeprecatedISAN,
    VersionedISAN,
    Eidr,
}

struct HyphenSeparatedCheckedHex {
    version: HyphenSeparatedCheckedHexVersion,
}

impl HyphenSeparatedCheckedHex {
    fn read(&self, bits: &mut Bits) -> String {
        let (check_indices, index_max) = match self.version {
            HyphenSeparatedCheckedHexVersion::DeprecatedISAN => (vec![4], 4),
            HyphenSeparatedCheckedHexVersion::VersionedISAN => (vec![4, 7], 7),
            HyphenSeparatedCheckedHexVersion::Eidr => (vec![5], 5),
        };
        let mut sections = vec![];
        for i in 0..=index_max {
            if check_indices.contains(&i) {
                sections.push(check_char(&sections).to_string());
            } else {
                let mut s = String::with_capacity(4);
                write!(&mut s, "{:04x}", bits.u16(16)).unwrap();
                sections.push(s.to_uppercase());
            }
        }
        sections.join("-")
    }
}

const CHAR_ARRAY: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// The check calculation is taken from isan_check_digit_calculation_v2.0.pdf included
// in the repository.
fn check_char(isan: &[String]) -> char {
    let isan: Vec<String> = isan
        .iter()
        .filter(|s| s.chars().count() > 1)
        .cloned()
        .collect();
    let adjusted_product = isan.join("").chars().fold(36, |adjusted_sum, char| {
        let decimal_value = u8::from_str_radix(&char.to_string(), 16)
            .expect("Should be safe as all non-check chars in ISAN array are hexadecimal.");
        let mut sum = adjusted_sum + decimal_value;
        if sum > 36 {
            sum -= 36;
        }
        let mut product = sum * 2;
        if product >= 37 {
            product -= 37;
        }
        product
    });
    if adjusted_product == 1 {
        '0'
    } else {
        CHAR_ARRAY
            .get((37 - adjusted_product) as usize)
            .unwrap()
            .clone()
    }
}
