/// CRC state object — matches C struct CF_Crc field-for-field.
///
/// Fields:
///   working: accumulator for the current 4-byte word being assembled
///   result:  running CRC sum
///   index:   byte position within the current 4-byte word (0..3)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Crc {
    pub working: u32,
    pub result: u32,
    pub index: u8,
}

/// Type alias matching the C typedef `CF_Crc_t`.
pub type CF_Crc_t = CF_Crc;