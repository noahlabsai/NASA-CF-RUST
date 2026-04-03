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

use crate::cf_chunk::CF_Chunk_t;
use crate::cf_test_utils::*;
use crate::uttest::*;
use crate::utstubs::*;
use crate::utgenstub::*;

#[cfg(test)]
mod stubs {
    use super::*;

    /// Default always returns NULL, an alt handler can be registered for other pointer returns
    pub fn ut_default_handler_cf_chunklist_get_first_chunk(
        user_obj: *mut std::ffi::c_void,
        func_key: UT_EntryKey_t,
        context: *const UT_StubContext_t,
    ) {
        let chunk: Option<Box<CF_Chunk_t>> = None;
        unsafe {
            UT_Stub_SetReturnValue(func_key, chunk.map_or(std::ptr::null_mut(), |b| Box::into_raw(b)));
        }
    }
}