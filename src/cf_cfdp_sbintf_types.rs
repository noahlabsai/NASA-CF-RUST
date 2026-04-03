//! CF CFDP Software Bus Interface type definitions.
//!
//! Translated from: cf_cfdp_sbintf.h (type definitions only)

use crate::common_types::*;
use crate::cf_cfdp_pdu::*;

/// PDU command encapsulation structure
///
/// Encapsulates a CFDP PDU into a format sent/received over the
/// software bus, adding "command" encapsulation.
///
/// Note: this is only the definition of the header. In reality all messages are
/// larger than this, up to CF_MAX_PDU_SIZE.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_PduCmdMsg_t {
    /// software bus headers, not really used by CF
    pub hdr: CFE_MSG_CommandHeader_t,
    /// Beginning of CFDP headers
    pub ph: CF_CFDP_PduHeader_t,
}

/// PDU telemetry encapsulation structure
///
/// Encapsulates a CFDP PDU into a format sent/received over the
/// software bus, adding "telemetry" encapsulation.
///
/// Note: this is only the definition of the header. In reality all messages are
/// larger than this, up to CF_MAX_PDU_SIZE.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_PduTlmMsg_t {
    /// software bus headers, not really used by CF
    pub hdr: CFE_MSG_TelemetryHeader_t,
    /// Beginning of CFDP headers
    pub ph: CF_CFDP_PduHeader_t,
}
