//! CF Application main module.
//!
//! Translated from: cf_app.c / cf_app.h
//!
//! This file contains the functions that initialize the application and link
//! all logic and functionality to the CFS.

use std::os::raw::c_void;
use std::ptr;
use std::mem;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_app_types::*;
use crate::cf_eventids::*;
use crate::cf_perfids::*;
use crate::cf_msg::*;
use crate::cf_cfdp_pdu::CF_CFDP_PduFileDataContent_t;
use crate::cf_version::*;
use crate::cf_cfdp::CF_CFDP_InitEngine;
use crate::cf_dispatch::CF_AppPipe;

// =====================================================================
// Global application data singleton
// =====================================================================

/// The CF application global state structure.
/// C: `CF_AppData_t CF_AppData;`
///
/// NOTE: In the C code this is a file-scope global. In Rust we use
/// a mutable static behind unsafe. All access must be through unsafe blocks.
/// We zero-initialize the entire struct, matching C's BSS-segment behavior.
pub static mut CF_AppData: CF_AppData_t = unsafe { core::mem::zeroed() };

// =====================================================================
// CF_CheckTables
// =====================================================================

/// Checks to see if a table update is pending, and perform it.
///
/// C: `void CF_CheckTables(void)`
///
/// Updates the table if the engine is disabled. Releases the current
/// table address, manages the table (which may load a new image), then
/// re-acquires the address.
pub unsafe fn CF_CheckTables() {
    let status: CFE_Status_t;

    // check the table for an update only if engine is disabled
    if !CF_AppData.engine.enabled {
        /*
         * NOTE: As of CFE 7.0 (Caelum), some CFE TBL APIs return success codes
         * other than CFE_SUCCESS, so it is not sufficient to check for only this
         * result here. For now, the safest way to check is to check for negative
         * values, as the alt-success codes are in the positive range by design,
         * and error codes are all in the negative range of CFE_Status_t.
         */
        let mut s = CFE_TBL_ReleaseAddress(CF_AppData.config_handle);
        if s < CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_CHECK_REL_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: error in CFE_TBL_ReleaseAddress (check), returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
            CF_AppData.RunStatus = CFE_ES_RunStatus_APP_ERROR;
        }

        s = CFE_TBL_Manage(CF_AppData.config_handle);
        if s < CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_CHECK_MAN_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: error in CFE_TBL_Manage (check), returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
            CF_AppData.RunStatus = CFE_ES_RunStatus_APP_ERROR;
        }

        s = CFE_TBL_GetAddress(
            &mut CF_AppData.config_table as *mut *mut CF_ConfigTable_t as *mut *mut c_void,
            CF_AppData.config_handle,
        );
        if s < CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_CHECK_GA_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: failed to get table address (check), returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
            CF_AppData.RunStatus = CFE_ES_RunStatus_APP_ERROR;
        }
    }
}

// =====================================================================
// CF_ValidateConfigTable
// =====================================================================

/// Validation function for config table.
///
/// C: `CFE_Status_t CF_ValidateConfigTable(void *tbl_ptr)`
///
/// Checks that the config table being loaded has correct data.
pub unsafe fn CF_ValidateConfigTable(tbl_ptr: *mut c_void) -> CFE_Status_t {
    let tbl = tbl_ptr as *const CF_ConfigTable_t;
    let mut ret: CFE_Status_t = CFE_STATUS_VALIDATION_FAILURE;

    if (*tbl).ticks_per_second == 0 {
        CFE_EVS_SendEvent!(
            CF_INIT_TPS_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: config table has zero ticks per second\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    } else if (*tbl).rx_crc_calc_bytes_per_wakeup == 0
        || ((*tbl).rx_crc_calc_bytes_per_wakeup & 0x3ff) != 0
    {
        CFE_EVS_SendEvent!(
            CF_INIT_CRC_ALIGN_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: config table has rx CRC size not aligned with 1024\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    } else if (*tbl).outgoing_file_chunk_size as usize
        > mem::size_of::<CF_CFDP_PduFileDataContent_t>()
    {
        CFE_EVS_SendEvent!(
            CF_INIT_OUTGOING_SIZE_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: config table has outgoing file chunk size too large\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    } else {
        ret = CFE_SUCCESS;
    }

    ret
}

// =====================================================================
// CF_TableInit
// =====================================================================

/// Load the table on application start.
///
/// C: `CFE_Status_t CF_TableInit(void)`
///
/// Registers, loads, manages, and gets the address of the config table.
pub unsafe fn CF_TableInit() -> CFE_Status_t {
    let mut status: CFE_Status_t;

    status = CFE_TBL_Register(
        &mut CF_AppData.config_handle,
        CF_CONFIG_TABLE_NAME.as_ptr() as *const std::os::raw::c_char,
        mem::size_of::<CF_ConfigTable_t>(),
        CFE_TBL_OPT_SNGL_BUFFER | CFE_TBL_OPT_LOAD_DUMP,
        Some(CF_ValidateConfigTable_Wrapper),
    );
    if status != CFE_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_INIT_TBL_REG_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: error registering table, returned 0x%08lx\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    } else {
        status = CFE_TBL_Load(
            CF_AppData.config_handle,
            CFE_TBL_SRC_FILE,
            CF_CONFIG_TABLE_FILENAME.as_ptr() as *const c_void,
        );
        if status != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_LOAD_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: error loading table, returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
        }
    }

    if status == CFE_SUCCESS {
        status = CFE_TBL_Manage(CF_AppData.config_handle);
        if status != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_MANAGE_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: error in CFE_TBL_Manage, returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
        }
    }

    if status == CFE_SUCCESS {
        status = CFE_TBL_GetAddress(
            &mut CF_AppData.config_table as *mut *mut CF_ConfigTable_t as *mut *mut c_void,
            CF_AppData.config_handle,
        );
        // status will be CFE_TBL_INFO_UPDATED because it was just loaded,
        // but we can use CFE_SUCCESS too
        if status != CFE_TBL_INFO_UPDATED && status != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_TBL_GETADDR_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: error getting table address, returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
        } else {
            status = CFE_SUCCESS;
        }
    }

    status
}

/// Wrapper for the table validation callback.
/// CFE_TBL_Register expects `extern "C" fn(*mut c_void) -> CFE_Status_t`.
unsafe extern "C" fn CF_ValidateConfigTable_Wrapper(tbl_ptr: *mut c_void) -> CFE_Status_t {
    CF_ValidateConfigTable(tbl_ptr)
}

// =====================================================================
// CF_AppInit
// =====================================================================

/// CF app init function.
///
/// C: `CFE_Status_t CF_AppInit(void)`
///
/// Initializes all aspects of the CF application: messages, pipes,
/// events, table, and the CFDP engine.
pub unsafe fn CF_AppInit() -> CFE_Status_t {
    let mut status: CFE_Status_t;
    let mid_values: [CFE_SB_MsgId_Atom_t; 3] = [
        crate::cf_dispatch::CF_CMD_MID,
        crate::cf_dispatch::CF_SEND_HK_MID,
        crate::cf_dispatch::CF_WAKE_UP_MID,
    ];

    // Zero-out global data structure
    ptr::write_bytes(
        &mut CF_AppData as *mut CF_AppData_t as *mut u8,
        0,
        mem::size_of::<CF_AppData_t>(),
    );

    CF_AppData.RunStatus = CFE_ES_RunStatus_APP_RUN;

    CFE_MSG_Init(
        &mut CF_AppData.hk.TelemetryHeader as *mut CFE_MSG_TelemetryHeader_t
            as *mut CFE_MSG_Message_t,
        CFE_SB_ValueToMsgId(CF_HK_TLM_MID),
        mem::size_of::<CF_HkPacket_t>(),
    );

    status = CFE_EVS_Register(ptr::null(), 0, CFE_EVS_EventFilter_BINARY);
    if status != CFE_SUCCESS {
        CFE_ES_WriteToSysLog!(
            b"CF app: error registering with EVS, returned 0x%08lx\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    } else {
        status = CFE_SB_CreatePipe(
            &mut CF_AppData.CmdPipe,
            CF_PIPE_DEPTH,
            CF_PIPE_NAME.as_ptr() as *const std::os::raw::c_char,
        );
        if status != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_CR_PIPE_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF app: error creating pipe %s, returned 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
        }
    }

    if status == CFE_SUCCESS {
        for i in 0..mid_values.len() {
            status = CFE_SB_Subscribe(
                CFE_SB_ValueToMsgId(mid_values[i]),
                CF_AppData.CmdPipe,
            );
            if status != CFE_SUCCESS {
                CFE_ES_WriteToSysLog!(
                    b"CF app: failed to subscribe to MID 0x%04lx, returned 0x%08lx\0".as_ptr()
                        as *const std::os::raw::c_char,
                );
                break;
            }
        }
    }

    if status == CFE_SUCCESS {
        status = CF_TableInit(); // function sends event internally
    }

    if status == CFE_SUCCESS {
        status = CF_CFDP_InitEngine(); // function sends event internally
    }

    if status == CFE_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_INIT_INF_EID,
            CFE_EVS_EventType_INFORMATION,
            b"CF Initialized. Version %d.%d.%d.%d\0".as_ptr()
                as *const std::os::raw::c_char,
        );
    }

    status
}

// =====================================================================
// CF_AppMain
// =====================================================================

/// CF app entry point.
///
/// C: `void CF_AppMain(void)`
///
/// Main entry point of CF application. Calls the init function and
/// manages the app run loop.
pub unsafe fn CF_AppMain() {
    let mut status: CFE_Status_t;
    let mut buf_ptr: *mut CFE_SB_Buffer_t = ptr::null_mut();

    CFE_ES_PerfLogEntry(CF_PERF_ID_APPMAIN);

    status = CF_AppInit();
    if status != CFE_SUCCESS {
        CF_AppData.RunStatus = CFE_ES_RunStatus_APP_ERROR;
    }

    while CFE_ES_RunLoop(&mut CF_AppData.RunStatus) {
        CFE_ES_PerfLogExit(CF_PERF_ID_APPMAIN);

        status = CFE_SB_ReceiveBuffer(&mut buf_ptr, CF_AppData.CmdPipe, CF_RCVMSG_TIMEOUT);
        CFE_ES_PerfLogEntry(CF_PERF_ID_APPMAIN);

        if status == CFE_SUCCESS {
            CF_AppPipe(buf_ptr as *const CFE_SB_Buffer_t);
        } else if status != CFE_SB_TIME_OUT && status != CFE_SB_NO_MESSAGE {
            CFE_EVS_SendEvent!(
                CF_INIT_MSG_RECV_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: exiting due to CFE_SB_ReceiveBuffer error 0x%08lx\0".as_ptr()
                    as *const std::os::raw::c_char,
            );
            CF_AppData.RunStatus = CFE_ES_RunStatus_APP_ERROR;
        } else {
            // nothing — timeout or no message is normal
        }
    }

    CFE_ES_PerfLogExit(CF_PERF_ID_APPMAIN);
    CFE_ES_ExitApp(CF_AppData.RunStatus);
}
