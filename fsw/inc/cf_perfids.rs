/// NASA Docket No. GSC-18,447-1, and identified as "CFS CFDP (CF)
/// Application version 3.0.0"
///
/// Copyright (c) 2019 United States Government as represented by the
/// Administrator of the National Aeronautics and Space Administration.
/// All Rights Reserved.
///
/// Licensed under the Apache License, Version 2.0 (the "License"); you may
/// not use this file except in compliance with the License. You may obtain
/// a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

/// Define CF Performance IDs

/// Main application performance ID
pub const CF_PERF_ID_APPMAIN: i32 = 11;
/// File seek performance ID
pub const CF_PERF_ID_FSEEK: i32 = 12;
/// File open performance ID
pub const CF_PERF_ID_FOPEN: i32 = 13;
/// File close performance ID
pub const CF_PERF_ID_FCLOSE: i32 = 14;
/// File read performance ID
pub const CF_PERF_ID_FREAD: i32 = 15;
/// File write performance ID
pub const CF_PERF_ID_FWRITE: i32 = 16;
/// Cycle engine performance ID
pub const CF_PERF_ID_CYCLE_ENG: i32 = 17;
/// Directory read performance ID
pub const CF_PERF_ID_DIRREAD: i32 = 18;
/// Create performance ID
pub const CF_PERF_ID_CREAT: i32 = 19;
/// Rename performance ID
pub const CF_PERF_ID_RENAME: i32 = 20;

/// PDU Received performance ID
pub const fn cf_perf_id_pdurcvd(x: i32) -> i32 {
    30 + x
}

/// PDU Sent performance ID
pub const fn cf_perf_id_pdusent(x: i32) -> i32 {
    40 + x
}