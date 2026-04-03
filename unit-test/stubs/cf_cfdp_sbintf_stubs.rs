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
 * Auto-Generated stub implementations for functions defined in cf_cfdp_sbintf header
 */

#[cfg(test)]
mod cf_cfdp_sbintf_stubs {
    use crate::{CF_Logical_PduBuffer_t, CF_Transaction_t, CF_Channel_t};

    /// Default handler for CF_CFDP_MsgOutGet stub
    pub fn ut_default_handler_cf_cfdp_msg_out_get() -> Option<Box<CF_Logical_PduBuffer_t>> {
        None
    }

    /// Generated stub function for CF_CFDP_MsgOutGet()
    pub fn cf_cfdp_msg_out_get(
        _txn: &CF_Transaction_t,
        _silent: bool
    ) -> Option<Box<CF_Logical_PduBuffer_t>> {
        ut_default_handler_cf_cfdp_msg_out_get()
    }

    /// Generated stub function for CF_CFDP_ReceiveMessage()
    pub fn cf_cfdp_receive_message(_chan: &mut CF_Channel_t) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CFDP_Send()
    pub fn cf_cfdp_send(_chan_num: u8, _ph: &CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }
}

#[cfg(test)]
pub use cf_cfdp_sbintf_stubs::*;