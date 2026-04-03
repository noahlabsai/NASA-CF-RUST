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
 *   Specification for the CFS CFDP (CF) command function codes
 *
 * @note
 *   This file should be strictly limited to the command/function code (CC)
 *   macro definitions.  Other definitions such as enums, typedefs, or other
 *   macros should be placed in the msgdefs.h or msg.h files.
 */

/************************************************************************
 * Macro Definitions
 ************************************************************************/

macro_rules! CF_CCVAL {
    ($x:ident) => {
        CFunctionCode::$x
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CFunctionCode {
    NOOP = 0,
    RESET_COUNTERS = 1,
    TX_FILE = 2,
    PLAYBACK_DIR = 3,
    FREEZE = 4,
    THAW = 5,
    SUSPEND = 6,
    RESUME = 7,
    CANCEL = 8,
    ABANDON = 9,
    SET_PARAM = 10,
    GET_PARAM = 11,
    WRITE_QUEUE = 15,
    ENABLE_DEQUEUE = 16,
    DISABLE_DEQUEUE = 17,
    ENABLE_DIR_POLLING = 18,
    DISABLE_DIR_POLLING = 19,
    PURGE_QUEUE = 21,
    ENABLE_ENGINE = 22,
    DISABLE_ENGINE = 23,
}