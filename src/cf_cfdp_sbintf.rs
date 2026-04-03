//! CF CFDP Software Bus Interface.
//!
//! Translated from: cf_cfdp_sbintf.c / cf_cfdp_sbintf.h
//!
//! This is the interface to the CFE Software Bus for CF transmit/recv.
//! Specifically this implements 3 functions used by the CFDP engine:
//!  - CF_CFDP_MsgOutGet() - gets a buffer prior to transmitting
//!  - CF_CFDP_Send() - sends the buffer from CF_CFDP_MsgOutGet
//!  - CF_CFDP_ReceiveMessage() - gets a received message

use std::ptr;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;
use crate::cf_logical_pdu::*;
use crate::cf_cfdp_types::*;
use crate::cf_clist_types::*;
use crate::cf_codec_types::*;
use crate::cf_msg::*;
use crate::cf_perfids::*;
use crate::cf_eventids::*;
use crate::cf_app_types::CF_AppData_t;
use crate::cf_perfids::*;
use crate::CF_Assert;

// =====================================================================
// cFE API stubs (to be replaced with real FFI bindings)
// =====================================================================

unsafe fn CFE_SB_ReleaseMessageBuffer(_msg: *mut CFE_SB_Buffer_t) {}
unsafe fn CFE_SB_AllocateMessageBuffer(_size: usize) -> *mut CFE_SB_Buffer_t { ptr::null_mut() }
unsafe fn CFE_MSG_Init(_msg: *mut CFE_MSG_Message_t, _mid: CFE_SB_MsgId_t, _size: usize) {}
unsafe fn CFE_MSG_SetSize(_msg: *mut CFE_MSG_Message_t, _size: usize) {}
unsafe fn CFE_MSG_SetMsgTime(_msg: *mut CFE_MSG_Message_t, _time: CFE_TIME_SysTime_t) {}
unsafe fn CFE_MSG_GetSize(_msg: *const CFE_MSG_Message_t, size: *mut usize) { *size = 0; }
unsafe fn CFE_MSG_GetType(_msg: *const CFE_MSG_Message_t, mtype: *mut u32) { *mtype = 0; }
unsafe fn CFE_SB_TransmitBuffer(_msg: *mut CFE_SB_Buffer_t, _increment: bool) {}
unsafe fn CFE_SB_ReceiveBuffer(_buf: *mut *mut CFE_SB_Buffer_t, _pipe: osal_id_t, _timeout: i32) -> i32 { -1 }
unsafe fn CFE_SB_ValueToMsgId(val: u32) -> CFE_SB_MsgId_t { val }
unsafe fn CFE_ES_PerfLogEntry(_id: u32) {}
unsafe fn CFE_ES_PerfLogExit(_id: u32) {}
unsafe fn CFE_TIME_GetTime() -> CFE_TIME_SysTime_t { CFE_TIME_SysTime_t { seconds: 0, subseconds: 0 } }
unsafe fn OS_ObjectIdDefined(id: osal_id_t) -> bool { id != 0 }
unsafe fn OS_CountSemTimedWait(_id: osal_id_t, _timeout: u32) -> i32 { 0 }

const CFE_SB_POLL: i32 = 0;
const CFE_MSG_Type_Invalid: u32 = 0;
const CFE_MSG_Type_Tlm: u32 = 2;

/// Stub for CF_CFDP_EncodeStart (implemented in cf_cfdp.rs)
unsafe fn CF_CFDP_EncodeStart(
    _penc: *mut CF_EncoderState_t,
    _msgbuf: *mut CFE_SB_Buffer_t,
    _ph: *mut CF_Logical_PduBuffer_t,
    _encap_hdr_size: usize,
    _total_size: usize,
) {}

/// Stub for CF_CFDP_DecodeStart (implemented in cf_cfdp.rs)
unsafe fn CF_CFDP_DecodeStart(
    _pdec: *mut CF_DecoderState_t,
    _msgbuf: *const CFE_SB_Buffer_t,
    _ph: *mut CF_Logical_PduBuffer_t,
    _encap_hdr_size: usize,
    _total_size: usize,
) {}

/// Stub for CF_CFDP_ReceivePdu (implemented in cf_cfdp.rs)
unsafe fn CF_CFDP_ReceivePdu(_chan: *mut CF_Channel_t, _ph: *mut CF_Logical_PduBuffer_t) {}

/// Offset of `ph` field in CF_PduTlmMsg_t (encapsulation header size)
fn offsetof_pdu_tlm_ph() -> usize {
    core::mem::size_of::<CFE_MSG_TelemetryHeader_t>()
}

/// Offset of `ph` field in CF_PduCmdMsg_t (encapsulation header size)
fn offsetof_pdu_cmd_ph() -> usize {
    core::mem::size_of::<CFE_MSG_CommandHeader_t>()
}

// =====================================================================
// CF_CFDP_MsgOutGet
// =====================================================================

/// Get an output message buffer for transmitting a PDU.
///
/// C original: `CF_Logical_PduBuffer_t *CF_CFDP_MsgOutGet(const CF_Transaction_t *txn, bool silent)`
///
/// # Safety
/// `txn` must be a valid pointer to a CF_Transaction_t.
pub unsafe fn CF_CFDP_MsgOutGet(
    txn: *const CF_Transaction_t,
    silent: bool,
) -> *mut CF_Logical_PduBuffer_t {
    let app = CF_AppData_ptr();
    let chan_num = (*txn).chan_num as usize;
    let chan = &mut (*app).engine.channels[chan_num];
    let mut success = true;
    let mut ret: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

    // Release any existing outgoing message
    if !(*app).engine.out.msg.is_null() {
        CFE_SB_ReleaseMessageBuffer((*app).engine.out.msg);
        (*app).engine.out.msg = ptr::null_mut();
    }

    // Check max outgoing messages per wakeup
    if (*(*app).config_table).chan[chan_num].max_outgoing_messages_per_wakeup != 0
        && chan.outgoing_counter >= (*(*app).config_table).chan[chan_num].max_outgoing_messages_per_wakeup
    {
        success = false;
    }

    if success
        && (*app).hk.Payload.channel_hk[chan_num].frozen == 0
        && !(*txn).flags.com.suspended
    {
        let os_status;
        if OS_ObjectIdDefined(chan.sem_id) {
            os_status = OS_CountSemTimedWait(chan.sem_id, 0);
        } else {
            os_status = OS_SUCCESS;
        }

        if os_status == OS_SUCCESS {
            (*app).engine.out.msg = CFE_SB_AllocateMessageBuffer(
                offsetof_pdu_tlm_ph() + CF_MAX_PDU_SIZE + CF_PDU_ENCAPSULATION_EXTRA_TRAILING_BYTES,
            );
        }

        if (*app).engine.out.msg.is_null() {
            if !silent && os_status == OS_SUCCESS {
                CFE_EVS_SendEvent!(
                    CF_CFDP_NO_MSG_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    "CF: no output message buffer available",
                );
            }
            success = false;
        }

        if success {
            CFE_MSG_Init(
                &mut (*(*app).engine.out.msg).Msg,
                CFE_SB_ValueToMsgId((*(*app).config_table).chan[chan_num].mid_output),
                offsetof_pdu_tlm_ph(),
            );
            chan.outgoing_counter += 1;
            ret = &mut (*app).engine.out.tx_pdudata;
        }
    }

    if ret.is_null() {
        chan.tx_blocked = true;
    } else {
        CF_CFDP_EncodeStart(
            &mut (*app).engine.out.encode,
            (*app).engine.out.msg,
            ret,
            offsetof_pdu_tlm_ph(),
            offsetof_pdu_tlm_ph() + CF_MAX_PDU_SIZE,
        );
    }

    ret
}

// =====================================================================
// CF_CFDP_Send
// =====================================================================

/// Send a PDU buffer over the software bus.
///
/// C original: `void CF_CFDP_Send(uint8 chan_num, const CF_Logical_PduBuffer_t *ph)`
///
/// # Safety
/// `ph` must be a valid pointer.
pub unsafe fn CF_CFDP_Send(chan_num: u8, ph: *const CF_Logical_PduBuffer_t) {
    let app = CF_AppData_ptr();

    CF_Assert!((chan_num as usize) < CF_NUM_CHANNELS);

    let mut sb_msgsize: usize = offsetof_pdu_tlm_ph();
    sb_msgsize += (*ph).pdu_header.header_encoded_length as usize;
    sb_msgsize += (*ph).pdu_header.data_encoded_length as usize;
    sb_msgsize += CF_PDU_ENCAPSULATION_EXTRA_TRAILING_BYTES;

    CFE_MSG_SetSize(&mut (*(*app).engine.out.msg).Msg, sb_msgsize);
    CFE_MSG_SetMsgTime(&mut (*(*app).engine.out.msg).Msg, CFE_TIME_GetTime());
    CFE_SB_TransmitBuffer((*app).engine.out.msg, true);

    (*app).hk.Payload.channel_hk[chan_num as usize].counters.sent.pdu += 1;

    (*app).engine.out.msg = ptr::null_mut();
}

// =====================================================================
// CF_CFDP_ReceiveMessage
// =====================================================================

/// Receive and process PDU messages from the software bus.
///
/// C original: `void CF_CFDP_ReceiveMessage(CF_Channel_t *chan)`
///
/// # Safety
/// `chan` must be a valid pointer to a CF_Channel_t within CF_AppData.engine.channels.
pub unsafe fn CF_CFDP_ReceiveMessage(chan: *mut CF_Channel_t) {
    let app = CF_AppData_ptr();
    let chan_num = (chan as usize - (*app).engine.channels.as_ptr() as usize)
        / core::mem::size_of::<CF_Channel_t>();
    let mut count: u32 = 0;
    let mut bufptr: *mut CFE_SB_Buffer_t = ptr::null_mut();
    let mut msg_size: usize = 0;
    let mut msg_type: u32 = CFE_MSG_Type_Invalid;

    while count < (*(*app).config_table).chan[chan_num].rx_max_messages_per_wakeup {
        let status = CFE_SB_ReceiveBuffer(&mut bufptr, (*chan).pipe, CFE_SB_POLL);
        if status != CFE_SUCCESS {
            break;
        }

        let ph = &mut (*app).engine.r#in.rx_pdudata;
        CFE_ES_PerfLogEntry(CF_PERF_ID_PDURCVD_BASE + chan_num as u32);
        CFE_MSG_GetSize(&(*bufptr).Msg, &mut msg_size);
        CFE_MSG_GetType(&(*bufptr).Msg, &mut msg_type);

        if msg_size > CF_PDU_ENCAPSULATION_EXTRA_TRAILING_BYTES {
            msg_size -= CF_PDU_ENCAPSULATION_EXTRA_TRAILING_BYTES;
        } else {
            msg_size = 0;
        }

        if msg_type == CFE_MSG_Type_Tlm {
            CF_CFDP_DecodeStart(
                &mut (*app).engine.r#in.decode,
                bufptr,
                ph,
                offsetof_pdu_tlm_ph(),
                msg_size,
            );
        } else {
            CF_CFDP_DecodeStart(
                &mut (*app).engine.r#in.decode,
                bufptr,
                ph,
                offsetof_pdu_cmd_ph(),
                msg_size,
            );
        }

        CF_CFDP_ReceivePdu(chan, ph);

        CFE_ES_PerfLogExit(CF_PERF_ID_PDURCVD_BASE + chan_num as u32);

        count += 1;
    }
}

/// Returns a mutable pointer to the global CF_AppData singleton.
unsafe fn CF_AppData_ptr() -> *mut CF_AppData_t {
    crate::cf_dispatch::CF_AppData_ptr()
}

const CFE_EVS_EventType_ERROR: u16 = 1;
