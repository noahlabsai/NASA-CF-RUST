//! CF Application utility type definitions.
//!
//! Translated from: cf_utils.h (type definitions only)

use crate::common_types::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_types::*;
use crate::cf_clist_types::*;

/// Argument structure for use with CList_Traverse()
///
/// This identifies a specific transaction sequence number and entity ID.
/// The transaction pointer is set by the implementation.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Traverse_TransSeqArg_t {
    pub transaction_sequence_number: CF_TransactionSeq_t,
    pub src_eid: CF_EntityId_t,
    pub txn: *mut CF_Transaction_t,
}

impl Default for CF_Traverse_TransSeqArg_t {
    fn default() -> Self {
        Self {
            transaction_sequence_number: 0,
            src_eid: 0,
            txn: core::ptr::null_mut(),
        }
    }
}

/// Argument structure for use with CF_Traverse_WriteHistoryQueueEntryToFile()
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Traverse_WriteHistoryFileArg_t {
    pub fd: osal_id_t,
    pub filter_dir: CF_Direction_t,
    pub error: bool,
    pub counter: u32,
}

/// Argument structure for use with CF_Traverse_WriteTxnQueueEntryToFile()
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Traverse_WriteTxnFileArg_t {
    pub fd: osal_id_t,
    pub error: bool,
    pub counter: u32,
}

/// Callback function type for use with CF_TraverseAllTransactions()
pub type CF_TraverseAllTransactions_fn_t = Option<fn(txn: *mut CF_Transaction_t, context: *mut u8)>;

/// Argument structure for use with CF_TraverseAllTransactions()
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_TraverseAll_Arg_t {
    pub fn_callback: CF_TraverseAllTransactions_fn_t,
    pub context: *mut u8,
    pub counter: i32,
}

impl Default for CF_TraverseAll_Arg_t {
    fn default() -> Self {
        Self {
            fn_callback: None,
            context: core::ptr::null_mut(),
            counter: 0,
        }
    }
}

/// Argument structure for priority-sorted insertion
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Traverse_PriorityArg_t {
    pub txn: *mut CF_Transaction_t,
    pub priority: u8,
}

impl Default for CF_Traverse_PriorityArg_t {
    fn default() -> Self {
        Self {
            txn: core::ptr::null_mut(),
            priority: 0,
        }
    }
}
