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

//! Auto-Generated stub implementations for functions defined in cf_crc header

#[cfg(test)]
mod cf_crc_stubs {
    use crate::cf_crc::CfCrc;

    /// Generated stub function for CF_CRC_Digest()
    pub fn cf_crc_digest(crc: &mut CfCrc, data: &[u8]) {
        // Stub implementation - no-op for testing
    }

    /// Generated stub function for CF_CRC_Finalize()
    pub fn cf_crc_finalize(crc: &mut CfCrc) {
        // Stub implementation - no-op for testing
    }

    /// Generated stub function for CF_CRC_Start()
    pub fn cf_crc_start(crc: &mut CfCrc) {
        // Stub implementation - no-op for testing
    }
}

#[cfg(test)]
pub use cf_crc_stubs::*;