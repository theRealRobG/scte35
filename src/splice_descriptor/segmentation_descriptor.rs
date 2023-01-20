use crate::atsc::ATSCContentIdentifier;

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
#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
    /// `adID`)
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

#[derive(PartialEq, Eq)]
pub struct ManagedPrivateUPID {
    pub format_specifier: String,
    pub private_data: Vec<u8>,
}
