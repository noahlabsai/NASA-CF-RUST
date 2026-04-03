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

use std::ptr;

#[cfg(test)]
mod test_stubs {
    use super::*;

    // Type definitions for stub framework compatibility
    pub type UT_EntryKey_t = u32;
    
    #[repr(C)]
    pub struct UT_StubContext_t {
        // Stub context implementation details
        _private: [u8; 0],
    }

    // External stub framework functions
    extern "C" {
        fn UT_Stub_GetInt32StatusCode(context: *const UT_StubContext_t, status_code: *mut i32) -> bool;
        fn UT_Stub_SetReturnValue(func_key: UT_EntryKey_t, retval: usize);
    }

    /*----------------------------------------------------------------
     *
     * Translates return value into the correct size for returning
     *
     *-----------------------------------------------------------------*/
    #[no_mangle]
    pub extern "C" fn UT_DefaultHandler_CF_CFDP_CodecCheckSize(
        _user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let retval: bool;
        let mut status_code: i32 = 0;

        // SAFETY: context is assumed valid by the stub framework
        if unsafe { UT_Stub_GetInt32StatusCode(context, &mut status_code) } {
            retval = status_code != 0;
        } else {
            retval = false;
        }

        // SAFETY: func_key is valid and retval is converted to usize
        unsafe {
            UT_Stub_SetReturnValue(func_key, retval as usize);
        }
    }

    /*----------------------------------------------------------------
     *
     * Default always returns NULL, an alt handler can be registered for other pointer returns
     *
     *-----------------------------------------------------------------*/
    #[no_mangle]
    pub extern "C" fn UT_DefaultHandler_CF_CFDP_DoEncodeChunk(
        _user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        _context: *const UT_StubContext_t,
    ) {
        let retval: *mut std::ffi::c_void;

        /* This may not need to do anything else, it shouldn't be called outside of this module */
        retval = ptr::null_mut();

        // SAFETY: func_key is valid and retval is a valid pointer value
        unsafe {
            UT_Stub_SetReturnValue(func_key, retval as usize);
        }
    }

    /*----------------------------------------------------------------
     *
     * Default always returns NULL, an alt handler can be registered for other pointer returns
     *
     *-----------------------------------------------------------------*/
    #[no_mangle]
    pub extern "C" fn UT_DefaultHandler_CF_CFDP_DoDecodeChunk(
        _user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        _context: *const UT_StubContext_t,
    ) {
        let retval: *const std::ffi::c_void;

        /* This may not need to do anything else, it shouldn't be called outside of this module */
        retval = ptr::null();

        // SAFETY: func_key is valid and retval is a valid pointer value
        unsafe {
            UT_Stub_SetReturnValue(func_key, retval as usize);
        }
    }

    /*----------------------------------------------------------------
     *
     * Translates return value into the correct size for returning
     *
     *-----------------------------------------------------------------*/
    #[no_mangle]
    pub extern "C" fn UT_DefaultHandler_CF_CFDP_GetValueEncodedSize(
        _user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let retval: u8;
        let mut status_code: i32 = 0;

        // SAFETY: context is assumed valid by the stub framework
        if unsafe { UT_Stub_GetInt32StatusCode(context, &mut status_code) } {
            retval = status_code as u8;
        } else {
            /* this defaults to 1 since nothing can get encoded in a size of 0.
             * test case can still set a different value, of course. */
            retval = 1;
        }

        // SAFETY: func_key is valid and retval is converted to usize
        unsafe {
            UT_Stub_SetReturnValue(func_key, retval as usize);
        }
    }

    /*----------------------------------------------------------------
     *
     * Translates return value into the correct size for returning
     *
     *-----------------------------------------------------------------*/
    #[no_mangle]
    pub extern "C" fn UT_DefaultHandler_CF_DecodeIntegerInSize(
        _user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let retval: u64;
        let mut status_code: i32 = 0;

        // SAFETY: context is assumed valid by the stub framework
        if unsafe { UT_Stub_GetInt32StatusCode(context, &mut status_code) } {
            retval = status_code as u64;
        } else {
            retval = 0;
        }

        // SAFETY: func_key is valid and retval is converted to usize
        unsafe {
            UT_Stub_SetReturnValue(func_key, retval as usize);
        }
    }
}