//! NASA Docket No. GSC-18,447-1, and identified as "CFS CFDP (CF)
//! Application version 3.0.0"
//!
//! Copyright (c) 2019 United States Government as represented by the
//! Administrator of the National Aeronautics and Space Administration.
//! All Rights Reserved.
//!
//! Licensed under the Apache License, Version 2.0 (the "License"); you may
//! not use this file except in compliance with the License. You may obtain
//! a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! CFS CFDP (CF) Application Message IDs

use cfe_core_api_base_msgids::*;
use cf_msgid_values::*;

/// CFS CFDP Command Message IDs

/// Message ID for commands
pub const CF_CMD_MID: u32 = CFE_PLATFORM_CF_CMD_MIDVAL_CMD;

/// Message ID to request housekeeping telemetry
pub const CF_SEND_HK_MID: u32 = CFE_PLATFORM_CF_CMD_MIDVAL_SEND_HK;

/// Message ID for waking up the processing cycle
pub const CF_WAKE_UP_MID: u32 = CFE_PLATFORM_CF_CMD_MIDVAL_WAKE_UP;

/// CFS CFDP Telemetry Message IDs

/// Message ID for housekeeping telemetry
pub const CF_HK_TLM_MID: u32 = CFE_PLATFORM_CF_TLM_MIDVAL_HK_TLM;

/// Message ID for end of transaction telemetry
pub const CF_EOT_TLM_MID: u32 = CFE_PLATFORM_CF_TLM_MIDVAL_EOT_TLM;

/// CFS CFDP Data Interface Message IDs

pub const CF_CH0_TX_MID: u32 = CFE_PLATFORM_CF_CMD_MIDVAL_CH0_TX;
pub const CF_CH1_TX_MID: u32 = CFE_PLATFORM_CF_CMD_MIDVAL_CH1_TX;
pub const CF_CH0_RX_MID: u32 = CFE_PLATFORM_CF_TLM_MIDVAL_CH0_RX;
pub const CF_CH1_RX_MID: u32 = CFE_PLATFORM_CF_TLM_MIDVAL_CH1_RX;