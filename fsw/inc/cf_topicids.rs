/*
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
 */

//! CFDP (CF) Application Topic IDs

// Note: CFE_MISSION_CF_TIDVAL macro would need to be defined elsewhere
// These constants assume the macro expands to the DEFAULT values

/// Message ID for commands
pub const DEFAULT_CFE_MISSION_CF_CMD_TOPICID: u8 = 0xB3;

/// Message ID to request housekeeping telemetry
pub const DEFAULT_CFE_MISSION_CF_SEND_HK_TOPICID: u8 = 0xB4;

/// Message ID for waking up the processing cycle
pub const DEFAULT_CFE_MISSION_CF_WAKE_UP_TOPICID: u8 = 0xB5;

/// Message ID for housekeeping telemetry
pub const DEFAULT_CFE_MISSION_CF_HK_TLM_TOPICID: u8 = 0xB0;

/// Message ID for end of transaction telemetry
pub const DEFAULT_CFE_MISSION_CF_EOT_TLM_TOPICID: u8 = 0xB3;

pub const DEFAULT_CFE_MISSION_CF_CH0_TX_TOPICID: u8 = 0xB4;

pub const DEFAULT_CFE_MISSION_CF_CH1_TX_TOPICID: u8 = 0xB5;

pub const DEFAULT_CFE_MISSION_CF_CH0_RX_TOPICID: u8 = 0xB6;

pub const DEFAULT_CFE_MISSION_CF_CH1_RX_TOPICID: u8 = 0xB7;