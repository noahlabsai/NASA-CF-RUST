use crate::cf_verify::*;
use crate::cf_clist::*;
use crate::cf_assert::*;
use crate::cf_test_utils::*;
use crate::cf_cfdp::*;

#[cfg(test)]
mod stubs {
    use super::*;
    use std::ffi::c_void;

    #[derive(Debug, Default)]
    pub struct CF_CList_InsertBack_context_t {
        pub head: Option<*mut *mut CF_CListNode_t>,
        pub node: Option<*mut CF_CListNode_t>,
    }

    #[derive(Debug, Default)]
    pub struct CF_CList_Pop_context_t {
        pub head: Option<*mut *mut CF_CListNode_t>,
        pub forced_return: *mut CF_CListNode_t,
    }

    #[derive(Debug, Default)]
    pub struct CF_CList_Remove_context_t {
        pub head: Option<*mut *mut CF_CListNode_t>,
        pub node: Option<*mut CF_CListNode_t>,
    }

    #[derive(Debug, Default)]
    pub struct CF_CList_InsertAfter_context_t {
        pub head: Option<*mut *mut CF_CListNode_t>,
        pub start: Option<*mut CF_CListNode_t>,
        pub after: Option<*mut CF_CListNode_t>,
    }

    #[derive(Debug, Default)]
    pub struct CF_CList_Traverse_R_context_t {
        pub end: Option<*mut CF_CListNode_t>,
        pub fn_: Option<CF_CListFn_t>,
        pub context_t: Option<*mut CF_Transaction_t>,
    }

    pub fn ut_default_handler_cf_clist_init_node(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<*mut CF_CListNode_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                *ctxt_ptr = ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "node");
            }
        }
    }

    pub fn ut_default_handler_cf_clist_insert_back(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<CF_CList_InsertBack_context_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                (*ctxt_ptr).head = Some(ut_hook_get_arg_value_by_name::<*mut *mut CF_CListNode_t>(context, "head"));
                (*ctxt_ptr).node = Some(ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "node"));
            }
        }
    }

    pub fn ut_default_handler_cf_clist_cf_clist_pop(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<CF_CList_Pop_context_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                (*ctxt_ptr).head = Some(ut_hook_get_arg_value_by_name::<*mut *mut CF_CListNode_t>(context, "head"));
                ut_stub_set_return_value(func_key, (*ctxt_ptr).forced_return as usize);
            }
        }
    }

    pub fn ut_default_handler_cf_clist_remove(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<CF_CList_Remove_context_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                (*ctxt_ptr).head = Some(ut_hook_get_arg_value_by_name::<*mut *mut CF_CListNode_t>(context, "head"));
                (*ctxt_ptr).node = Some(ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "node"));
            }
        }
    }

    pub fn ut_default_handler_cf_clist_insert_after(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<CF_CList_InsertAfter_context_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                (*ctxt_ptr).head = Some(ut_hook_get_arg_value_by_name::<*mut *mut CF_CListNode_t>(context, "head"));
                (*ctxt_ptr).start = Some(ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "start"));
                (*ctxt_ptr).after = Some(ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "after"));
            }
        }
    }

    pub fn ut_default_handler_cf_clist_traverse(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<*mut c_void>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                *ctxt_ptr = ut_hook_get_arg_value_by_name::<*mut c_void>(context, "context");
            }
        }
    }

    pub fn ut_default_handler_cf_clist_traverse_r(
        user_obj: *mut c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        unsafe {
            let ctxt = ut_cf_get_context_buffer::<CF_CList_Traverse_R_context_t>(func_key);
            if let Some(ctxt_ptr) = ctxt {
                (*ctxt_ptr).end = Some(ut_hook_get_arg_value_by_name::<*mut CF_CListNode_t>(context, "end"));
                (*ctxt_ptr).fn_ = Some(ut_hook_get_arg_value_by_name::<CF_CListFn_t>(context, "fn"));
                (*ctxt_ptr).context_t = Some(ut_hook_get_arg_value_by_name::<*mut CF_Transaction_t>(context, "context"));
            }
        }
    }
}