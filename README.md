# scte35
scte35 is a SCTE35 parser that has been translated from Swift into Rust using my [SCTE35Parser](https:github.com/theRealRobG/SCTE35Parser) implementation as a base.

The 2020 SCTE-25 specification was used and can be found included in the Swift implementation repository.

Not all data fields from the SCTE35 message are included in the output struct; where the field is informational for parsing instruction only it is ommitted from inclusion in the public model.

## Usage
The primary integration point to the framework is the [`SpliceInfoSection`](./src/splice_info_section.rs#L53-L112) struct.

Given a hex encoded string SCTE35 message, the `SpliceInfoSection` offers the `try_from_hex_string` that returns `Result<SpliceInfoSection, ParseError>`:
```rs
use scte35::{
    splice_command::{time_signal::TimeSignal, SpliceCommand},
    splice_descriptor::{
        segmentation_descriptor::{
            DeliveryRestrictions, DeviceRestrictions, ScheduledEvent,
            SegmentationDescriptor, SegmentationTypeID, SegmentationUPID,
        },
        SpliceDescriptor,
    },
    splice_info_section::{SAPType, SpliceInfoSection},
    time::SpliceTime,
};

let hex_string = "0xFC3034000000000000FFFFF00506FE72BD0050001E021C435545494800008E7FCF0001A599B00808000000002CA0A18A3402009AC9D17E";
let splice_info_section = SpliceInfoSection::try_from_hex_string(hex_string).unwrap();
assert_eq!(
    SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(1924989008),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959694,
                scheduled_event: Some(ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: false,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: Some(27630000),
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA0A18A")),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityStart,
                    segment_num: 2,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x9AC9D17E,
        non_fatal_errors: vec![],
    },
    splice_info_section
);
```

Errors can be returned if there are some issues with the provided SCTE35 message that invalidate the parsing.

The parser also keeps a storage of `non_fatal_errors`. The idea here is that there may be some inconsistencies in the SCTE35 message (e.g. mis-match between declared `SpliceCommand` length and parsed length), but the message on the whole is still parsable, and so instead of killing the whole parse by throwing, the error is just logged to the `non_fatal_errors` instead.

There is also an initialiser provided for bytes `&[u8]`. This method can be used when you have a base64 string instead of hex by converting the string to bytes first, as the example below shows:
```rs
use base64::prelude::*;
use scte35::{
    splice_command::{time_signal::TimeSignal, SpliceCommand},
    splice_descriptor::{
        segmentation_descriptor::{
            DeliveryRestrictions, DeviceRestrictions, ScheduledEvent,
            SegmentationDescriptor, SegmentationTypeID, SegmentationUPID,
        },
        SpliceDescriptor,
    },
    splice_info_section::{SAPType, SpliceInfoSection},
    time::SpliceTime,
};

let base64_string = "/DA0AAAAAAAA///wBQb+cr0AUAAeAhxDVUVJSAAAjn/PAAGlmbAICAAAAAAsoKGKNAIAmsnRfg==";
let data = BASE64_STANDARD.decode(base64_string).expect("should be valid base64");
let splice_info_section = SpliceInfoSection::try_from_bytes(&data).unwrap();
assert_eq!(
    SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(1924989008),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959694,
                scheduled_event: Some(ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: false,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: Some(27630000),
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA0A18A")),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityStart,
                    segment_num: 2,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x9AC9D17E,
        non_fatal_errors: vec![],
    },
    splice_info_section
);
```

Both constructors are valid and should yield similar results:
```rs
use base64::prelude::*;
use scte35::splice_info_section::SpliceInfoSection;

let base64_string = "/DA0AAAAAAAA///wBQb+cr0AUAAeAhxDVUVJSAAAjn/PAAGlmbAICAAAAAAsoKGKNAIAmsnRfg==";
let base64_data = BASE64_STANDARD.decode(base64_string).expect("should be valid base64");
let hex_string = "0xFC3034000000000000FFFFF00506FE72BD0050001E021C435545494800008E7FCF0001A599B00808000000002CA0A18A3402009AC9D17E";
let splice_info_section_from_base64 = SpliceInfoSection::try_from_bytes(&base64_data).unwrap();
let splice_info_section_from_hex = SpliceInfoSection::try_from_hex_string(hex_string).unwrap();
assert_eq!(splice_info_section_from_base64, splice_info_section_from_hex);
```
