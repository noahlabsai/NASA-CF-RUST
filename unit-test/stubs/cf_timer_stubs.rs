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
 * Auto-Generated stub implementations for functions defined in cf_timer header
 */

use crate::cf_timer::{CF_Timer_t, CF_Timer_Seconds_t};

#[cfg(test)]
pub mod cf_timer_stubs {
    use super::*;

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_Timer_Expired()
     * ----------------------------------------------------
     */
    pub fn cf_timer_expired(_txn: &CF_Timer_t) -> bool {
        bool::default()
    }

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_Timer_InitRelSec()
     * ----------------------------------------------------
     */
    pub fn cf_timer_init_rel_sec(_txn: &mut CF_Timer_t, _rel_sec: CF_Timer_Seconds_t) {
        // Stub implementation - no operation
    }

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_Timer_Sec2Ticks()
     * ----------------------------------------------------
     */
    pub fn cf_timer_sec2_ticks(_sec: CF_Timer_Seconds_t) -> u32 {
        u32::default()
    }

    /*
     * ----------------------------------------------------
     * Generated stub function for CF_Timer_Tick()
     * ----------------------------------------------------
     */
    pub fn cf_timer_tick(_txn: &mut CF_Timer_t) {
        // Stub implementation - no operation
    }
}