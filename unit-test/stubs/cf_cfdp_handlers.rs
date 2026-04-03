/************************************************************************
 * NASA Docket No. GSC-18,447-1, and identified as "CFS CFDP (CF)
 * Application version 3.0.0"
 *
 * Copyright (c) 2019 United States Government as represented by the
 * Administrator of the National Aeronautics and Space Administration.
 * All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License. You may obtain
 * a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 ************************************************************************/

//! Stubs file for the CF Application main CFDP engine and PDU parsing file
//!
//! This file contains two sets of functions. The first is what is needed
//! to deal with CFDP PDUs. Specifically validating them for correctness
//! and ensuring the byte-order is correct for the target. The second
//! is incoming and outgoing CFDP PDUs pass through here. All receive
//! CFDP PDU logic is performed here and the data is passed to the
//! R (rx) and S (tx) logic.

#[cfg(test)]
mod cf_cfdp_stubs {
    use crate::cf_app::*;
    use crate::cf_cfdp::*;
    use crate::cf_test_utils::*;
    use crate::cf_utils::*;
    use crate::cf_verify::*;
    use crate::cfe::*;
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_void};
    use std::ptr;

    /// Default always returns NULL, an alt handler can be registered for other pointer returns
    pub extern "C" fn ut_default_handler_cf_cfdp_construct_pdu_header(
        user_obj: *mut c_void,
        func_key: UtEntryKey,
        context: *const UtStubContext,
    ) {
        let retval: *mut CfLogicalPduBuffer = ptr::null_mut();
        ut_stub_set_return_value(func_key, retval as *mut c_void);
    }

    /// For compatibility with other tests, this has a mechanism to save its
    /// arguments to a test-provided context capture buffer.
    pub extern "C" fn ut_default_handler_cf_cfdp_tx_file(
        user_obj: *mut c_void,
        func_key: UtEntryKey,
        context: *const UtStubContext,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CfCfdpTxFileContext>(func_key);
        
        if let Some(ctxt) = ctxt {
            if let Some(ptr) = ut_hook_get_arg_value_by_name::<*const c_char>(context, "src_filename") {
                let src_str = unsafe { CStr::from_ptr(ptr) };
                if let Ok(src_utf8) = src_str.to_str() {
                    let copy_len = std::cmp::min(src_utf8.len(), ctxt.src_filename.len() - 1);
                    ctxt.src_filename[..copy_len].copy_from_slice(src_utf8.as_bytes());
                    ctxt.src_filename[copy_len] = 0;
                }
            }
            
            if let Some(ptr) = ut_hook_get_arg_value_by_name::<*const c_char>(context, "dst_filename") {
                let dst_str = unsafe { CStr::from_ptr(ptr) };
                if let Ok(dst_utf8) = dst_str.to_str() {
                    let copy_len = std::cmp::min(dst_utf8.len(), ctxt.dst_filename.len() - 1);
                    ctxt.dst_filename[..copy_len].copy_from_slice(dst_utf8.as_bytes());
                    ctxt.dst_filename[copy_len] = 0;
                }
            }
            
            if let Some(cfdp_class) = ut_hook_get_arg_value_by_name::<CfCfdpClass>(context, "cfdp_class") {
                ctxt.cfdp_class = cfdp_class;
            }
            
            if let Some(keep) = ut_hook_get_arg_value_by_name::<u8>(context, "keep") {
                ctxt.keep = keep;
            }
            
            if let Some(chan) = ut_hook_get_arg_value_by_name::<u8>(context, "chan") {
                ctxt.chan = chan;
            }
            
            if let Some(priority) = ut_hook_get_arg_value_by_name::<u8>(context, "priority") {
                ctxt.priority = priority;
            }
            
            if let Some(dest_id) = ut_hook_get_arg_value_by_name::<CfEntityId>(context, "dest_id") {
                ctxt.dest_id = dest_id;
            }
        }
    }

    /// For compatibility with other tests, this has a mechanism to save its
    /// arguments to a test-provided context capture buffer.
    pub extern "C" fn ut_default_handler_cf_cfdp_playback_dir(
        user_obj: *mut c_void,
        func_key: UtEntryKey,
        context: *const UtStubContext,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CfCfdpPlaybackDirContext>(func_key);
        
        if let Some(ctxt) = ctxt {
            if let Some(ptr) = ut_hook_get_arg_value_by_name::<*const c_char>(context, "src_filename") {
                let src_str = unsafe { CStr::from_ptr(ptr) };
                if let Ok(src_utf8) = src_str.to_str() {
                    let copy_len = std::cmp::min(src_utf8.len(), ctxt.src_filename.len() - 1);
                    ctxt.src_filename[..copy_len].copy_from_slice(src_utf8.as_bytes());
                    ctxt.src_filename[copy_len] = 0;
                }
            }
            
            if let Some(ptr) = ut_hook_get_arg_value_by_name::<*const c_char>(context, "dst_filename") {
                let dst_str = unsafe { CStr::from_ptr(ptr) };
                if let Ok(dst_utf8) = dst_str.to_str() {
                    let copy_len = std::cmp::min(dst_utf8.len(), ctxt.dst_filename.len() - 1);
                    ctxt.dst_filename[..copy_len].copy_from_slice(dst_utf8.as_bytes());
                    ctxt.dst_filename[copy_len] = 0;
                }
            }
            
            if let Some(cfdp_class) = ut_hook_get_arg_value_by_name::<CfCfdpClass>(context, "cfdp_class") {
                ctxt.cfdp_class = cfdp_class;
            }
            
            if let Some(keep) = ut_hook_get_arg_value_by_name::<u8>(context, "keep") {
                ctxt.keep = keep;
            }
            
            if let Some(chan) = ut_hook_get_arg_value_by_name::<u8>(context, "chan") {
                ctxt.chan = chan;
            }
            
            if let Some(priority) = ut_hook_get_arg_value_by_name::<u8>(context, "priority") {
                ctxt.priority = priority;
            }
            
            if let Some(dest_id) = ut_hook_get_arg_value_by_name::<u16>(context, "dest_id") {
                ctxt.dest_id = dest_id;
            }
        }
    }

    /// For compatibility with other tests, this has a mechanism to save its
    /// arguments to a test-provided context capture buffer.
    pub extern "C" fn ut_default_handler_cf_cfdp_reset_transaction(
        user_obj: *mut c_void,
        func_key: UtEntryKey,
        context: *const UtStubContext,
    ) {
        let ctxt = ut_cf_get_context_buffer::<CfCfdpResetTransactionContext>(func_key);
        
        if let Some(ctxt) = ctxt {
            if let Some(txn) = ut_hook_get_arg_value_by_name::<*mut CfTransaction>(context, "txn") {
                ctxt.txn = txn;
            }
            
            if let Some(keep_history) = ut_hook_get_arg_value_by_name::<bool>(context, "keep_history") {
                ctxt.keep_history = keep_history;
            }
        }
    }

    /// For compatibility with other tests, this has a mechanism to save its
    /// arguments to a test-provided context capture buffer.
    pub extern "C" fn ut_default_handler_cf_cfdp_cancel_transaction(
        user_obj: *mut c_void,
        func_key: UtEntryKey,
        context: *const UtStubContext,
    ) {
        let ctxt = ut_cf_get_context_buffer::<*mut CfTransaction>(func_key);
        
        if let Some(ctxt) = ctxt {
            if let Some(txn) = ut_hook_get_arg_value_by_name::<*mut CfTransaction>(context, "txn") {
                *ctxt = txn;
            }
        }
    }
}