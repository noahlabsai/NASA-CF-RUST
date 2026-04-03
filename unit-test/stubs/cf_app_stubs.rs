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

/**
 * @file
 *
 * Auto-Generated stub implementations for functions defined in cf_app header
 */

#[cfg(test)]
mod cf_app_stubs {
    use crate::cf_app::*;
    use std::ffi::c_void;

    /// Generated stub function for CF_AppInit()
    pub fn cf_app_init() -> CfeStatusT {
        // UT_GenStub_SetupReturnBuffer(CF_AppInit, CFE_Status_t);
        // UT_GenStub_Execute(CF_AppInit, Basic, NULL);
        // return UT_GenStub_GetReturnValue(CF_AppInit, CFE_Status_t);
        CfeStatusT::default()
    }

    /// Generated stub function for CF_AppMain()
    pub fn cf_app_main() {
        // UT_GenStub_Execute(CF_AppMain, Basic, NULL);
    }

    /// Generated stub function for CF_CheckTables()
    pub fn cf_check_tables() {
        // UT_GenStub_Execute(CF_CheckTables, Basic, NULL);
    }

    /// Generated stub function for CF_TableInit()
    pub fn cf_table_init() -> CfeStatusT {
        // UT_GenStub_SetupReturnBuffer(CF_TableInit, CFE_Status_t);
        // UT_GenStub_Execute(CF_TableInit, Basic, NULL);
        // return UT_GenStub_GetReturnValue(CF_TableInit, CFE_Status_t);
        CfeStatusT::default()
    }

    /// Generated stub function for CF_ValidateConfigTable()
    pub fn cf_validate_config_table(tbl_ptr: *mut c_void) -> CfeStatusT {
        // UT_GenStub_SetupReturnBuffer(CF_ValidateConfigTable, CFE_Status_t);
        // UT_GenStub_AddParam(CF_ValidateConfigTable, void *, tbl_ptr);
        // UT_GenStub_Execute(CF_ValidateConfigTable, Basic, NULL);
        // return UT_GenStub_GetReturnValue(CF_ValidateConfigTable, CFE_Status_t);
        let _ = tbl_ptr; // Suppress unused parameter warning
        CfeStatusT::default()
    }
}