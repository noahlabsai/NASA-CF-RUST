use crate::cf_app::*;
use crate::cf_verify::*;
use crate::cf_cfdp::*;
use crate::cf_utils::*;
use crate::cf_eventids::*;
use crate::cf_perfids::*;
use crate::cf_assert::*;
use crate::cf_test_utils::*;

#[cfg(test)]
mod stubs {
    use super::*;

    pub fn ut_default_handler_cf_reset_history(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_CFDP_ResetHistory_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.chan = ut_hook_get_arg_value_by_name::<*mut CF_Channel_t>(context, "chan");
            ctxt.history = ut_hook_get_arg_value_by_name::<*mut CF_History_t>(context, "history");
        }
    }

    pub fn ut_default_handler_cf_find_transaction_by_sequence_number(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_FindTransactionBySequenceNumber_context_t>(func_key);
        let forced_return;

        if let Some(ctxt) = ctxt {
            ctxt.chan = ut_hook_get_arg_value_by_name::<*mut CF_Channel_t>(context, "chan");
            ctxt.transaction_sequence_number = ut_hook_get_arg_value_by_name::<CF_TransactionSeq_t>(context, "transaction_sequence_number");
            ctxt.src_eid = ut_hook_get_arg_value_by_name::<CF_EntityId_t>(context, "src_eid");

            forced_return = ctxt.forced_return;
        } else {
            forced_return = std::ptr::null_mut();
        }

        ut_stub_set_return_value(func_key, forced_return as *mut std::ffi::c_void);
    }

    pub fn ut_default_handler_cf_find_unused_transaction(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let forced_return = std::ptr::null_mut::<CF_Transaction_t>();

        ut_stub_set_return_value(func_key, forced_return as *mut std::ffi::c_void);
    }

    pub fn ut_default_handler_cf_write_txn_queue_data_to_file(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_WriteTxnQueueDataToFile_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.fd = ut_hook_get_arg_value_by_name::<i32>(context, "fd");
            ctxt.chan = ut_hook_get_arg_value_by_name::<*mut CF_Channel_t>(context, "chan");
            ctxt.queue = ut_hook_get_arg_value_by_name::<CF_QueueIdx_t>(context, "queue");
        }
    }

    pub fn ut_default_handler_cf_write_history_queue_data_to_file(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_WriteHistoryQueueDataToFile_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.fd = ut_hook_get_arg_value_by_name::<i32>(context, "fd");
            ctxt.chan = ut_hook_get_arg_value_by_name::<*mut CF_Channel_t>(context, "chan");
            ctxt.dir = ut_hook_get_arg_value_by_name::<CF_Direction_t>(context, "dir");
        }
    }

    pub fn ut_default_handler_cf_traverse_all_transactions(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_TraverseAllTransactions_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.chan = ut_hook_get_arg_value_by_name::<*mut CF_Channel_t>(context, "chan");
            ctxt.fn_ptr = ut_hook_get_arg_value_by_name::<CF_TraverseAllTransactions_fn_t>(context, "fn");
            ctxt.context = ut_hook_get_arg_value_by_name::<*mut std::ffi::c_void>(context, "context");
        }
    }

    pub fn ut_default_handler_cf_traverse_all_transactions_all_channels(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_TraverseAllTransactions_All_Channels_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.fn_ptr = ut_hook_get_arg_value_by_name::<CF_TraverseAllTransactions_fn_t>(context, "fn");
            ctxt.context = ut_hook_get_arg_value_by_name::<*mut std::ffi::c_void>(context, "context");

            ut_stub_set_return_value(func_key, ctxt.forced_return as *mut std::ffi::c_void);
        }
    }

    pub fn ut_default_handler_cf_wrapped_open_create(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CF_WrappedOpenCreate_context_t>(func_key);

        if let Some(ctxt) = ctxt {
            ctxt.fd = ut_hook_get_arg_value_by_name::<*mut osal_id_t>(context, "fd");
            ctxt.fname = ut_hook_get_arg_value_by_name::<*const std::ffi::c_char>(context, "fname");
            ctxt.flags = ut_hook_get_arg_value_by_name::<i32>(context, "flags");
            ctxt.access = ut_hook_get_arg_value_by_name::<i32>(context, "access");

            ut_stub_set_return_value(func_key, ctxt.forced_return as *mut std::ffi::c_void);
        }
    }

    pub fn ut_default_handler_cf_txn_status_is_error(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let result = unsafe { (*context).int32_status_code != 0 };

        ut_stub_set_return_value(func_key, result as *mut std::ffi::c_void);
    }
}