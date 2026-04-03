use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

// Assuming these types are defined elsewhere in the codebase
use crate::{
    CF_CFDP_AckTxnStatus_t, CF_Transaction_t, CF_Channel_t, CF_TransactionSeq_t, CF_EntityId_t,
    CFE_Status_t, CF_CListNode_t, CF_Traverse_TransSeqArg_t, CF_Direction_t, CF_QueueIdx_t,
    CF_CListTraverse_Status_t, CF_History_t, CF_TraverseAllTransactions_fn_t, CF_TxnStatus_t,
    CF_CFDP_ConditionCode_t, osal_id_t, off_t
};

// Mock stub framework types
#[derive(Debug, Clone, Copy)]
pub struct UT_EntryKey_t(pub u32);

#[derive(Debug)]
pub struct UT_StubContext_t {
    // Mock context fields
}

// Default handler function type
type UT_DefaultHandler = fn(*mut c_void, UT_EntryKey_t, *const UT_StubContext_t);

#[cfg(test)]
mod cf_utils_stubs {
    use super::*;

    // Default handler declarations
    pub fn ut_default_handler_cf_find_transaction_by_sequence_number(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_find_unused_transaction(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_reset_history(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_traverse_all_transactions(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_traverse_all_transactions_all_channels(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_txn_status_is_error(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_wrapped_open_create(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_write_history_queue_data_to_file(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn ut_default_handler_cf_write_txn_queue_data_to_file(
        _: *mut c_void,
        _: UT_EntryKey_t,
        _: *const UT_StubContext_t,
    ) {
    }

    pub fn cf_cfdp_get_ack_txn_status(_txn: *mut CF_Transaction_t) -> CF_CFDP_AckTxnStatus_t {
        Default::default()
    }

    pub fn cf_find_transaction_by_sequence_number(
        _chan: *mut CF_Channel_t,
        _transaction_sequence_number: CF_TransactionSeq_t,
        _src_eid: CF_EntityId_t,
    ) -> *mut CF_Transaction_t {
        std::ptr::null_mut()
    }

    pub fn cf_find_transaction_by_sequence_number_impl(
        _node: *mut CF_CListNode_t,
        _context: *mut CF_Traverse_TransSeqArg_t,
    ) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_find_unused_transaction(
        _chan: *mut CF_Channel_t,
        _direction: CF_Direction_t,
    ) -> *mut CF_Transaction_t {
        std::ptr::null_mut()
    }

    pub fn cf_free_transaction(_txn: *mut CF_Transaction_t, _chan: u8) {
        // Stub implementation - no operation
    }

    pub fn cf_get_channel_from_txn(_txn: *mut CF_Transaction_t) -> *mut CF_Channel_t {
        std::ptr::null_mut()
    }

    pub fn cf_get_chunk_list_head(
        _chan: *mut CF_Channel_t,
        _direction: u8,
    ) -> *mut *mut CF_CListNode_t {
        std::ptr::null_mut()
    }

    pub fn cf_insert_sort_prio(_txn: *mut CF_Transaction_t, _queue: CF_QueueIdx_t) {
        // Stub implementation - no operation
    }

    pub fn cf_prio_search(
        _node: *mut CF_CListNode_t,
        _context: *mut c_void,
    ) -> CF_CListTraverse_Status_t {
        Default::default()
    }

    pub fn cf_reset_history(_chan: *mut CF_Channel_t, _history: *mut CF_History_t) {
        // Stub implementation - no operation
    }

    pub fn cf_traverse_all_transactions(
        _chan: *mut CF_Channel_t,
        _fn: CF_TraverseAllTransactions_fn_t,
        _context: *mut c_void,
    ) -> i32 {
        0
    }

    pub fn cf_traverse_all_transactions_all_channels(
        _fn: CF_TraverseAllTransactions_fn_t,
        _context: *mut c_void,
    ) -> i32 {
        0
    }

    pub fn cf_traverse_all_transactions_impl(
        _node: *mut CF_CListNode_t,
        _arg: *mut c_void,
    ) -> CF_CListTraverse_Status_t {
        Default::default()
    }

    pub fn cf_traverse_write_history_queue_entry_to_file(
        _node: *mut CF_CListNode_t,
        _arg: *mut c_void,
    ) -> CF_CListTraverse_Status_t {
        Default::default()
    }

    pub fn cf_traverse_write_txn_queue_entry_to_file(
        _node: *mut CF_CListNode_t,
        _arg: *mut c_void,
    ) -> CF_CListTraverse_Status_t {
        Default::default()
    }

    pub fn cf_txn_status_from_condition_code(_cc: CF_CFDP_ConditionCode_t) -> CF_TxnStatus_t {
        Default::default()
    }

    pub fn cf_txn_status_to_condition_code(_txn_stat: CF_TxnStatus_t) -> CF_CFDP_ConditionCode_t {
        Default::default()
    }

    pub fn cf_wrapped_close(_fd: osal_id_t) {
        // Stub implementation - no operation
    }

    pub fn cf_wrapped_lseek(_fd: osal_id_t, _offset: off_t, _mode: c_int) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_wrapped_open_create(
        _fd: *mut osal_id_t,
        _fname: *const c_char,
        _flags: i32,
        _access: i32,
    ) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_wrapped_read(_fd: osal_id_t, _buf: *mut c_void, _read_size: usize) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_wrapped_write(
        _fd: osal_id_t,
        _buf: *const c_void,
        _write_size: usize,
    ) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_write_history_entry_to_file(
        _fd: osal_id_t,
        _history: *const CF_History_t,
    ) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_write_history_queue_data_to_file(
        _fd: osal_id_t,
        _chan: *mut CF_Channel_t,
        _dir: CF_Direction_t,
    ) -> CFE_Status_t {
        Default::default()
    }

    pub fn cf_write_txn_queue_data_to_file(
        _fd: osal_id_t,
        _chan: *mut CF_Channel_t,
        _queue: CF_QueueIdx_t,
    ) -> CFE_Status_t {
        Default::default()
    }
}