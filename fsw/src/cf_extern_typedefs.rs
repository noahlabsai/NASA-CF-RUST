//! CF Application external type definitions.
//!
//! Translated from: default_cf_extern_typedefs.h
//!
//! These types are part of the CF public interface and are used
//! in commands, telemetry, and table definitions.

use crate::cf_platform_cfg::*;

/// Values for CFDP file transfer class.
///
/// Defined per section 7.1 of CCSDS 727.0-B-5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CF_CFDP_Class_t {
    /// CFDP class 1 - Unreliable transfer
    #[default]
    CF_CFDP_CLASS_1 = 0,
    /// CFDP class 2 - Reliable transfer
    CF_CFDP_CLASS_2 = 1,
}

/// CF queue identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CF_QueueIdx_t {
    CF_QueueIdx_PEND      = 0,
    CF_QueueIdx_TX        = 1,
    CF_QueueIdx_RX        = 2,
    CF_QueueIdx_HIST      = 3,
    CF_QueueIdx_HIST_FREE = 4,
    #[default]
    CF_QueueIdx_FREE      = 5,
}

/// Number of queue indices
pub const CF_QueueIdx_NUM: usize = 6;

/// Cache of source and destination filename
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_TxnFilenames_t {
    pub src_filename: [u8; CF_FILENAME_MAX_LEN],
    pub dst_filename: [u8; CF_FILENAME_MAX_LEN],
}

impl Default for CF_TxnFilenames_t {
    fn default() -> Self {
        Self {
            src_filename: [0u8; CF_FILENAME_MAX_LEN],
            dst_filename: [0u8; CF_FILENAME_MAX_LEN],
        }
    }
}

/// Entity id size — must be one of u8, u16, u32, u64
pub type CF_EntityId_t = u32;

/// Transaction sequence number size — must be one of u8, u16, u32, u64
pub type CF_TransactionSeq_t = u32;
