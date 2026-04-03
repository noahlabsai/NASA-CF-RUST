//! Logical CFDP PDU structures.
//!
//! Translated from: cf_logical_pdu.h
//!
//! These are CF-specific data structures that reflect the logical
//! content of the CFDP PDUs. They are NOT the bitwise on-wire structures,
//! but rather the decoded/native values for use by software.

use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;

/// Maximum number of TLV values in a single PDU
pub const CF_PDU_MAX_TLV: usize = 4;

/// Maximum number of segment requests in a single PDU
pub const CF_PDU_MAX_SEGMENTS: usize = CF_NAK_MAX_SEGMENTS;

/// Type for logical file size/offset value
pub type CF_FileSize_t = u32;

// =====================================================================
// Logical PDU header
// =====================================================================

/// Logical representation of the base CFDP PDU header
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduHeader_t {
    pub version: u8,
    pub pdu_type: u8,
    pub direction: u8,
    pub txm_mode: u8,
    pub crc_flag: u8,
    pub large_flag: u8,
    pub segmentation_control: u8,
    pub eid_length: u8,
    pub segment_meta_flag: u8,
    pub txn_seq_length: u8,
    pub header_encoded_length: u16,
    pub data_encoded_length: u16,
    pub source_eid: CF_EntityId_t,
    pub destination_eid: CF_EntityId_t,
    pub sequence_num: CF_TransactionSeq_t,
}

/// Logical file directive header
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduFileDirectiveHeader_t {
    pub directive_code: CF_CFDP_FileDirective_t,
}

// =====================================================================
// LV, TLV, Segment structures
// =====================================================================

/// Logical LV (Length + Value) pair
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Logical_Lv_t {
    pub length: u8,
    pub data_ptr: *const u8,
}

impl Default for CF_Logical_Lv_t {
    fn default() -> Self {
        Self {
            length: 0,
            data_ptr: core::ptr::null(),
        }
    }
}

/// Union of various data items that may occur in a TLV item
#[derive(Clone, Copy)]
#[repr(C)]
pub union CF_Logical_TlvData_t {
    pub eid: CF_EntityId_t,
    pub data_ptr: *const u8,
}

impl Default for CF_Logical_TlvData_t {
    fn default() -> Self {
        Self { eid: 0 }
    }
}

impl core::fmt::Debug for CF_Logical_TlvData_t {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CF_Logical_TlvData_t {{ eid: {} }}", unsafe { self.eid })
    }
}

/// Logical TLV (Type + Length + Value) tuple
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_Tlv_t {
    pub r#type: CF_CFDP_TlvType_t,
    pub length: u8,
    pub data: CF_Logical_TlvData_t,
}

/// Logical Segment Request
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_SegmentRequest_t {
    pub offset_start: CF_FileSize_t,
    pub offset_end: CF_FileSize_t,
}

/// List of segment requests
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Logical_SegmentList_t {
    pub num_segments: u8,
    pub segments: [CF_Logical_SegmentRequest_t; CF_PDU_MAX_SEGMENTS],
}

impl Default for CF_Logical_SegmentList_t {
    fn default() -> Self {
        Self {
            num_segments: 0,
            segments: [CF_Logical_SegmentRequest_t::default(); CF_PDU_MAX_SEGMENTS],
        }
    }
}

/// List of TLV entries
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Logical_TlvList_t {
    pub num_tlv: u8,
    pub tlv: [CF_Logical_Tlv_t; CF_PDU_MAX_TLV],
}

impl Default for CF_Logical_TlvList_t {
    fn default() -> Self {
        Self {
            num_tlv: 0,
            tlv: [CF_Logical_Tlv_t::default(); CF_PDU_MAX_TLV],
        }
    }
}

// =====================================================================
// Logical PDU type-specific structures
// =====================================================================

/// Logical End of File PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduEof_t {
    pub cc: CF_CFDP_ConditionCode_t,
    pub crc: u32,
    pub size: CF_FileSize_t,
    pub tlv_list: CF_Logical_TlvList_t,
}

/// Logical Finished PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduFin_t {
    pub cc: CF_CFDP_ConditionCode_t,
    pub file_status: CF_CFDP_FinFileStatus_t,
    pub delivery_code: u8,
    pub tlv_list: CF_Logical_TlvList_t,
}

/// Logical Acknowledge PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduAck_t {
    pub ack_directive_code: u8,
    pub ack_subtype_code: u8,
    pub cc: CF_CFDP_ConditionCode_t,
    pub txn_status: CF_CFDP_AckTxnStatus_t,
}

/// Logical Metadata PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduMd_t {
    pub close_req: u8,
    pub checksum_type: u8,
    pub size: CF_FileSize_t,
    pub source_filename: CF_Logical_Lv_t,
    pub dest_filename: CF_Logical_Lv_t,
}

/// Logical Non-Acknowledge PDU
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduNak_t {
    pub scope_start: CF_FileSize_t,
    pub scope_end: CF_FileSize_t,
    pub segment_list: CF_Logical_SegmentList_t,
}

/// Logical File Data header
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Logical_PduFileDataHeader_t {
    pub continuation_state: u8,
    pub segment_list: CF_Logical_SegmentList_t,
    pub offset: CF_FileSize_t,
    pub data_ptr: *const u8,
    pub data_len: usize,
}

// =====================================================================
// Union of all internal header types
// =====================================================================

/// Union of all possible internal header types in a PDU
#[derive(Clone, Copy)]
#[repr(C)]
pub union CF_Logical_IntHeader_t {
    pub eof: CF_Logical_PduEof_t,
    pub fin: CF_Logical_PduFin_t,
    pub ack: CF_Logical_PduAck_t,
    pub md: CF_Logical_PduMd_t,
    pub nak: CF_Logical_PduNak_t,
    pub fd: CF_Logical_PduFileDataHeader_t,
}

impl Default for CF_Logical_IntHeader_t {
    fn default() -> Self {
        // Zero-initialize (safe because all variants are valid when zeroed)
        unsafe { core::mem::zeroed() }
    }
}

impl core::fmt::Debug for CF_Logical_IntHeader_t {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CF_Logical_IntHeader_t {{ ... }}")
    }
}

// =====================================================================
// Top-level PDU buffer
// =====================================================================

/// Encapsulates the entire PDU information
#[derive(Debug, Default)]
#[repr(C)]
pub struct CF_Logical_PduBuffer_t {
    /// Encoder state pointer (set during transmit)
    pub penc: *mut crate::cf_codec_types::CF_EncoderState_t,
    /// Decoder state pointer (set during receive)
    pub pdec: *mut crate::cf_codec_types::CF_DecoderState_t,

    /// Data in PDU header (applicable to all packets)
    pub pdu_header: CF_Logical_PduHeader_t,

    /// File directive header (pdu_type=0 only)
    pub fdirective: CF_Logical_PduFileDirectiveHeader_t,

    /// Internal header (union of all possible types)
    pub int_header: CF_Logical_IntHeader_t,

    /// Content CRC (if present)
    pub content_crc: u32,
}
