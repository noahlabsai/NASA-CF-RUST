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
 * Auto-Generated stub implementations for functions defined in cf_cfdp_dispatch header
 */

#[cfg(test)]
mod cf_cfdp_dispatch_stubs {
    use crate::cf_cfdp_dispatch::*;
    use crate::cf_types::*;
    use crate::utgenstub::*;

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_CFDP_R_DispatchRecv()
     * ----------------------------------------------------
     */
    pub fn cf_cfdp_r_dispatch_recv(
        txn: &mut CF_Transaction_t,
        ph: &mut CF_Logical_PduBuffer_t,
        dispatch: &CF_CFDP_R_SubstateDispatchTable_t,
        fd_fn: CF_CFDP_StateRecvFunc_t,
    ) {
        ut_gen_stub_add_param("CF_CFDP_R_DispatchRecv", "txn", txn as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_R_DispatchRecv", "ph", ph as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_R_DispatchRecv", "dispatch", dispatch as *const _ as *const std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_R_DispatchRecv", "fd_fn", fd_fn as *const std::ffi::c_void);

        ut_gen_stub_execute("CF_CFDP_R_DispatchRecv", UT_GenStub_ExecuteType::Basic, std::ptr::null_mut());
    }

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_CFDP_RxStateDispatch()
     * ----------------------------------------------------
     */
    pub fn cf_cfdp_rx_state_dispatch(
        txn: &mut CF_Transaction_t,
        ph: &mut CF_Logical_PduBuffer_t,
        dispatch: &CF_CFDP_TxnRecvDispatchTable_t,
    ) {
        ut_gen_stub_add_param("CF_CFDP_RxStateDispatch", "txn", txn as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_RxStateDispatch", "ph", ph as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_RxStateDispatch", "dispatch", dispatch as *const _ as *const std::ffi::c_void);

        ut_gen_stub_execute("CF_CFDP_RxStateDispatch", UT_GenStub_ExecuteType::Basic, std::ptr::null_mut());
    }

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_CFDP_S_DispatchRecv()
     * ----------------------------------------------------
     */
    pub fn cf_cfdp_s_dispatch_recv(
        txn: &mut CF_Transaction_t,
        ph: &mut CF_Logical_PduBuffer_t,
        dispatch: &CF_CFDP_S_SubstateRecvDispatchTable_t,
    ) {
        ut_gen_stub_add_param("CF_CFDP_S_DispatchRecv", "txn", txn as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_S_DispatchRecv", "ph", ph as *mut _ as *mut std::ffi::c_void);
        ut_gen_stub_add_param("CF_CFDP_S_DispatchRecv", "dispatch", dispatch as *const _ as *const std::ffi::c_void);

        ut_gen_stub_execute("CF_CFDP_S_DispatchRecv", UT_GenStub_ExecuteType::Basic, std::ptr::null_mut());
    }
}