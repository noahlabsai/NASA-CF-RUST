//! Type declarations for CF CFDP Receive-File (R) transaction handlers.
//!
//! This corresponds to cf_cfdp_r.h in the C source.
//! The actual function implementations are in cf_cfdp_r.rs.

use crate::cf_cfdp_types::CF_Transaction_t;
use crate::cf_logical_pdu::CF_Logical_PduNak_t;

/// Argument for Gap Compute function.
///
/// This is used in conjunction with `CF_CFDP_R2_GapCompute`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_GapComputeArgs_t {
    /// Current transaction being processed
    pub txn: *mut CF_Transaction_t,
    /// Current NAK PDU contents
    pub nak: *mut CF_Logical_PduNak_t,
}

impl Default for CF_GapComputeArgs_t {
    fn default() -> Self {
        Self {
            txn: core::ptr::null_mut(),
            nak: core::ptr::null_mut(),
        }
    }
}
