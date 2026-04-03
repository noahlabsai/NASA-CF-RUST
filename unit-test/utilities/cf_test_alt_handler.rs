use std::ptr;

/* UT includes */
use crate::uttest::*;
use crate::utassert::*;
use crate::utstubs::*;

use crate::cf_test_utils::*;
use crate::cf_test_alt_handler::*;
use crate::cf_utils::*;

/*----------------------------------------------------------------
 *
 * A handler for CF_CList_Traverse which saves its arguments
 * including the opaque context pointer as a CF_TraverseAll_Arg_t object.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_cf_clist_traverse_traverse_all_args_t(
    user_obj: Option<*mut std::ffi::c_void>,
    func_key: UT_EntryKey_t,
    context: &UT_StubContext_t,
) {
    let ctxt: *mut CF_CList_Traverse_TRAVERSE_ALL_ARGS_T_context_t;
    let arg = ut_hook_get_arg_value_by_name::<*mut CF_TraverseAll_Arg_t>(context, "context");

    if let Some(user_obj_ptr) = user_obj {
        ctxt = user_obj_ptr as *mut CF_CList_Traverse_TRAVERSE_ALL_ARGS_T_context_t;
    } else {
        ctxt = ut_cf_get_context_buffer::<CF_CList_Traverse_TRAVERSE_ALL_ARGS_T_context_t>(func_key);
    }

    /* the counter seems to be an output */
    if !arg.is_null() {
        unsafe {
            (*arg).counter += 1;
        }
    }

    if !ctxt.is_null() {
        unsafe {
            (*ctxt).start = ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "start");
            (*ctxt).fn_ = ut_hook_get_arg_value_by_name::<CF_CListFn_t>(context, "fn");
            if !arg.is_null() {
                (*ctxt).context_fn = (*arg).fn_;
                (*ctxt).context_counter = (*arg).counter;
                (*ctxt).context_context = (*arg).context;
            }
        }
    }
}

/*----------------------------------------------------------------
 *
 * A handler for CF_CList_Traverse which saves its arguments
 * to a CF_CList_Traverse_POINTER_context_t object.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_cf_clist_traverse_pointer(
    user_obj: Option<*mut std::ffi::c_void>,
    func_key: UT_EntryKey_t,
    context: &UT_StubContext_t,
) {
    let ctxt: *mut CF_CList_Traverse_POINTER_context_t;

    if let Some(user_obj_ptr) = user_obj {
        ctxt = user_obj_ptr as *mut CF_CList_Traverse_POINTER_context_t;
    } else {
        ctxt = ut_cf_get_context_buffer::<CF_CList_Traverse_POINTER_context_t>(func_key);
    }

    if !ctxt.is_null() {
        unsafe {
            (*ctxt).start = ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "start");
            (*ctxt).fn_ = ut_hook_get_arg_value_by_name::<CF_CListFn_t>(context, "fn");
            (*ctxt).context = ut_hook_get_arg_value_by_name::<*mut std::ffi::c_void>(context, "context");
        }
    }
}

/*----------------------------------------------------------------
 *
 * A handler for CF_CList_Traverse which saves its arguments
 * including the opaque context pointer as a CF_Traverse_PriorityArg_t object.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_cf_clist_traverse_r_prio(
    user_obj: Option<*mut std::ffi::c_void>,
    func_key: UT_EntryKey_t,
    context: &UT_StubContext_t,
) {
    let ctxt: *mut CF_CList_Traverse_R_context_t;
    let arg = ut_hook_get_arg_value_by_name::<*mut CF_Traverse_PriorityArg_t>(context, "context");

    if let Some(user_obj_ptr) = user_obj {
        ctxt = user_obj_ptr as *mut CF_CList_Traverse_R_context_t;
    } else {
        ctxt = ut_cf_get_context_buffer::<CF_CList_Traverse_R_context_t>(func_key);
    }

    if !ctxt.is_null() {
        unsafe {
            (*ctxt).end = ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "end");
            (*ctxt).fn_ = ut_hook_get_arg_value_by_name::<CF_CListFn_t>(context, "fn");

            /* This handler is a little different in that it sets the output to the caller */
            if !arg.is_null() {
                (*arg).txn = (*ctxt).context_t;
            }
        }
    }
}

/*----------------------------------------------------------------
 *
 * A handler for CF_TraverseAllTransactions which _sets_ the opaque context
 * pointer as an int* object.  The value is taken from the UserObj opaque pointer.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_cf_traverse_all_transactions_all_channels_set_context(
    user_obj: Option<*mut std::ffi::c_void>,
    func_key: UT_EntryKey_t,
    context: &UT_StubContext_t,
) {
    let call_context = ut_hook_get_arg_value_by_name::<*mut i32>(context, "context");
    let req_context = user_obj.unwrap() as *mut i32;
    let forced_return: i32;

    unsafe {
        *call_context = *req_context;
    }
    forced_return = -1;

    ut_stub_set_return_value(func_key, forced_return as isize);
}

/*----------------------------------------------------------------
 *
 * A simple handler that can be used for any stub that returns a pointer.
 * it just forces the return value to be the object passed in as UserObj.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_generic_pointer_return(
    user_obj: Option<*mut std::ffi::c_void>,
    func_key: UT_EntryKey_t,
    _context: &UT_StubContext_t,
) {
    ut_stub_set_return_value(func_key, user_obj.unwrap_or(ptr::null_mut()) as isize);
}

/*----------------------------------------------------------------
 *
 * Function: UT_AltHandler_CaptureTransactionStatus
 *
 * A handler for CF_CFDP_SetTxnStatus() and similar that captures the CF_TxnStatus_t
 * value to the supplied storage location.
 *
 *-----------------------------------------------------------------*/
pub fn ut_alt_handler_capture_transaction_status(
    user_obj: Option<*mut std::ffi::c_void>,
    _func_key: UT_EntryKey_t,
    context: &UT_StubContext_t,
) {
    let p_txn_stat = user_obj.unwrap() as *mut CF_TxnStatus_t;
    let in_stat = ut_hook_get_arg_value_by_name::<CF_TxnStatus_t>(context, "txn_stat");

    unsafe {
        *p_txn_stat = in_stat;
    }
}