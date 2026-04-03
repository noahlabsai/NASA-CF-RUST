//! CF CFDP Dispatch module.
//!
//! Translated from: cf_cfdp_dispatch.c / cf_cfdp_dispatch.h
//!
//! Dispatches received PDUs to the appropriate handler based on
//! transaction state and PDU type.

use std::ptr;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;
use crate::cf_logical_pdu::*;
use crate::cf_cfdp_types::*;
use crate::cf_cfdp_dispatch_types::*;
use crate::cf_eventids::*;
use crate::cf_app_types::CF_AppData_t;
use crate::CF_Assert;

/// Stub for CFE_EVS_SendEvent

/// Get printable CFDP class number (1 or 2)
#[inline]
pub fn CF_CFDP_GetPrintClass(txn: *const CF_Transaction_t) -> i32 {
    unsafe { ((*txn).reliable_mode as i32) + 1 }
}

/// Check if transaction status is OK (no error)
#[inline]
pub fn CF_CFDP_TxnIsOK(txn: *const CF_Transaction_t) -> bool {
    unsafe { (*txn).flags.com.q_index != CF_QueueIdx_t::CF_QueueIdx_FREE as u8 }
}

/// Dispatch received PDU for receive-file (R) transactions.
///
/// C original: `void CF_CFDP_R_DispatchRecv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph,
///              const CF_CFDP_R_SubstateDispatchTable_t *dispatch, CF_CFDP_StateRecvFunc_t fd_fn)`
pub unsafe fn CF_CFDP_R_DispatchRecv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
    dispatch: *const CF_CFDP_R_SubstateDispatchTable_t,
    fd_fn: CF_CFDP_StateRecvFunc_t,
) {
    let mut selected_handler: CF_CFDP_StateRecvFunc_t = None;

    // pdu_type == 0 means file directive
    if (*ph).pdu_header.pdu_type == 0 {
        let directive_code = (*ph).fdirective.directive_code as usize;
        if directive_code < CF_CFDP_FileDirective_INVALID_MAX {
            let sub_state = (*txn).state_data.sub_state as usize;
            if sub_state < CF_RxSubState_NUM_STATES {
                let state_table = (*dispatch).state[sub_state];
                if !state_table.is_null() {
                    if directive_code < (*state_table).fdirective.len() {
                        selected_handler = (*state_table).fdirective[directive_code];
                    }
                }
            }
        } else {
            // Invalid directive code
            CF_APP_DATA.hk.Payload.channel_hk[(*txn).chan_num as usize]
                .counters
                .recv
                .spurious += 1;
            CFE_EVS_SendEvent!(
                CF_CFDP_R_DC_INV_ERR_EID,
                CFE_EVS_EventType_ERROR,
                "CF: received PDU with invalid directive code",
            );
        }
    } else {
        // File data PDU
        if CF_CFDP_TxnIsOK(txn) {
            selected_handler = fd_fn;
        } else {
            CF_APP_DATA.hk.Payload.channel_hk[(*txn).chan_num as usize]
                .counters
                .recv
                .dropped += 1;
        }
    }

    if let Some(handler) = selected_handler {
        handler(txn, ph);
    }
}

/// Dispatch received PDU for send-file (S) transactions.
///
/// C original: `void CF_CFDP_S_DispatchRecv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph,
///              const CF_CFDP_S_SubstateRecvDispatchTable_t *dispatch)`
pub unsafe fn CF_CFDP_S_DispatchRecv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
    dispatch: *const CF_CFDP_S_SubstateRecvDispatchTable_t,
) {
    let mut selected_handler: CF_CFDP_StateRecvFunc_t = None;

    if (*ph).pdu_header.pdu_type == 0 {
        let directive_code = (*ph).fdirective.directive_code as usize;
        if directive_code < CF_CFDP_FileDirective_INVALID_MAX {
            let sub_state = (*txn).state_data.sub_state as usize;
            if sub_state < CF_TxSubState_NUM_STATES {
                let substate_tbl = (*dispatch).substate[sub_state];
                if !substate_tbl.is_null() {
                    if directive_code < (*substate_tbl).fdirective.len() {
                        selected_handler = (*substate_tbl).fdirective[directive_code];
                    }
                }
            }
        } else {
            CF_APP_DATA.hk.Payload.channel_hk[(*txn).chan_num as usize]
                .counters
                .recv
                .spurious += 1;
            CFE_EVS_SendEvent!(
                CF_CFDP_S_DC_INV_ERR_EID,
                CFE_EVS_EventType_ERROR,
                "CF: received PDU with invalid directive code",
            );
        }
    } else {
        // Sender should not receive file data PDUs
        CFE_EVS_SendEvent!(
            CF_CFDP_S_NON_FD_PDU_ERR_EID,
            CFE_EVS_EventType_ERROR,
            "CF: received non-file directive PDU on sender",
        );
    }

    if let Some(handler) = selected_handler {
        handler(txn, ph);
    }
}

/// Top-level receive state dispatch.
///
/// C original: `void CF_CFDP_RxStateDispatch(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph,
///              const CF_CFDP_TxnRecvDispatchTable_t *dispatch)`
pub unsafe fn CF_CFDP_RxStateDispatch(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
    dispatch: *const CF_CFDP_TxnRecvDispatchTable_t,
) {
    // If the acknowledged mode in the PDU doesn't match the transaction, drop it
    let state = if ((*ph).pdu_header.txm_mode == 0) != (*txn).reliable_mode {
        CF_TxnState_t::CF_TxnState_DROP
    } else {
        (*txn).state
    };

    CF_Assert!((state as usize) < CF_TxnState_INVALID);

    let state_idx = state as usize;
    if state_idx < (*dispatch).rx.len() {
        if let Some(handler) = (*dispatch).rx[state_idx] {
            handler(txn, ph);
        }
    }
}

/// Top-level transmit state dispatch.
///
/// C original: `void CF_CFDP_TxStateDispatch(CF_Transaction_t *txn,
///              const CF_CFDP_TxnSendDispatchTable_t *dispatch)`
pub unsafe fn CF_CFDP_TxStateDispatch(
    txn: *mut CF_Transaction_t,
    dispatch: *const CF_CFDP_TxnSendDispatchTable_t,
) {
    CF_Assert!(((*txn).state as usize) < CF_TxnState_INVALID);

    let state_idx = (*txn).state as usize;
    if state_idx < (*dispatch).tx.len() {
        if let Some(handler) = (*dispatch).tx[state_idx] {
            handler(txn);
        }
    }
}

/// Global application data (mutable static, matches C global)
///
/// # Safety
/// Access must be synchronized. In the C original this is a single-threaded
/// application so no synchronization is needed.
pub static mut CF_APP_DATA: CF_AppData_t = unsafe { core::mem::zeroed() };
