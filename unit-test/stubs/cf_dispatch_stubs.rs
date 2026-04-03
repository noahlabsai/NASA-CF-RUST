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

//! Auto-Generated stub implementations for functions defined in cf_dispatch header

use crate::cf_dispatch::*;
use crate::utgenstub::*;
use crate::cfe_sb::CFE_SB_Buffer_t;

#[cfg(test)]
mod stubs {
    use super::*;

    /// Generated stub function for CF_AppPipe()
    pub fn cf_app_pipe(buf_ptr: &CFE_SB_Buffer_t) {
        ut_gen_stub_add_param("CF_AppPipe", "BufPtr", buf_ptr as *const _ as *const std::ffi::c_void);
        ut_gen_stub_execute("CF_AppPipe", "Basic", std::ptr::null_mut());
    }

    /// Generated stub function for CF_ProcessGroundCommand()
    pub fn cf_process_ground_command(buf_ptr: &CFE_SB_Buffer_t) {
        ut_gen_stub_add_param("CF_ProcessGroundCommand", "BufPtr", buf_ptr as *const _ as *const std::ffi::c_void);
        ut_gen_stub_execute("CF_ProcessGroundCommand", "Basic", std::ptr::null_mut());
    }
}

#[cfg(test)]
pub use stubs::*;