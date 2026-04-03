//! CFDP PDU (Protocol Data Unit) structure definitions.
//!
//! Translated from: cf_cfdp_pdu.h
//!
//! These structures define the on-wire binary format of CFDP PDUs
//! per CCSDS 727.0-B-5. They use wrapper structs with byte arrays
//! to ensure packed/unaligned representation.

use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;

use core::mem::size_of;

// =====================================================================
// Encoded integer wrapper types (packed, unaligned)
// =====================================================================

/// Encoded 8-bit value in the CFDP PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_uint8_t {
    pub octets: [u8; 1],
}

/// Encoded 16-bit value in the CFDP PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_uint16_t {
    pub octets: [u8; 2],
}

/// Encoded 32-bit value in the CFDP PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_uint32_t {
    pub octets: [u8; 4],
}

/// Encoded 64-bit value in the CFDP PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_uint64_t {
    pub octets: [u8; 8],
}

// =====================================================================
// Header size constants
// =====================================================================

/// Maximum encoded size of a CFDP PDU header (8 bytes per variable item)
pub const CF_CFDP_MAX_HEADER_SIZE: usize =
    size_of::<CF_CFDP_PduHeader_t>() + (3 * size_of::<CF_CFDP_uint64_t>());

/// Minimum encoded size of a CFDP PDU header (1 byte per variable item)
pub const CF_CFDP_MIN_HEADER_SIZE: usize =
    size_of::<CF_CFDP_PduHeader_t>() + (3 * size_of::<CF_CFDP_uint8_t>());

/// Maximum encoded size of a CFDP PDU that this implementation can accept
pub const CF_APP_MAX_HEADER_SIZE: usize =
    size_of::<CF_CFDP_PduHeader_t>()
        + size_of::<CF_TransactionSeq_t>()
        + (2 * size_of::<CF_EntityId_t>());

// =====================================================================
// PDU header structures
// =====================================================================

/// Base CFDP PDU header (fixed portion only)
///
/// Defined per section 5.1 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduHeader_t {
    pub flags: CF_CFDP_uint8_t,
    pub length: CF_CFDP_uint16_t,
    pub eid_tsn_lengths: CF_CFDP_uint8_t,
    // variable-length data follows (at least 3 additional bytes)
}

/// File Directive Header
///
/// Defined per section 5.2 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduFileDirectiveHeader_t {
    pub directive_code: CF_CFDP_uint8_t,
}

/// LV (Length + Value) object format
///
/// Defined per table 5-2 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_lv_t {
    pub length: CF_CFDP_uint8_t,
}

/// TLV (Type + Length + Value) object format
///
/// Defined per table 5-3 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_tlv_t {
    pub r#type: CF_CFDP_uint8_t,
    pub length: CF_CFDP_uint8_t,
}

// =====================================================================
// TLV type values
// =====================================================================

/// Values for "type" field of TLV structure
///
/// Defined per section 5.4 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_TlvType_t {
    CF_CFDP_TLV_TYPE_FILESTORE_REQUEST      = 0,
    CF_CFDP_TLV_TYPE_FILESTORE_RESPONSE     = 1,
    CF_CFDP_TLV_TYPE_MESSAGE_TO_USER        = 2,
    CF_CFDP_TLV_TYPE_FAULT_HANDLER_OVERRIDE = 4,
    CF_CFDP_TLV_TYPE_FLOW_LABEL             = 5,
    CF_CFDP_TLV_TYPE_ENTITY_ID              = 6,
    CF_CFDP_TLV_TYPE_INVALID_MAX            = 7,
}

impl Default for CF_CFDP_TlvType_t {
    fn default() -> Self {
        Self::CF_CFDP_TLV_TYPE_INVALID_MAX
    }
}

// =====================================================================
// File directive codes
// =====================================================================

/// Values for "directive_code" within CF_CFDP_PduFileDirectiveHeader_t
///
/// Defined per table 5-4 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_FileDirective_t {
    CF_CFDP_FileDirective_INVALID_MIN = 0,
    CF_CFDP_FileDirective_EOF         = 4,
    CF_CFDP_FileDirective_FIN         = 5,
    CF_CFDP_FileDirective_ACK         = 6,
    CF_CFDP_FileDirective_METADATA    = 7,
    CF_CFDP_FileDirective_NAK         = 8,
    CF_CFDP_FileDirective_PROMPT      = 9,
    CF_CFDP_FileDirective_KEEP_ALIVE  = 12,
    CF_CFDP_FileDirective_INVALID_MAX = 13,
}

impl Default for CF_CFDP_FileDirective_t {
    fn default() -> Self {
        Self::CF_CFDP_FileDirective_INVALID_MIN
    }
}

// =====================================================================
// ACK transaction status
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_AckTxnStatus_t {
    CF_CFDP_AckTxnStatus_UNDEFINED    = 0,
    CF_CFDP_AckTxnStatus_ACTIVE       = 1,
    CF_CFDP_AckTxnStatus_TERMINATED   = 2,
    CF_CFDP_AckTxnStatus_UNRECOGNIZED = 3,
    CF_CFDP_AckTxnStatus_INVALID      = 4,
}

impl Default for CF_CFDP_AckTxnStatus_t {
    fn default() -> Self {
        Self::CF_CFDP_AckTxnStatus_UNDEFINED
    }
}

// =====================================================================
// FIN delivery code
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_FinDeliveryCode_t {
    CF_CFDP_FinDeliveryCode_COMPLETE   = 0,
    CF_CFDP_FinDeliveryCode_INCOMPLETE = 1,
    CF_CFDP_FinDeliveryCode_INVALID    = 2,
}

impl Default for CF_CFDP_FinDeliveryCode_t {
    fn default() -> Self {
        Self::CF_CFDP_FinDeliveryCode_COMPLETE
    }
}

// =====================================================================
// FIN file status
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_FinFileStatus_t {
    CF_CFDP_FinFileStatus_DISCARDED           = 0,
    CF_CFDP_FinFileStatus_DISCARDED_FILESTORE = 1,
    CF_CFDP_FinFileStatus_RETAINED            = 2,
    CF_CFDP_FinFileStatus_UNREPORTED          = 3,
    CF_CFDP_FinFileStatus_INVALID             = 4,
}

impl Default for CF_CFDP_FinFileStatus_t {
    fn default() -> Self {
        Self::CF_CFDP_FinFileStatus_UNREPORTED
    }
}

// =====================================================================
// Condition codes
// =====================================================================

/// Condition code values
///
/// Defined per table 5-5 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_CFDP_ConditionCode_t {
    CF_CFDP_ConditionCode_NO_ERROR                  = 0,
    CF_CFDP_ConditionCode_POS_ACK_LIMIT_REACHED     = 1,
    CF_CFDP_ConditionCode_KEEP_ALIVE_LIMIT_REACHED  = 2,
    CF_CFDP_ConditionCode_INVALID_TRANSMISSION_MODE = 3,
    CF_CFDP_ConditionCode_FILESTORE_REJECTION       = 4,
    CF_CFDP_ConditionCode_FILE_CHECKSUM_FAILURE     = 5,
    CF_CFDP_ConditionCode_FILE_SIZE_ERROR           = 6,
    CF_CFDP_ConditionCode_NAK_LIMIT_REACHED         = 7,
    CF_CFDP_ConditionCode_INACTIVITY_DETECTED       = 8,
    CF_CFDP_ConditionCode_INVALID_FILE_STRUCTURE     = 9,
    CF_CFDP_ConditionCode_CHECK_LIMIT_REACHED       = 10,
    CF_CFDP_ConditionCode_UNSUPPORTED_CHECKSUM_TYPE = 11,
    CF_CFDP_ConditionCode_SUSPEND_REQUEST_RECEIVED  = 14,
    CF_CFDP_ConditionCode_CANCEL_REQUEST_RECEIVED   = 15,
}

impl Default for CF_CFDP_ConditionCode_t {
    fn default() -> Self {
        Self::CF_CFDP_ConditionCode_NO_ERROR
    }
}

// =====================================================================
// PDU-specific structures
// =====================================================================

/// End of file PDU
///
/// Defined per section 5.2.2 / table 5-6 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduEof_t {
    pub cc: CF_CFDP_uint8_t,
    pub crc: CF_CFDP_uint32_t,
    pub size: CF_CFDP_uint32_t,
}

/// Finished PDU
///
/// Defined per section 5.2.3 / table 5-7 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduFin_t {
    pub flags: CF_CFDP_uint8_t,
}

/// Acknowledge PDU
///
/// Defined per section 5.2.4 / table 5-8 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduAck_t {
    pub directive_and_subtype_code: CF_CFDP_uint8_t,
    pub cc_and_transaction_status: CF_CFDP_uint8_t,
}

/// Segment Request
///
/// Defined per section 5.2.6 / table 5-11 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_SegmentRequest_t {
    pub offset_start: CF_CFDP_uint32_t,
    pub offset_end: CF_CFDP_uint32_t,
}

/// Non-Acknowledge PDU
///
/// Defined per section 5.2.6 / table 5-10 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduNak_t {
    pub scope_start: CF_CFDP_uint32_t,
    pub scope_end: CF_CFDP_uint32_t,
}

/// Metadata PDU
///
/// Defined per section 5.2.5 / table 5-9 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduMd_t {
    pub segmentation_control: CF_CFDP_uint8_t,
    pub size: CF_CFDP_uint32_t,
}

/// File Data header
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CFDP_PduFileDataHeader_t {
    pub offset: CF_CFDP_uint32_t,
}

/// File Data content — maximum possible data block
///
/// Size = CF_MAX_PDU_SIZE - sizeof(CF_CFDP_PduFileDataHeader_t) - CF_CFDP_MIN_HEADER_SIZE
pub const CF_CFDP_FILE_DATA_MAX_LEN: usize =
    CF_MAX_PDU_SIZE - size_of::<CF_CFDP_PduFileDataHeader_t>() - CF_CFDP_MIN_HEADER_SIZE;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_CFDP_PduFileDataContent_t {
    pub data: [u8; CF_CFDP_FILE_DATA_MAX_LEN],
}

impl Default for CF_CFDP_PduFileDataContent_t {
    fn default() -> Self {
        Self {
            data: [0u8; CF_CFDP_FILE_DATA_MAX_LEN],
        }
    }
}
