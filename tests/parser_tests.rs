use base64::prelude::*;
use pretty_assertions::assert_eq;
use scte35::{
    atsc::ATSCContentIdentifier,
    error::ParseError,
    splice_command::{
        splice_insert::{self, SpliceInsert},
        time_signal::TimeSignal,
        SpliceCommand, SpliceCommandType,
    },
    splice_descriptor::{
        avail_descriptor::AvailDescriptor,
        dtmf_descriptor::DTMFDescriptor,
        segmentation_descriptor::{
            self, DeliveryRestrictions, DeviceRestrictions, ManagedPrivateUPID,
            SegmentationDescriptor, SegmentationTypeID, SegmentationUPID, SegmentationUPIDType,
        },
        SpliceDescriptor,
    },
    splice_info_section::{SAPType, SpliceInfoSection},
    time::{BreakDuration, SpliceTime},
};

// MARK: - SCTE-35 2020 - 14. Sample SCTE 35 Messages (Informative)

// 14.1. time_signal – Placement Opportunity Start
#[test]
fn test_time_signal_placement_opportunity_start() {
    let base64_string =
        "/DA0AAAAAAAA///wBQb+cr0AUAAeAhxDVUVJSAAAjn/PAAGlmbAICAAAAAAsoKGKNAIAmsnRfg==";
    let hex_string = "0xFC3034000000000000FFFFF00506FE72BD0050001E021C435545494800008E7FCF0001A599B00808000000002CA0A18A3402009AC9D17E";
    let expected_splice_info_section = SpliceInfoSection {
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
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
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
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.2. splice_insert
#[test]
fn test_splice_insert() {
    let base64_string = "/DAvAAAAAAAA///wFAVIAACPf+/+c2nALv4AUsz1AAAAAAAKAAhDVUVJAAABNWLbowo=";
    let hex_string = "0xFC302F000000000000FFFFF014054800008F7FEFFE7369C02EFE0052CCF500000000000A0008435545490000013562DBA30A";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 1207959695,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: true,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(1936310318),
                        }),
                    },
                ),
                break_duration: Some(BreakDuration {
                    auto_return: true,
                    duration: 5426421,
                }),
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![SpliceDescriptor::AvailDescriptor(AvailDescriptor {
            identifier: 1129661769,
            provider_avail_id: 309,
        })],
        crc_32: 0x62DBA30A,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.3. time_signal – Placement Opportunity End
#[test]
fn test_time_signal_placement_opportunity_end() {
    let base64_string = "/DAvAAAAAAAA///wBQb+dGKQoAAZAhdDVUVJSAAAjn+fCAgAAAAALKChijUCAKnMZ1g=";
    let hex_string = "0xFC302F000000000000FFFFF00506FE746290A000190217435545494800008E7F9F0808000000002CA0A18A350200A9CC6758";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(1952616608),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959694,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA0A18A")),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityEnd,
                    segment_num: 2,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xA9CC6758,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.4. time_signal – Program Start/End
#[test]
fn test_time_signal_program_start_end() {
    let base64_string = "/DBIAAAAAAAA///wBQb+ek2ItgAyAhdDVUVJSAAAGH+fCAgAAAAALMvDRBEAAAIXQ1VFSUgAABl/nwgIAAAAACyk26AQAACZcuND";
    let hex_string = "0xFC3048000000000000FFFFF00506FE7A4D88B60032021743554549480000187F9F0808000000002CCBC344110000021743554549480000197F9F0808000000002CA4DBA01000009972E343";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(2051901622),
            },
        }),
        splice_descriptors: vec![
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959576,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CCBC344")),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959577,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA4DBA0")),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
        ],
        crc_32: 0x9972E343,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.5. time_signal – Program Overlap Start
#[test]
fn test_time_signal_program_overlap_start() {
    let base64_string = "/DAvAAAAAAAA///wBQb+rr//ZAAZAhdDVUVJSAAACH+fCAgAAAAALKVs9RcAAJUdsKg=";
    let hex_string = "0xFC302F000000000000FFFFF00506FEAEBFFF640019021743554549480000087F9F0808000000002CA56CF5170000951DB0A8";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(2931818340),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959560,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA56CF5")),
                    segmentation_type_id: SegmentationTypeID::ProgramOverlapStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x951DB0A8,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.6. time_signal – Program Blackout Override / Program End
#[test]
fn test_time_signal_program_blackoutoverride_program_end() {
    let base64_string = "/DBIAAAAAAAA///wBQb+ky44CwAyAhdDVUVJSAAACn+fCAgAAAAALKCh4xgAAAIXQ1VFSUgAAAl/nwgIAAAAACygoYoRAAC0IX6w";
    let hex_string = "0xFC3048000000000000FFFFF00506FE932E380B00320217435545494800000A7F9F0808000000002CA0A1E3180000021743554549480000097F9F0808000000002CA0A18A110000B4217EB0";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(2469279755),
            },
        }),
        splice_descriptors: vec![
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959562,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA0A1E3")),
                    segmentation_type_id: SegmentationTypeID::ProgramBlackoutOverride,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959561,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA0A18A")),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
        ],
        crc_32: 0xB4217EB0,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.7. time_signal – Program End
#[test]
fn test_time_signal_program_end() {
    let base64_string = "/DAvAAAAAAAA///wBQb+rvF8TAAZAhdDVUVJSAAAB3+fCAgAAAAALKVslxEAAMSHai4=";
    let hex_string = "0xFC302F000000000000FFFFF00506FEAEF17C4C0019021743554549480000077F9F0808000000002CA56C97110000C4876A2E";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(2935061580),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959559,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CA56C97")),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xC4876A2E,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// 14.8. time_signal – Program Start/End - Placement Opportunity End
#[test]
fn test_time_signal_program_start_end_placement_opportunity_end() {
    let base64_string = "/DBhAAAAAAAA///wBQb+qM1E7QBLAhdDVUVJSAAArX+fCAgAAAAALLLXnTUCAAIXQ1VFSUgAACZ/nwgIAAAAACyy150RAAACF0NVRUlIAAAnf58ICAAAAAAsstezEAAAihiGnw==";
    let hex_string = "0xFC3061000000000000FFFFF00506FEA8CD44ED004B021743554549480000AD7F9F0808000000002CB2D79D350200021743554549480000267F9F0808000000002CB2D79D110000021743554549480000277F9F0808000000002CB2D7B31000008A18869F";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(2832024813),
            },
        }),
        splice_descriptors: vec![
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959725,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CB2D79D")),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityEnd,
                    segment_num: 2,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959590,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CB2D79D")),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959591,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: true,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x000000002CB2D7B3")),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
        ],
        crc_32: 0x8A18869F,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// MARK: - Examples from https://openidconnectweb.azurewebsites.net/Cue

#[test]
fn test_time_signal_segmentation_descriptor_ad_id() {
    let base64_string =
        "/DA4AAAAAAAA///wBQb+AAAAAAAiAiBDVUVJAAAAA3//AAApPWwDDEFCQ0QwMTIzNDU2SBAAAGgCL9A=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2702700),
                    segmentation_upid: SegmentationUPID::AdID(String::from("ABCD0123456H")),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x68022FD0,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_umid() {
    let base64_string = "/DBHAAAAAAAA///wBQb+AAAAAAAxAi9DVUVJAAAAA3+/BCAGCis0AQEBBQEBDSATAAAA0skDbI8ZU0OrcBTS1xi/2hEAAPUV9+0=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::UMID(String::from(
                        "060A2B34.01010105.01010D20.13000000.D2C9036C.8F195343.AB7014D2.D718BFDA",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xF515F7ED,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_isan_program_start() {
    let base64_string =
        "/DA4AAAAAAAA///wBQb+AAAAAAAiAiBDVUVJAAAABn//AAApPWwGDAAAAAA6jQAAAAAAABAAAPaArb4=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 6,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2702700),
                    segmentation_upid: SegmentationUPID::ISAN(String::from(
                        "0000-0000-3A8D-0000-Z-0000-0000-6",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xF680ADBE,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_isan_program_end() {
    let base64_string = "/DAzAAAAAAAA///wBQb+AAAAAAAdAhtDVUVJAAAABn+/BgwAAAAAOo0AAAAAAAARAAAT5alN";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 6,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::ISAN(String::from(
                        "0000-0000-3A8D-0000-Z-0000-0000-6",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x13E5A94D,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_tid_program_start() {
    let base64_string =
        "/DA4AAAAAAAA///wBQb+AAAAAAAiAiBDVUVJAAAAA3//AAApPWwHDE1WMDAwNDE0NjQwMBAAAIH4Mwc=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2702700),
                    segmentation_upid: SegmentationUPID::TID(String::from("MV0004146400")),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x81F83307,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_tid_program_end() {
    let base64_string = "/DAzAAAAAAAA///wBQb+AAAAAAAdAhtDVUVJAAAAA3+/BwxNVjAwMDQxNDY0MDARAAB2a6fC";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TID(String::from("MV0004146400")),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x766BA7C2,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_adi_ppo_start() {
    let base64_string = "/DBLAAAAAAAA///wBQb+AAAAAAA1AjNDVUVJYgAFin//AABSZcAJH1NJR05BTDpEUjIxWjA3WlQ4YThhc25pdVVoZWlBPT00AADz3GdX";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1644168586,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(5400000),
                    segmentation_upid: SegmentationUPID::ADI(String::from(
                        "SIGNAL:DR21Z07ZT8a8asniuUheiA==",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xF3DC6757,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_adi_ppo_end() {
    let base64_string = "/DBEAAAAAAAA///wBQb+AFJlwAAuAixDVUVJYgAFin+/CR1TSUdOQUw6My1zUTROZ0ZUME9qUHNHNFdxVVFvdzUAAEukzlg=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(5400000),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1644168586,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::ADI(String::from(
                        "SIGNAL:3-sQ4NgFT0OjPsG4WqUQow",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x4BA4CE58,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_eidr_program_start() {
    let base64_string =
        "/DA4AAAAAAAA///wBQb+AAAAAAAiAiBDVUVJAAAAA3//AAApPWwKDBR4+FrhALBoW4+xyBAAAGij1lQ=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2702700),
                    segmentation_upid: SegmentationUPID::EIDR(String::from(
                        "10.5240/F85A-E100-B068-5B8F-B1C8-T",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x68A3D654,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_invalid_eidr() {
    let hex_string =
        "0xFC30280000000000000000700506FF1252E9220012021043554549000000007F9F0A013050000015871049";
    match SpliceInfoSection::try_from_hex_string(hex_string) {
        Ok(_) => panic!("Should have returned error but instead succeeded"),
        Err(e) => match e {
            ParseError::UnexpectedSegmentationUPIDLength {
                declared_segmentation_upid_length,
                expected_segmentation_upid_length,
                segmentation_upid_type,
            } => {
                assert_eq!(1, declared_segmentation_upid_length);
                assert_eq!(12, expected_segmentation_upid_length);
                assert_eq!(SegmentationUPIDType::EIDR, segmentation_upid_type);
            }
            _ => panic!("Should have returned UnexpectedSegmentationUPIDLength error"),
        },
    }
}

#[test]
fn test_time_signal_segmentation_descriptor_atsc_content_identifier_program_start() {
    let base64_string =
        "/DA4AAAAAAAA///wBQb+AAAAAAAiAiBDVUVJAAAAA3//AAApPWwLDADx7/9odW1hbjAxMhAAALdaWG4=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2702700),
                    segmentation_upid: SegmentationUPID::ATSCContentIdentifier(
                        ATSCContentIdentifier {
                            tsid: 241,
                            end_of_day: 23,
                            unique_for: 511,
                            content_id: String::from("human012"),
                        },
                    ),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xB75A586E,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_atsc_content_identifier_program_end() {
    let base64_string = "/DAzAAAAAAAA///wBQb+AAAAAAAdAhtDVUVJAAAAA3+/CwwA8e//aHVtYW4wMTIRAABAycyr";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 3,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::ATSCContentIdentifier(
                        ATSCContentIdentifier {
                            tsid: 241,
                            end_of_day: 23,
                            unique_for: 511,
                            content_id: String::from("human012"),
                        },
                    ),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x40C9CCAB,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_ti_mpu() {
    let base64_string = "/DB5AAAAAAAAAP/wBQb/DkfmpABjAhdDVUVJhPHPYH+/CAgAAAAABy4QajEBGAIcQ1VFSYTx71B//wAAK3NwCAgAAAAABy1cxzACGAIqQ1VFSYTx751/vwwbUlRMTjFIAQAAAAAxMzU2MTY2MjQ1NTUxQjEAAQAALL95dg==";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(4534560420),
            },
        }),
        splice_descriptors: vec![
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 2230439776,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::TI(String::from("0x00000000072E106A")),
                    segmentation_type_id: SegmentationTypeID::ProviderAdvertisementEnd,
                    segment_num: 1,
                    segments_expected: 24,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 2230447952,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2847600),
                    segmentation_upid: SegmentationUPID::TI(String::from("0x00000000072D5CC7")),
                    segmentation_type_id: SegmentationTypeID::ProviderAdvertisementStart,
                    segment_num: 2,
                    segments_expected: 24,
                    sub_segment: None,
                }),
            }),
            SpliceDescriptor::SegmentationDescriptor(SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 2230448029,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::MPU(ManagedPrivateUPID {
                        format_specifier: String::from("RTLN"),
                        private_data: BASE64_STANDARD
                            .decode("MUgBAAAAADEzNTYxNjYyNDU1NTFCMQA=")
                            .unwrap(),
                    }),
                    segmentation_type_id: SegmentationTypeID::ContentIdentification,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            }),
        ],
        crc_32: 0x2CBF7976,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_mid_ads_ti() {
    let base64_string =
        "/DA9AAAAAAAAAACABQb+0fha8wAnAiVDVUVJSAAAv3/PAAD4+mMNEQ4FTEEzMDkICAAAAAAuU4SBNAAAPIaCPw==";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0x8,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(3522714355),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 1207959743,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: false,
                        no_regional_blackout: true,
                        archive_allowed: true,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: Some(16317027),
                    segmentation_upid: SegmentationUPID::MID(vec![
                        SegmentationUPID::ADSInformation(String::from("LA309")),
                        SegmentationUPID::TI(String::from("0x000000002E538481")),
                    ]),
                    segmentation_type_id: SegmentationTypeID::ProviderPlacementOpportunityStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x3C86823F,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_ads_program_start() {
    let base64_string = "/DBZAAAAAAAA///wBQb+AAAAAABDAkFDVUVJAAAAC3//AAApMuAOLUFEUy1VUElEOmFhODViYmI2LTVjNDMtNGI2YS1iZWJiLWVlM2IxM2ViNzk5ORAAAJd2uP4=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 11,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2700000),
                    segmentation_upid: SegmentationUPID::ADSInformation(String::from(
                        "ADS-UPID:aa85bbb6-5c43-4b6a-bebb-ee3b13eb7999",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x9776B8FE,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_ads_program_end() {
    let base64_string = "/DBUAAAAAAAA///wBQb+AAAAAAA+AjxDVUVJAAAAC3+/Di1BRFMtVVBJRDphYTg1YmJiNi01YzQzLTRiNmEtYmViYi1lZTNiMTNlYjc5OTkRAACV15uV";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 11,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::ADSInformation(String::from(
                        "ADS-UPID:aa85bbb6-5c43-4b6a-bebb-ee3b13eb7999",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x95D79B95,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_uri_program_start() {
    let base64_string = "/DBZAAAAAAAA///wBQb+AAAAAABDAkFDVUVJAAAACn//AAApMuAPLXVybjp1dWlkOmFhODViYmI2LTVjNDMtNGI2YS1iZWJiLWVlM2IxM2ViNzk5ORAAAFz7UQA=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 10,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(2700000),
                    segmentation_upid: SegmentationUPID::URI(String::from(
                        "urn:uuid:aa85bbb6-5c43-4b6a-bebb-ee3b13eb7999",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x5CFB5100,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_segmentation_descriptor_uri_program_end() {
    let base64_string = "/DBUAAAAAAAA///wBQb+AAAAAAA+AjxDVUVJAAAACn+/Dy11cm46dXVpZDphYTg1YmJiNi01YzQzLTRiNmEtYmViYi1lZTNiMTNlYjc5OTkRAAB2c6LA";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime { pts_time: Some(0) },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 10,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: None,
                    segmentation_upid: SegmentationUPID::URI(String::from(
                        "urn:uuid:aa85bbb6-5c43-4b6a-bebb-ee3b13eb7999",
                    )),
                    segmentation_type_id: SegmentationTypeID::ProgramEnd,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0x7673A2C0,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_splice_insert_avail_descriptor_hex() {
    let hex_string = "0xFC302F000000000000FFFFF014054800008F7FEFFE7369C02EFE0052CCF500000000000A0008435545490000013562DBA30A";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 1207959695,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: true,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(1936310318),
                        }),
                    },
                ),
                break_duration: Some(BreakDuration {
                    auto_return: true,
                    duration: 5426421,
                }),
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![SpliceDescriptor::AvailDescriptor(AvailDescriptor {
            identifier: 1129661769,
            provider_avail_id: 309,
        })],
        crc_32: 0x62DBA30A,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

#[test]
fn test_splice_insert_avail_descriptor_base64() {
    let base64_string = "/DAvAAAAAAAAAP///wViAAWKf+//CXVCAv4AUmXAAzUAAAAKAAhDVUVJADgyMWLvc/g=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 1644168586,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: true,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(4453646850),
                        }),
                    },
                ),
                break_duration: Some(BreakDuration {
                    auto_return: true,
                    duration: 5400000,
                }),
                unique_program_id: 821,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![SpliceDescriptor::AvailDescriptor(AvailDescriptor {
            identifier: 1129661769,
            provider_avail_id: 3682865,
        })],
        crc_32: 0x62EF73F8,
        non_fatal_errors: vec![ParseError::UnexpectedSpliceCommandLength {
            declared_splice_command_length_in_bits: 32760,
            actual_splice_command_length_in_bits: 160,
            splice_command_type: SpliceCommandType::SpliceInsert,
        }],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_splice_insert_hex() {
    let hex_string = "0xFC302100000000000000FFF01005000003DB7FEF7F7E0020F580C0000000000019913DA5";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 987,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: true,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime { pts_time: None }),
                    },
                ),
                break_duration: Some(BreakDuration {
                    auto_return: false,
                    duration: 2160000,
                }),
                unique_program_id: 49152,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![],
        crc_32: 0x19913DA5,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

#[test]
fn test_splice_insert_hex_with_no0x() {
    let hex_string = "fc302000000000000000fff00f0500000fa07f4ffe1faf4e1400000000000061bd0585";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 4000,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: false,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(531582484),
                        }),
                    },
                ),
                break_duration: None,
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![],
        crc_32: 0x61BD0585,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

#[test]
fn test_splice_insert_out() {
    let base64_string = "/DAlAAAAAAAAAP/wFAUAAAPvf+//adb6P/4AUmXAAAAAAAAAoeikig==";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 1007,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: true,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(6070663743),
                        }),
                    },
                ),
                break_duration: Some(BreakDuration {
                    auto_return: true,
                    duration: 5400000,
                }),
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![],
        crc_32: 0xA1E8A48A,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_splice_insert_in() {
    let base64_string = "/DAgAAAAAAAAAP/wDwUAAAPvf0//ahTGjwAAAAAAALda4HI=";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 1007,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: false,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(6074713743),
                        }),
                    },
                ),
                break_duration: None,
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![],
        crc_32: 0xB75AE072,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

// Example taken from https://github.com/futzu/threefive/blob/441ba290854f0ddc7baccc7350e25ee8148665cd/examples/dtmf/Dtmf_Descriptor.py
#[test]
fn test_dtmf_with_alignment_stuffing() {
    let base64_string = "/DAsAAAAAAAAAP/wDwUAAABef0/+zPACTQAAAAAADAEKQ1VFSbGfMTIxIxGolm3/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceInsert(SpliceInsert {
            event_id: 94,
            scheduled_event: Some(splice_insert::ScheduledEvent {
                out_of_network_indicator: false,
                is_immediate_splice: false,
                splice_mode: splice_insert::SpliceMode::ProgramSpliceMode(
                    splice_insert::ProgramMode {
                        splice_time: Some(SpliceTime {
                            pts_time: Some(3438281293),
                        }),
                    },
                ),
                break_duration: None,
                unique_program_id: 0,
                avail_num: 0,
                avails_expected: 0,
            }),
        }),
        splice_descriptors: vec![SpliceDescriptor::DTMFDescriptor(DTMFDescriptor {
            identifier: 1129661769,
            preroll: 177,
            dtmf_chars: String::from("121#"),
        })],
        crc_32: 0xFFFFFFFF,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

// Example taken from https://github.com/futzu/threefive/blob/8025c0f7df31a4a4f7649cb68a4b4f0e560b73f5/examples/splicenull/Splice_Null.cue
#[test]
fn test_splice_null() {
    let hex_string = "0xFC301100000000000000FFFFFF0000004F253396";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::SpliceNull,
        splice_descriptors: vec![],
        crc_32: 0x4F253396,
        non_fatal_errors: vec![ParseError::UnexpectedSpliceCommandLength {
            declared_splice_command_length_in_bits: 32760,
            actual_splice_command_length_in_bits: 0,
            splice_command_type: SpliceCommandType::SpliceNull,
        }],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}

// MARK: - Further examples

#[test]
fn test_time_signal_segmentation_descriptor_mid() {
    let base64_string = "/DBwAAAAAAAAAP/wBQb/AAAAAABaAlhDVUVJAAAAAn//AABSZcANRAoMFHeL5eP2AAAAAAAACgwUd4vl4/YAAAAAAAAJJlNJR05BTDpMeTlFTUd4S1IwaEZaVXRwTUhkQ1VWWm5SVUZuWnowNgEB1Dao2g==";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(4294967296),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 2,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: None,
                    component_segments: None,
                    segmentation_duration: Some(5400000),
                    segmentation_upid: SegmentationUPID::MID(vec![
                        // TODO - EIDR DOI suffix is not always ISAN, as demonstrated here.
                        // It may be worth creating a struct for the EIDR so as not to force
                        // an unexpected format (the below examples should be "10.5239/8BE5-E3F6").
                        SegmentationUPID::EIDR(String::from("10.5239/8BE5-E3F6-0000-0000-0000-B")),
                        SegmentationUPID::EIDR(String::from("10.5239/8BE5-E3F6-0000-0000-0000-B")),
                        SegmentationUPID::ADI(String::from(
                            "SIGNAL:Ly9EMGxKR0hFZUtpMHdCUVZnRUFnZz0",
                        )),
                    ]),
                    segmentation_type_id: SegmentationTypeID::DistributorPlacementOpportunityStart,
                    segment_num: 1,
                    segments_expected: 1,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xD436A8DA,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_bytes(
            &BASE64_STANDARD
                .decode(base64_string)
                .expect("should be valid base64")
        )
        .expect("should be valid splice info section from base64"),
        "unexpected splice info section from base64"
    );
}

#[test]
fn test_time_signal_provider_ad_start_mpu() {
    let hex_string = "0xFC309100000000000000FFF00506FF63EE6B06007B027943554549000000647FC30000F735E10C654E4243557B2261737365744964223A22706561636F636B5F363030313131222C2263756544617461223A7B2263756554797065223A227374616E646172645F627265616B222C226B6579223A227062222C2276616C7565223A227374616E64617264227D7D300000A9C80D12";
    let expected_splice_info_section = SpliceInfoSection {
        table_id: 252,
        sap_type: SAPType::Unspecified,
        protocol_version: 0,
        encrypted_packet: None,
        pts_adjustment: 0,
        tier: 0xFFF,
        splice_command: SpliceCommand::TimeSignal(TimeSignal {
            splice_time: SpliceTime {
                pts_time: Some(5971536646),
            },
        }),
        splice_descriptors: vec![SpliceDescriptor::SegmentationDescriptor(
            SegmentationDescriptor {
                identifier: 1129661769,
                event_id: 100,
                scheduled_event: Some(segmentation_descriptor::ScheduledEvent {
                    delivery_restrictions: Some(DeliveryRestrictions {
                        web_delivery_allowed: false,
                        no_regional_blackout: false,
                        archive_allowed: false,
                        device_restrictions: DeviceRestrictions::None,
                    }),
                    component_segments: None,
                    segmentation_duration: Some(16201185),
                    segmentation_upid: SegmentationUPID::MPU(ManagedPrivateUPID {
                        format_specifier: String::from("NBCU"),
                        private_data: BASE64_STANDARD.decode("eyJhc3NldElkIjoicGVhY29ja182MDAxMTEiLCJjdWVEYXRhIjp7ImN1ZVR5cGUiOiJzdGFuZGFyZF9icmVhayIsImtleSI6InBiIiwidmFsdWUiOiJzdGFuZGFyZCJ9fQ==").unwrap(),
                    }),
                    segmentation_type_id: SegmentationTypeID::ProviderAdvertisementStart,
                    segment_num: 0,
                    segments_expected: 0,
                    sub_segment: None,
                }),
            },
        )],
        crc_32: 0xA9C80D12,
        non_fatal_errors: vec![],
    };
    assert_eq!(
        &expected_splice_info_section,
        &SpliceInfoSection::try_from_hex_string(hex_string)
            .expect("should be valid splice info section from hex"),
        "unexpected splice info section from hex"
    );
}
