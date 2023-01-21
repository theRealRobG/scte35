/// The `PrivateCommand` structure provides a means to distribute user-defined commands using the
/// SCTE 35 protocol. The first bit field in each user-defined command is a 32-bit identifier,
/// unique for each participating vendor. Receiving equipment should skip any `SpliceInfoSection`
/// messages containing `PrivateCommand` structures with unknown identifiers.
/**
```
// private_command() {
//   identifier           32 uimsbf
//   for(i=0; i<N; i++) {
//     private_byte        8 uimsbf
//   }
// }
```
*/
#[derive(PartialEq, Eq, Debug)]
pub struct PrivateCommand {
    /// This 32-bit number is used to identify the owner of the command.
    ///
    /// The identifier is a 32-bit field as defined in [ITU-T H.222.0]. Refer to clauses 2.6.8 and
    /// 2.6.9 of [ITU-T H.222.0] for descriptions of Registration descriptor and semantic
    /// definition of fields in registration descriptor. Only identifier values registered and
    /// recognized by SMPTE Registration Authority, LLC should be used (see [b-SMPTE RA]). Its use
    /// in the `PrivateCommand` structure shall scope and identify only the private information
    /// contained within this command.
    pub identifier: String,
    /// The remainder of the descriptor is dedicated to data fields as required by the descriptor
    /// being defined.
    pub private_bytes: Vec<u8>,
}
