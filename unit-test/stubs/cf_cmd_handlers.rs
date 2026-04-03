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

//! @file
//! @brief The CF Application command handling stubs file
//!
//! All ground commands are processed in this file. All supporting functions
//! necessary to process the commands are also here.

#[cfg(test)]
mod cf_app_stubs {
    use crate::cf_test_utils::*;
    use crate::cf_app::*;
    use crate::ut_stubs::*;

    /// For compatibility with other tests, this has a mechanism to save its
    /// arguments to a test-provided context capture buffer.
    pub fn ut_default_handler_cf_process_ground_command(
        user_obj: Option<&mut dyn std::any::Any>,
        func_key: UtEntryKey,
        context: &UtStubContext,
    ) {
        if let Some(ctxt) = ut_cf_get_context_buffer::<*mut CfeSbBuffer>(func_key) {
            if let Some(buf_ptr) = ut_hook_get_arg_value_by_name::<*mut CfeSbBuffer>(context, "BufPtr") {
                unsafe {
                    *ctxt = buf_ptr;
                }
            }
        }
    }
}

#[cfg(test)]
pub use cf_app_stubs::*;