use crate::{
    error::ParseError, hex, splice_command::SpliceCommand, splice_descriptor::SpliceDescriptor,
};

/// The `SpliceInfoSection` shall be carried in transport packets whereby only one section or
/// partial section may be in any transport packet. `SpliceInfoSection`s shall always start at the
/// beginning of a transport packet payload.
/**
 ```
 // {
 //   table_id                         8 uimsbf
 //   section_syntax_indicator         1 bslbf
 //   private_indicator                1 bslbf
 //   sap_type                         2 bslbf
 //   section_length                  12 uimsbf
 //   protocol_version                 8 uimsbf
 //   encrypted_packet                 1 bslbf
 //   encryption_algorithm             6 uimsbf
 //   pts_adjustment                  33 uimsbf
 //   cw_index                         8 uimsbf
 //   tier                            12 bslbf
 //   splice_command_length           12 uimsbf
 //   splice_command_type              8 uimsbf E
 //   if(splice_command_type == 0x00)
 //     splice_null()                           E
 //   if(splice_command_type == 0x04)
 //     splice_schedule()                       E
 //   if(splice_command_type == 0x05)
 //     splice_insert()                         E
 //   if(splice_command_type == 0x06)
 //     time_signal()                           E
 //   if(splice_command_type == 0x07)
 //     bandwidth_reservation()                 E
 //   if(splice_command_type == 0xff)
 //     private_command()                       E
 //   descriptor_loop_length          16 uimsbf E
 //   for(i=0; i<N1; i++)
 //     splice_descriptor()                     E
 //   for(i=0; i<N2; i++)
 //     alignment_stuffing             8 bslbf  E
 //   if(encrypted_packet)
 //     E_CRC_32                      32 rpchof E
 //   CRC_32                          32 rpchof
 // }
*/
#[derive(PartialEq, Eq, Debug)]
pub struct SpliceInfoSection {
    /// This is an 8-bit field. Its value shall be 0xFC.
    pub table_id: u8,
    /// A two-bit field that indicates if the content preparation system has created a Stream
    /// Access Point (SAP) at the signaled point in the stream. SAP types are defined in ISO
    /// 14496-12, Annex I.
    pub sap_type: SAPType,
    /// An 8-bit unsigned integer field whose function is to allow, in the future, this table type
    /// to carry parameters that may be structured differently than those defined in the current
    /// protocol. At present, the only valid value for `protocol_version` is zero. Non-zero values
    /// of `protocol_version` may be used by a future version of this standard to indicate
    /// structurally different tables.
    pub protocol_version: u8,
    /// When this is set, it indicates that portions of the `SpliceInfoSection`, starting with
    /// `splice_command_type` and ending with and including `e_crc_32`, are encrypted. When this is
    /// not set, no part of this message is encrypted. The potentially encrypted portions of the
    /// `SpliceInfoTable` are indicated by an `E` in the Encrypted column of Table 5 (included in
    /// the doc-string for this `struct`).
    pub encrypted_packet: Option<EncryptedPacket>,
    /// A 33-bit unsigned integer that appears in the clear and that shall be used by a splicing
    /// device as an offset to be added to the (sometimes) encrypted `pts_time` field(s) throughout
    /// this message, to obtain the intended splice time(s). When this field has a zero value, then
    /// the `pts_time` field(s) shall be used without an offset. Normally, the creator of a cueing
    /// message will place a zero value into this field. This adjustment value is the means by
    /// which an upstream device, which restamps PCR/PTS/DTS, may convey to the splicing device the
    /// means by which to convert the `pts_time` field of the message to a newly imposed time
    /// domain.
    ///
    /// It is intended that the first device that restamps PCR/PTS/DTS and that passes the cueing
    /// message will insert a value into the `pts_adjustment` field, which is the delta time
    /// between this device’s input time domain and its output time domain. All subsequent devices,
    /// which also restamp PCR/PTS/DTS, may further alter the `pts_adjustment` field by adding
    /// their delta time to the field’s existing delta time and placing the result back in the
    /// `pts_adjustment` field. Upon each alteration of the `pts_adjustment` field, the altering
    /// device shall recalculate and update the `crc_32` field.
    ///
    /// The `pts_adjustment` shall, at all times, be the proper value to use for conversion of the
    /// `pts_time` field to the current time-base. The conversion is done by adding the two fields.
    /// In the presence of a wrap or overflow condition, the carry shall be ignored.
    pub pts_adjustment: u64,
    /// A 12-bit value used by the SCTE 35 message provider to assign messages to authorization
    /// tiers. This field may take any value between 0x000 and 0xFFF. The value of 0xFFF provides
    /// backwards compatibility and shall be ignored by downstream equipment. When using tier, the
    /// message provider should keep the entire message in a single transport stream packet.
    pub tier: u16,
    /// Information on the intention of this `SpliceInfoSection`.
    pub splice_command: SpliceCommand,
    /// Further descriptors in addition to the `splice_command`.
    pub splice_descriptors: Vec<SpliceDescriptor>,
    /// This is a 32-bit field that contains the CRC value that gives a zero output of the
    /// registers in the decoder defined in [MPEG Systems]after processing the entire
    /// `SpliceInfoSection`, which includes the `table_id` field through the `crc_32` field. The
    /// processing of `crc_32` shall occur prior to decryption of the encrypted fields and shall
    /// utilize the encrypted fields in their encrypted state.
    pub crc_32: u32,
    /// A list of errors that have not caused the message to be un-parsable, but are inconsistent
    /// with the specification. An example of this could be a splice command who's computed length
    /// after parsing did not match the indicated length of the command.
    pub non_fatal_errors: Vec<ParseError>,
}

impl SpliceInfoSection {
    /// Creates a `SpliceInfoSection` using the provided hex encoded string.
    pub fn from_hex_string(hex_string: &str) -> Result<SpliceInfoSection, ParseError> {
        let data = hex::decode_hex(hex_string)?;
        Self::from(data)
    }

    pub fn from(data: Vec<u8>) -> Result<SpliceInfoSection, ParseError> {
        todo!()
    }
}

/// A two-bit field that indicates if the content preparation system has created a Stream Access
/// Point (SAP) at the signaled point in the stream. SAP types are defined in ISO 14496-12, Annex
/// I.
#[derive(PartialEq, Eq, Debug)]
pub enum SAPType {
    /// Closed GOP with no leading pictures
    Type1,
    /// Closed GOP with leading pictures
    Type2,
    /// Open GOP
    Type3,
    /// The type of SAP, if any, is not signaled
    Unspecified,
}

impl SAPType {
    pub fn value(&self) -> u8 {
        match *self {
            SAPType::Type1 => 0x0,
            SAPType::Type2 => 0x1,
            SAPType::Type3 => 0x3,
            SAPType::Unspecified => 0x3,
        }
    }
}

/// This indicates that portions of the `SpliceInfoSection`, starting with `splice_command_type`
/// and ending with and including `e_crc_32`, are encrypted.
#[derive(PartialEq, Eq, Debug)]
pub struct EncryptedPacket {
    /// The `encryption_algorithm` field of the `SpliceInfoSection` is a 6-bit value. All Data
    /// Encryption Standard variants use a 64-bit key (actually 56 bits plus a checksum) to encrypt
    /// or decrypt a block of 8 bytes. In the case of triple DES, there will need to be 3 64-bit
    /// keys, one for each of the three passes of the DES algorithm. The “standard” triple DES
    /// actually uses two keys, where the first and third keys are identical.
    pub encryption_algorithm: Option<EncryptionAlgorithm>,
    /// An 8-bit unsigned integer that conveys which control word (key) is to be used to decrypt
    /// the message. The splicing device may store up to 256 keys previously provided for this
    /// purpose. When the `encrypted_packet` is `false`, this field is present but undefined.
    pub cw_index: u8,
    /// When encryption is used, this field is a function of the particular encryption algorithm
    /// chosen. Since some encryption algorithms require a specific length for the encrypted data,
    /// it is necessary to allow the insertion of stuffing bytes. For example, DES requires a
    /// multiple of 8 bytes be present in order to encrypt to the end of the packet. This allows
    /// standard DES to be used, as opposed to requiring a special version of the encryption
    /// algorithm.
    pub alignment_stuffing: u8,
    /// This is a 32-bit field that contains the CRC value that gives a zero output of the
    /// registers in the decoder defined in [MPEG Systems] after processing the entire decrypted
    /// portion of the `SpliceInfoSection`. This field is intended to give an indication that the
    /// decryption was performed successfully. Hence, the zero output is obtained following
    /// decryption and by processing the fields `SpliceCommandType` through `e_crc_32`.
    pub e_crc_32: u32,
}

/// The `encryption_algorithm` field of the `SpliceInfoSection` is a 6-bit value. All Data
/// Encryption Standard variants use a 64-bit key (actually 56 bits plus a checksum) to encrypt or
/// decrypt a block of 8 bytes. In the case of triple DES, there will need to be 3 64-bit keys, one
/// for each of the three passes of the DES algorithm. The “standard” triple DES actually uses two
/// keys, where the first and third keys are identical.
#[derive(PartialEq, Eq, Debug)]
pub enum EncryptionAlgorithm {
    /// No encryption
    NoEncryption,
    /// DES - ECB Mode
    DesEcbMode,
    /// DES - CBC Mode
    DesCbcMode,
    /// Triple DES EDE3 - ECB Mode
    TripleDes,
    /// User private
    UserPrivate(u8),
}

impl EncryptionAlgorithm {
    fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NoEncryption),
            1 => Some(Self::DesEcbMode),
            2 => Some(Self::DesCbcMode),
            3 => Some(Self::TripleDes),
            4..=31 => None,
            32..=63 => Some(Self::UserPrivate(value)),
            _ => None,
        }
    }
}
