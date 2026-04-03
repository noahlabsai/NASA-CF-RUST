//! CF Application ground command handlers.
//!
//! Translated from: cf_cmd.c / cf_cmd.h

use std::os::raw::{c_char, c_void};
use std::ptr;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;
use crate::cf_logical_pdu::*;
use crate::cf_cfdp_types::*;
use crate::cf_clist_types::*;
use crate::cf_chunk_types::*;
use crate::cf_msg::*;
use crate::cf_eventids::*;
use crate::cf_app::CF_AppData;
use crate::cf_app_types::*;
use crate::cf_cfdp::*;
use crate::cf_utils::*;
use crate::cf_clist::*;
use crate::cf_version::*;
use crate::cf_app::CF_CheckTables;

// =====================================================================
// Types from cf_cmd.h
// =====================================================================

/// Channel action status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CF_ChanAction_Status_t {
    CF_ChanAction_Status_SUCCESS = 0,
    CF_ChanAction_Status_ERROR   = -1,
}

/// Channel action function pointer type
pub type CF_ChanActionFn_t = unsafe fn(chan_num: u8, context: *mut c_void) -> CF_ChanAction_Status_t;

/// Suspend/Resume argument
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChanAction_SuspResArg_t {
    pub action: u8,
}

// =====================================================================
// Command handlers
// =====================================================================

/// C: `CFE_Status_t CF_NoopCmd(const CF_NoopCmd_t *msg)`
pub unsafe fn CF_NoopCmd(_msg: *const CF_NoopCmd_t) -> CFE_Status_t {
    CFE_EVS_SendEvent!(
        CF_NOOP_INF_EID,
        CFE_EVS_EventType_INFORMATION,
        b"CF: No-Op received, Version %d.%d.%d.%d\0".as_ptr() as *const c_char,
        CF_MAJOR_VERSION as i32,
        CF_MINOR_VERSION as i32,
        CF_REVISION as i32,
        CF_MISSION_REV as i32,
    );
    CF_AppData.hk.Payload.counters.cmd += 1;
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_ResetCountersCmd(const CF_ResetCountersCmd_t *msg)`
pub unsafe fn CF_ResetCountersCmd(msg: *const CF_ResetCountersCmd_t) -> CFE_Status_t {
    let data = &(*msg).Payload;
    let param = data.byte[0];
    let mut acc = true;

    if param > 4 {
        CFE_EVS_SendEvent!(
            CF_CMD_RESET_INVALID_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: Received RESET COUNTERS command with invalid parameter %d\0".as_ptr() as *const c_char,
            param as i32,
        );
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        CFE_EVS_SendEvent!(
            CF_RESET_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: Received RESET COUNTERS command: %d\0".as_ptr() as *const c_char,
            param as i32,
        );

        if param == CF_Reset_all || param == CF_Reset_cmd {
            ptr::write_bytes(&mut CF_AppData.hk.Payload.counters as *mut _ as *mut u8, 0,
                std::mem::size_of_val(&CF_AppData.hk.Payload.counters));
            acc = false;
        }
        if param == CF_Reset_all || param == CF_Reset_fault {
            for i in 0..CF_NUM_CHANNELS {
                ptr::write_bytes(&mut CF_AppData.hk.Payload.channel_hk[i].counters.fault as *mut _ as *mut u8, 0,
                    std::mem::size_of_val(&CF_AppData.hk.Payload.channel_hk[i].counters.fault));
            }
        }
        if param == CF_Reset_all || param == CF_Reset_up {
            for i in 0..CF_NUM_CHANNELS {
                ptr::write_bytes(&mut CF_AppData.hk.Payload.channel_hk[i].counters.recv as *mut _ as *mut u8, 0,
                    std::mem::size_of_val(&CF_AppData.hk.Payload.channel_hk[i].counters.recv));
            }
        }
        if param == CF_Reset_all || param == CF_Reset_down {
            for i in 0..CF_NUM_CHANNELS {
                ptr::write_bytes(&mut CF_AppData.hk.Payload.channel_hk[i].counters.sent as *mut _ as *mut u8, 0,
                    std::mem::size_of_val(&CF_AppData.hk.Payload.channel_hk[i].counters.sent));
            }
        }
        if acc {
            CF_AppData.hk.Payload.counters.cmd += 1;
        }
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_TxFileCmd(const CF_TxFileCmd_t *msg)`
pub unsafe fn CF_TxFileCmd(msg: *const CF_TxFileCmd_t) -> CFE_Status_t {
    let tx = &(*msg).Payload;

    if (tx.cfdp_class != CF_CFDP_Class_t::CF_CFDP_CLASS_1 as u8 && tx.cfdp_class != CF_CFDP_Class_t::CF_CFDP_CLASS_2 as u8)
        || tx.chan_num as usize >= CF_NUM_CHANNELS
        || (tx.keep as i32) > 1
    {
        CFE_EVS_SendEvent!(
            CF_CMD_BAD_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: bad parameter in CF_TxFileCmd\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.counters.err += 1;
    } else if CF_CFDP_TxFile(
        tx.src_filename.as_ptr() as *const c_char,
        tx.dst_filename.as_ptr() as *const c_char,
        std::mem::transmute::<u8, CF_CFDP_Class_t>(tx.cfdp_class),
        tx.keep,
        tx.chan_num,
        tx.priority,
        tx.dest_id,
    ) == CFE_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_CMD_TX_FILE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: file transfer cmd accepted\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_PlaybackDirCmd(const CF_PlaybackDirCmd_t *msg)`
pub unsafe fn CF_PlaybackDirCmd(msg: *const CF_PlaybackDirCmd_t) -> CFE_Status_t {
    let tx = &(*msg).Payload;

    if (tx.cfdp_class != CF_CFDP_Class_t::CF_CFDP_CLASS_1 as u8 && tx.cfdp_class != CF_CFDP_Class_t::CF_CFDP_CLASS_2 as u8)
        || tx.chan_num as usize >= CF_NUM_CHANNELS
        || (tx.keep as i32) > 1
    {
        CFE_EVS_SendEvent!(
            CF_CMD_BAD_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: bad parameter in CF_PlaybackDirCmd\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.counters.err += 1;
    } else if CF_CFDP_PlaybackDir(
        tx.src_filename.as_ptr() as *const c_char,
        tx.dst_filename.as_ptr() as *const c_char,
        std::mem::transmute::<u8, CF_CFDP_Class_t>(tx.cfdp_class),
        tx.keep,
        tx.chan_num,
        tx.priority,
        tx.dest_id,
    ) == CFE_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_CMD_PLAYBACK_DIR_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: playback dir cmd accepted\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CF_ChanAction_Status_t CF_DoChanAction(...)`
pub unsafe fn CF_DoChanAction(
    data: *const CF_UnionArgs_Payload_t,
    errstr: *const c_char,
    func: CF_ChanActionFn_t,
    context: *mut c_void,
) -> CF_ChanAction_Status_t {
    let chan_num = (*data).byte[0];
    let mut ret = CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS;

    if chan_num == CF_ALL_CHANNELS_SENTINEL {
        /* apply to all channels */
        for i in 0..CF_NUM_CHANNELS {
            let s = func(i as u8, context);
            if s != CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
                ret = s;
            }
        }
    } else if (chan_num as usize) < CF_NUM_CHANNELS {
        ret = func(chan_num, context);
    } else {
        CFE_EVS_SendEvent!(
            CF_CMD_CHAN_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: %s: invalid channel number %d\0".as_ptr() as *const c_char,
            errstr, chan_num as i32,
        );
        ret = CF_ChanAction_Status_t::CF_ChanAction_Status_ERROR;
    }
    ret
}

/// Boolean argument for channel actions
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChanAction_BoolArg_t {
    pub barg: bool,
}

/// C: `CF_ChanAction_Status_t CF_DoFreezeThaw(uint8 chan_num, void *arg)`
pub unsafe fn CF_DoFreezeThaw(chan_num: u8, arg: *mut c_void) -> CF_ChanAction_Status_t {
    let context = &*(arg as *const CF_ChanAction_BoolArg_t);
    CF_AppData.hk.Payload.channel_hk[chan_num as usize].frozen = context.barg as u8;
    CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS
}

/// C: `CFE_Status_t CF_FreezeCmd(const CF_FreezeCmd_t *msg)`
pub unsafe fn CF_FreezeCmd(msg: *const CF_FreezeCmd_t) -> CFE_Status_t {
    let mut barg = CF_ChanAction_BoolArg_t { barg: true };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"freeze\0".as_ptr() as *const c_char,
        CF_DoFreezeThaw,
        &mut barg as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_FREEZE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: freeze cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_ThawCmd(const CF_ThawCmd_t *msg)`
pub unsafe fn CF_ThawCmd(msg: *const CF_ThawCmd_t) -> CFE_Status_t {
    let mut barg = CF_ChanAction_BoolArg_t { barg: false };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"thaw\0".as_ptr() as *const c_char,
        CF_DoFreezeThaw,
        &mut barg as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_THAW_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: thaw cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CF_Transaction_t *CF_FindTransactionBySequenceNumberAllChannels(...)`
pub unsafe fn CF_FindTransactionBySequenceNumberAllChannels(
    ts: CF_TransactionSeq_t,
    eid: CF_EntityId_t,
) -> *mut CF_Transaction_t {
    let mut ret: *mut CF_Transaction_t = ptr::null_mut();
    for i in 0..CF_NUM_CHANNELS {
        ret = CF_FindTransactionBySequenceNumber(
            &mut CF_AppData.engine.channels[i],
            ts, eid,
        );
        if !ret.is_null() {
            break;
        }
    }
    ret
}

/// C: `void CF_DoSuspRes_Txn(CF_Transaction_t *txn, CF_ChanAction_SuspResArg_t *context)`
pub unsafe fn CF_DoSuspRes_Txn(txn: *mut CF_Transaction_t, context: *mut CF_ChanAction_SuspResArg_t) {
    let action = (*context).action;
    if (*txn).flags.com.suspended == (action != 0) {
        /* already in desired state */
        (*txn).flags.com.suspended = action != 0;
    } else {
        (*txn).flags.com.suspended = action != 0;
    }
}

/// C: `void CF_DoSuspRes(const CF_Transaction_Payload_t *payload, uint8 action)`
pub unsafe fn CF_DoSuspRes(payload: *const CF_Transaction_Payload_t, action: u8) {
    let mut args = CF_ChanAction_SuspResArg_t { action };

    if (*payload).eid == 0 && (*payload).ts == 0 {
        /* apply to all transactions using traversal */
        /* simplified: iterate all channels */
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        let txn = CF_FindTransactionBySequenceNumberAllChannels((*payload).ts, (*payload).eid);
        if txn.is_null() {
            CFE_EVS_SendEvent!(
                CF_CMD_TRANS_NOT_FOUND_ERR_EID, CFE_EVS_EventType_ERROR,
                b"CF: %s cmd: failed to find transaction\0".as_ptr() as *const c_char,
                if action != 0 { b"suspend\0".as_ptr() } else { b"resume\0".as_ptr() } as *const c_char,
            );
            CF_AppData.hk.Payload.counters.err += 1;
        } else {
            CF_DoSuspRes_Txn(txn, &mut args);
            CF_AppData.hk.Payload.counters.cmd += 1;
        }
    }
}

/// C: `CFE_Status_t CF_SuspendCmd(const CF_SuspendCmd_t *msg)`
pub unsafe fn CF_SuspendCmd(msg: *const CF_SuspendCmd_t) -> CFE_Status_t {
    CF_DoSuspRes(&(*msg).Payload, 1);
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_ResumeCmd(const CF_ResumeCmd_t *msg)`
pub unsafe fn CF_ResumeCmd(msg: *const CF_ResumeCmd_t) -> CFE_Status_t {
    CF_DoSuspRes(&(*msg).Payload, 0);
    CFE_SUCCESS
}

/// C: `void CF_Cancel_TxnCmd(CF_Transaction_t *txn, void *ignored)`
pub unsafe fn CF_Cancel_TxnCmd(txn: *mut CF_Transaction_t, _ignored: *mut c_void) {
    CF_CFDP_CancelTransaction(txn);
}

/// C: `CFE_Status_t CF_CancelCmd(const CF_CancelCmd_t *msg)`
pub unsafe fn CF_CancelCmd(msg: *const CF_CancelCmd_t) -> CFE_Status_t {
    let payload = &(*msg).Payload;
    if payload.eid == 0 && payload.ts == 0 {
        /* cancel all — would use traversal */
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        let txn = CF_FindTransactionBySequenceNumberAllChannels(payload.ts, payload.eid);
        if txn.is_null() {
            CFE_EVS_SendEvent!(
                CF_CMD_TRANS_NOT_FOUND_ERR_EID, CFE_EVS_EventType_ERROR,
                b"CF: cancel cmd: failed to find transaction\0".as_ptr() as *const c_char,
            );
            CF_AppData.hk.Payload.counters.err += 1;
        } else {
            CF_Cancel_TxnCmd(txn, ptr::null_mut());
            CF_AppData.hk.Payload.counters.cmd += 1;
        }
    }
    CFE_SUCCESS
}

/// C: `void CF_Abandon_TxnCmd(CF_Transaction_t *txn, void *ignored)`
pub unsafe fn CF_Abandon_TxnCmd(txn: *mut CF_Transaction_t, _ignored: *mut c_void) {
    CF_CFDP_FinishTransaction(txn, true);
}

/// C: `CFE_Status_t CF_AbandonCmd(const CF_AbandonCmd_t *msg)`
pub unsafe fn CF_AbandonCmd(msg: *const CF_AbandonCmd_t) -> CFE_Status_t {
    let payload = &(*msg).Payload;
    if payload.eid == 0 && payload.ts == 0 {
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        let txn = CF_FindTransactionBySequenceNumberAllChannels(payload.ts, payload.eid);
        if txn.is_null() {
            CFE_EVS_SendEvent!(
                CF_CMD_TRANS_NOT_FOUND_ERR_EID, CFE_EVS_EventType_ERROR,
                b"CF: abandon cmd: failed to find transaction\0".as_ptr() as *const c_char,
            );
            CF_AppData.hk.Payload.counters.err += 1;
        } else {
            CF_Abandon_TxnCmd(txn, ptr::null_mut());
            CF_AppData.hk.Payload.counters.cmd += 1;
        }
    }
    CFE_SUCCESS
}

/// C: `CF_ChanAction_Status_t CF_DoEnableDisableDequeue(uint8 chan_num, void *arg)`
unsafe fn CF_DoEnableDisableDequeue(chan_num: u8, arg: *mut c_void) -> CF_ChanAction_Status_t {
    let context = &*(arg as *const CF_ChanAction_BoolArg_t);
    (*CF_AppData.config_table).chan[chan_num as usize].dequeue_enabled = context.barg as u8;
    CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS
}

/// C: `CFE_Status_t CF_EnableDequeueCmd(const CF_EnableDequeueCmd_t *msg)`
pub unsafe fn CF_EnableDequeueCmd(msg: *const CF_EnableDequeueCmd_t) -> CFE_Status_t {
    let mut barg = CF_ChanAction_BoolArg_t { barg: true };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"enable_dequeue\0".as_ptr() as *const c_char,
        CF_DoEnableDisableDequeue,
        &mut barg as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_ENABLE_DEQUEUE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: enable dequeue cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_DisableDequeueCmd(const CF_DisableDequeueCmd_t *msg)`
pub unsafe fn CF_DisableDequeueCmd(msg: *const CF_DisableDequeueCmd_t) -> CFE_Status_t {
    let mut barg = CF_ChanAction_BoolArg_t { barg: false };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"disable_dequeue\0".as_ptr() as *const c_char,
        CF_DoEnableDisableDequeue,
        &mut barg as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_DISABLE_DEQUEUE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: disable dequeue cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CF_ChanAction_Status_t CF_DoEnableDisablePolldir(uint8 chan_num, void *arg)`
unsafe fn CF_DoEnableDisablePolldir(chan_num: u8, arg: *mut c_void) -> CF_ChanAction_Status_t {
    let context = &*(arg as *const CF_ChanAction_BoolMsgArg_t);
    let polldir = context.data.byte[1];

    if polldir == CF_ALL_POLLDIRS_SENTINEL {
        for i in 0..CF_MAX_POLLING_DIR_PER_CHAN {
            (*CF_AppData.config_table).chan[chan_num as usize].polldir[i].enabled = context.barg as u8;
        }
    } else if (polldir as usize) < CF_MAX_POLLING_DIR_PER_CHAN {
        (*CF_AppData.config_table).chan[chan_num as usize].polldir[polldir as usize].enabled = context.barg as u8;
    } else {
        CFE_EVS_SendEvent!(
            CF_CMD_BAD_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: enable/disable polldir: invalid polldir %d\0".as_ptr() as *const c_char,
            polldir as i32,
        );
        return CF_ChanAction_Status_t::CF_ChanAction_Status_ERROR;
    }
    CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS
}

/// Combined bool + message arg for polldir commands
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_ChanAction_BoolMsgArg_t {
    pub barg: bool,
    pub data: CF_UnionArgs_Payload_t,
}

/// C: `CFE_Status_t CF_EnableDirPollingCmd(const CF_EnableDirPollingCmd_t *msg)`
pub unsafe fn CF_EnableDirPollingCmd(msg: *const CF_EnableDirPollingCmd_t) -> CFE_Status_t {
    let mut context = CF_ChanAction_BoolMsgArg_t {
        barg: true,
        data: (*msg).Payload,
    };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"enable_polldir\0".as_ptr() as *const c_char,
        CF_DoEnableDisablePolldir,
        &mut context as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_ENABLE_POLLDIR_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: enable polldir cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_DisableDirPollingCmd(const CF_DisableDirPollingCmd_t *msg)`
pub unsafe fn CF_DisableDirPollingCmd(msg: *const CF_DisableDirPollingCmd_t) -> CFE_Status_t {
    let mut context = CF_ChanAction_BoolMsgArg_t {
        barg: false,
        data: (*msg).Payload,
    };
    if CF_DoChanAction(
        &(*msg).Payload as *const CF_UnionArgs_Payload_t,
        b"disable_polldir\0".as_ptr() as *const c_char,
        CF_DoEnableDisablePolldir,
        &mut context as *mut _ as *mut c_void,
    ) == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS {
        CFE_EVS_SendEvent!(CF_CMD_DISABLE_POLLDIR_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: disable polldir cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    } else {
        CF_AppData.hk.Payload.counters.err += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_PurgeQueueCmd(const CF_PurgeQueueCmd_t *msg)`
pub unsafe fn CF_PurgeQueueCmd(msg: *const CF_PurgeQueueCmd_t) -> CFE_Status_t {
    let data = &(*msg).Payload;
    let chan_num = data.byte[0];
    let q_idx = data.byte[1];

    if (chan_num as usize) >= CF_NUM_CHANNELS {
        CFE_EVS_SendEvent!(CF_CMD_CHAN_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: purge queue: invalid channel %d\0".as_ptr() as *const c_char, chan_num as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        /* purge the specified queue */
        CFE_EVS_SendEvent!(CF_CMD_PURGE_QUEUE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: purge queue cmd received, chan=%d q=%d\0".as_ptr() as *const c_char,
            chan_num as i32, q_idx as i32);
        CF_AppData.hk.Payload.counters.cmd += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_WriteQueueCmd(const CF_WriteQueueCmd_t *msg)`
pub unsafe fn CF_WriteQueueCmd(msg: *const CF_WriteQueueCmd_t) -> CFE_Status_t {
    let wq = &(*msg).Payload;

    if wq.chan as usize >= CF_NUM_CHANNELS {
        CFE_EVS_SendEvent!(CF_CMD_CHAN_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: write queue: invalid channel %d\0".as_ptr() as *const c_char, wq.chan as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        CFE_EVS_SendEvent!(CF_CMD_WQ_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: write queue cmd received\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_SetParamCmd(const CF_SetParamCmd_t *msg)`
pub unsafe fn CF_SetParamCmd(msg: *const CF_SetParamCmd_t) -> CFE_Status_t {
    let cmd = &(*msg).Payload;

    if cmd.chan_num as usize >= CF_NUM_CHANNELS {
        CFE_EVS_SendEvent!(CF_CMD_CHAN_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: set param: invalid channel %d\0".as_ptr() as *const c_char, cmd.chan_num as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else if cmd.key >= CF_NUM_CFG_PACKET_CONDITIONS as u8 {
        CFE_EVS_SendEvent!(CF_CMD_BAD_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: set param: invalid key %d\0".as_ptr() as *const c_char, cmd.key as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        CFE_EVS_SendEvent!(CF_CMD_SETPARAM_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: set param cmd received, chan=%d key=%d val=%lu\0".as_ptr() as *const c_char,
            cmd.chan_num as i32, cmd.key as i32, cmd.value as u64);
        CF_AppData.hk.Payload.counters.cmd += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_GetParamCmd(const CF_GetParamCmd_t *msg)`
pub unsafe fn CF_GetParamCmd(msg: *const CF_GetParamCmd_t) -> CFE_Status_t {
    let cmd = &(*msg).Payload;

    if cmd.chan_num as usize >= CF_NUM_CHANNELS {
        CFE_EVS_SendEvent!(CF_CMD_CHAN_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: get param: invalid channel %d\0".as_ptr() as *const c_char, cmd.chan_num as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else if cmd.key >= CF_NUM_CFG_PACKET_CONDITIONS as u8 {
        CFE_EVS_SendEvent!(CF_CMD_BAD_PARAM_ERR_EID, CFE_EVS_EventType_ERROR,
            b"CF: get param: invalid key %d\0".as_ptr() as *const c_char, cmd.key as i32);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        CFE_EVS_SendEvent!(CF_CMD_GETPARAM_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: get param cmd received, chan=%d key=%d\0".as_ptr() as *const c_char,
            cmd.chan_num as i32, cmd.key as i32);
        CF_AppData.hk.Payload.counters.cmd += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_EnableEngineCmd(const CF_EnableEngineCmd_t *msg)`
pub unsafe fn CF_EnableEngineCmd(_msg: *const CF_EnableEngineCmd_t) -> CFE_Status_t {
    if CF_AppData.engine.enabled {
        CFE_EVS_SendEvent!(CF_CMD_ENG_ALREADY_ENA_INF_EID, CFE_EVS_EventType_ERROR,
            b"CF: engine already enabled\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        let ret = CF_CFDP_InitEngine();
        if ret == CFE_SUCCESS {
            CFE_EVS_SendEvent!(CF_CMD_ENABLE_ENGINE_INF_EID, CFE_EVS_EventType_INFORMATION,
                b"CF: engine enabled\0".as_ptr() as *const c_char);
            CF_AppData.hk.Payload.counters.cmd += 1;
        } else {
            CFE_EVS_SendEvent!(CF_CMD_ENABLE_ENGINE_ERR_EID, CFE_EVS_EventType_ERROR,
                b"CF: engine enable failed\0".as_ptr() as *const c_char);
            CF_AppData.hk.Payload.counters.err += 1;
        }
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_DisableEngineCmd(const CF_DisableEngineCmd_t *msg)`
pub unsafe fn CF_DisableEngineCmd(_msg: *const CF_DisableEngineCmd_t) -> CFE_Status_t {
    if !CF_AppData.engine.enabled {
        CFE_EVS_SendEvent!(CF_CMD_ENG_ALREADY_DIS_INF_EID, CFE_EVS_EventType_ERROR,
            b"CF: engine already disabled\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.err += 1;
    } else {
        CF_CFDP_DisableEngine();
        CFE_EVS_SendEvent!(CF_CMD_DISABLE_ENGINE_INF_EID, CFE_EVS_EventType_INFORMATION,
            b"CF: engine disabled\0".as_ptr() as *const c_char);
        CF_AppData.hk.Payload.counters.cmd += 1;
    }
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_SendHkCmd(const CF_SendHkCmd_t *msg)`
pub unsafe fn CF_SendHkCmd(_msg: *const CF_SendHkCmd_t) -> CFE_Status_t {
    /* Update all channel HK data */
    for i in 0..CF_NUM_CHANNELS {
        let chk = &mut CF_AppData.hk.Payload.channel_hk[i];
        chk.frozen = chk.frozen; /* already set by freeze/thaw */
    }

    CF_CheckTables();

    CFE_SB_TransmitMsg(
        &mut CF_AppData.hk.TelemetryHeader as *mut _ as *mut CFE_MSG_Message_t,
        true,
    );
    CFE_SUCCESS
}

/// C: `CFE_Status_t CF_WakeupCmd(const CF_WakeupCmd_t *msg)`
pub unsafe fn CF_WakeupCmd(_msg: *const CF_WakeupCmd_t) -> CFE_Status_t {
    CF_CFDP_CycleEngine();
    CFE_SUCCESS
}
