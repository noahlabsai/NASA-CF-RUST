//! CF CFDP core engine implementation.
//!
//! Translated from: cf_cfdp.c / cf_cfdp.h
//!
//! This module contains the core CFDP protocol engine functions.

use std::mem;
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
use crate::cf_timer_types::*;
use crate::cf_crc_types::*;
use crate::cf_codec_types::*;
use crate::cf_cfdp_dispatch_types::*;
use crate::cf_cfdp_s::{CF_CFDP_S_SubstateSendFileData, CF_CFDP_S_Tick, CF_CFDP_S_Tick_Nak};
use crate::cf_msg::*;
use crate::cf_eventids::*;
use crate::cf_perfids::*;
use crate::cf_app::CF_AppData;
use crate::cf_app_types::*;
use crate::cf_utils::*;
use crate::cf_cfdp_r::CF_CFDP_R_Init;
use crate::cf_cfdp_r::CF_CFDP_R_Tick;
use crate::cf_cfdp_s::CF_CFDP_S_Init;
use crate::cf_cfdp_sbintf::*;
use crate::cf_clist::*;
use crate::cf_chunk::*;
use crate::cf_timer::*;
use crate::cf_crc::*;

// =====================================================================
// Inline helpers (from cf_cfdp.h)
// =====================================================================

/// Get printable class number (1 or 2) for event messages.
/// C: `static inline int CF_CFDP_GetPrintClass(const CF_Transaction_t *txn)`
#[inline]
pub fn CF_CFDP_GetPrintClass(txn: *const CF_Transaction_t) -> i32 {
    unsafe { (*txn).flags.com.q_index as i32 + 1 }
}

/// Get the CFDP class of a transaction.
/// C: `static inline CF_CFDP_Class_t CF_CFDP_GetClass(const CF_Transaction_t *txn)`
#[inline]
pub fn CF_CFDP_GetClass(txn: *const CF_Transaction_t) -> CF_CFDP_Class_t {
    // In C: return (txn->flags.com.q_index == CF_QueueIdx_RX) ? CLASS_2 : CLASS_1
    // Simplified: the class is stored in the state
    unsafe {
        if (*txn).state == CF_TxnState_t::CF_TxnState_R2
            || (*txn).state == CF_TxnState_t::CF_TxnState_S2
        {
            CF_CFDP_Class_t::CF_CFDP_CLASS_2
        } else {
            CF_CFDP_Class_t::CF_CFDP_CLASS_1
        }
    }
}

/// Check if transaction is a sender.
/// C: `static inline bool CF_CFDP_IsSender(CF_Transaction_t *txn)`
#[inline]
pub fn CF_CFDP_IsSender(txn: *const CF_Transaction_t) -> bool {
    unsafe {
        CF_Assert!(!(*txn).history.is_null());
        (*(*txn).history).dir == CF_Direction_t::CF_Direction_TX
    }
}

// =====================================================================
// Encode/Decode start
// =====================================================================

/// C: `void CF_CFDP_EncodeStart(...)`
pub unsafe fn CF_CFDP_EncodeStart(
    penc: *mut CF_EncoderState_t,
    msgbuf: *mut c_void,
    ph: *mut CF_Logical_PduBuffer_t,
    encap_hdr_size: usize,
    total_size: usize,
) {
    /* Clear the PDU buffer structure to start */
    ptr::write_bytes(ph, 0, 1);

    /* attach encoder object to PDU buffer which is attached to SB (encapsulation) buffer */
    (*penc).base = msgbuf as *mut u8;
    (*ph).penc = penc;

    CF_CFDP_CodecReset(&mut (*penc).codec_state, total_size);

    /* adjust so that the base points to the actual PDU Header */
    if total_size > encap_hdr_size {
        (*penc).codec_state.max_size -= encap_hdr_size;
        (*penc).base = (*penc).base.add(encap_hdr_size);
    } else {
        CF_CFDP_CodecSetDone(&mut (*penc).codec_state);
    }
}

/// C: `void CF_CFDP_DecodeStart(...)`
pub unsafe fn CF_CFDP_DecodeStart(
    pdec: *mut CF_DecoderState_t,
    msgbuf: *const c_void,
    ph: *mut CF_Logical_PduBuffer_t,
    encap_hdr_size: usize,
    total_size: usize,
) {
    /* Clear the PDU buffer structure to start */
    ptr::write_bytes(ph, 0, 1);

    /* attach decoder object to PDU buffer which is attached to SB (encapsulation) buffer */
    (*pdec).base = msgbuf as *const u8;
    (*ph).pdec = pdec;

    CF_CFDP_CodecReset(&mut (*pdec).codec_state, total_size);

    /* adjust so that the base points to the actual PDU Header */
    if total_size > encap_hdr_size {
        (*pdec).codec_state.max_size -= encap_hdr_size;
        (*pdec).base = (*pdec).base.add(encap_hdr_size);
    } else {
        CF_CFDP_CodecSetDone(&mut (*pdec).codec_state);
    }
}

// =====================================================================
// Timer helpers
// =====================================================================

/// C: `void CF_CFDP_ArmAckTimer(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_ArmAckTimer(txn: *mut CF_Transaction_t) {
    CF_Timer_InitRelSec(
        &mut (*txn).ack_timer,
        (*CF_AppData.config_table).chan[(*txn).chan_num as usize].ack_timer_s,
        (*CF_AppData.config_table).ticks_per_second,
    );
    (*txn).flags.com.ack_timer_armed = true;
}

/// C: `void CF_CFDP_ArmInactTimer(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_ArmInactTimer(txn: *mut CF_Transaction_t) {
    let sec: CF_Timer_Seconds_t;

    /* select timeout based on the state */
    if CF_CFDP_GetAckTxnStatus(txn) == CF_CFDP_AckTxnStatus_t::CF_CFDP_AckTxnStatus_ACTIVE {
        /* in an active transaction, use the normal inactivity timer */
        sec = (*CF_AppData.config_table).chan[(*txn).chan_num as usize].inactivity_timer_s;
    } else {
        /* in an inactive transaction, use double the ack timer */
        sec = (*CF_AppData.config_table).chan[(*txn).chan_num as usize].ack_timer_s * 2;
    }

    CF_Timer_InitRelSec(&mut (*txn).inactivity_timer, sec, (*CF_AppData.config_table).ticks_per_second);
}

/// C: `bool CF_CFDP_CheckAckNakCount(CF_Transaction_t *txn, uint8 *counter)`
pub unsafe fn CF_CFDP_CheckAckNakCount(txn: *mut CF_Transaction_t, counter: *mut u8) -> bool {
    let is_ok: bool;

    /* Check limit and handle if needed */
    is_ok = *counter < (*CF_AppData.config_table).chan[(*txn).chan_num as usize].ack_limit;

    if is_ok {
        /* Under limit, Increment acknak counter */
        *counter += 1;
    } else {
        /* Reached limit */
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.ack_limit += 1;

        let event: u16;
        if (*(*txn).history).dir == CF_Direction_t::CF_Direction_TX {
            event = CF_CFDP_S_ACK_LIMIT_ERR_EID;
        } else {
            event = CF_CFDP_R_ACK_LIMIT_ERR_EID;
        }

        CFE_EVS_SendEvent!(
            event,
            CFE_EVS_EventType_ERROR,
            b"CF(%lu:%lu): ACK/NAK limit reached\0".as_ptr() as *const c_char,
        );
    }

    is_ok
}

// =====================================================================
// Dispatch
// =====================================================================

/// C: `void CF_CFDP_DispatchRecv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_DispatchRecv(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) {
    use crate::cf_cfdp_dispatch::CF_CFDP_RxStateDispatch;
    use crate::cf_cfdp_r::*;
    use crate::cf_cfdp_s::*;

    static STATE_FNS: CF_CFDP_TxnRecvDispatchTable_t = CF_CFDP_TxnRecvDispatchTable_t {
        rx: [
            Some(CF_CFDP_RecvInit as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t)),
            Some(CF_CFDP_R1_Recv),
            Some(CF_CFDP_S1_Recv),
            Some(CF_CFDP_R2_Recv),
            Some(CF_CFDP_S2_Recv),
            Some(CF_CFDP_RecvDrop),
            Some(CF_CFDP_RecvHold),
            None, /* padding to match CF_TxnState_INVALID size */
        ],
    };

    CF_CFDP_RxStateDispatch(txn, ph, &STATE_FNS);
    CF_CFDP_ArmInactTimer(txn); /* whenever a packet was received, always arm inactivity timer */
}

// =====================================================================
// PDU construction / send
// =====================================================================

/// C: `CF_Logical_PduBuffer_t *CF_CFDP_ConstructPduHeader(...)`
pub unsafe fn CF_CFDP_ConstructPduHeader(
    txn: *const CF_Transaction_t,
    directive_code: CF_CFDP_FileDirective_t,
    src_eid: CF_EntityId_t,
    dst_eid: CF_EntityId_t,
    towards_sender: bool,
    tsn: CF_TransactionSeq_t,
    silent: bool,
) -> *mut CF_Logical_PduBuffer_t {
    /* directive_code == 0 if file data */
    let ph: *mut CF_Logical_PduBuffer_t;
    let eid_len: u8;

    ph = CF_CFDP_MsgOutGet(txn, silent);

    if !ph.is_null() {
        let hdr: *mut CF_Logical_PduHeader_t = &mut (*ph).pdu_header;

        (*hdr).version   = 1;
        (*hdr).pdu_type  = (directive_code as u8 == 0) as u8;  /* '1' for file data PDU, '0' for directive */
        (*hdr).direction = towards_sender as u8;                /* '1' toward sender, '0' toward receiver */
        (*hdr).txm_mode  = (CF_CFDP_GetClass(txn) == CF_CFDP_Class_t::CF_CFDP_CLASS_1) as u8;

        /* choose the larger of the two EIDs to determine size */
        if src_eid > dst_eid {
            eid_len = CF_CFDP_GetValueEncodedSize(src_eid);
        } else {
            eid_len = CF_CFDP_GetValueEncodedSize(dst_eid);
        }

        (*hdr).eid_length     = eid_len;
        (*hdr).txn_seq_length = CF_CFDP_GetValueEncodedSize(tsn);

        (*hdr).source_eid      = src_eid;
        (*hdr).destination_eid = dst_eid;
        (*hdr).sequence_num    = tsn;

        /* encode the known parts so far (total_size not yet known) */
        CF_CFDP_EncodeHeaderWithoutSize((*ph).penc, hdr);

        /* If directive code is non-zero, encode the file directive header */
        if directive_code as u8 != 0 {
            (*ph).fdirective.directive_code = directive_code;
            CF_CFDP_EncodeFileDirectiveHeader((*ph).penc, &(*ph).fdirective);
        }
    }

    ph
}

/// C: `CFE_Status_t CF_CFDP_SendMd(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_SendMd(txn: *mut CF_Transaction_t) -> CFE_Status_t {
    let ph = CF_CFDP_ConstructPduHeader(
        txn,
        CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_METADATA,
        (*CF_AppData.config_table).local_eid,
        (*(*txn).history).peer_eid,
        false,
        (*(*txn).history).seq_num,
        false,
    );
    let mut sret: CFE_Status_t = CFE_SUCCESS;

    if ph.is_null() {
        sret = CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    } else {
        let md: *mut CF_Logical_PduMd_t = &mut (*ph).int_header.md;

        CF_Assert!(
            (*txn).state == CF_TxnState_t::CF_TxnState_S1
                || (*txn).state == CF_TxnState_t::CF_TxnState_S2
        );

        (*md).size      = (*txn).fsize;
        (*md).close_req = (*txn).flags.com.close_req as u8;

        /* at this point, need to append filenames into md packet */
        /* this does not actually copy here - that is done during encode */
        (*md).source_filename.length = OS_strnlen(
            (*(*txn).history).fnames.src_filename.as_ptr() as *const c_char,
            mem::size_of_val(&(*(*txn).history).fnames.src_filename),
        ) as u8;
        (*md).source_filename.data_ptr =
            (*(*txn).history).fnames.src_filename.as_ptr() as *const u8;
        (*md).dest_filename.length = OS_strnlen(
            (*(*txn).history).fnames.dst_filename.as_ptr() as *const c_char,
            mem::size_of_val(&(*(*txn).history).fnames.dst_filename),
        ) as u8;
        (*md).dest_filename.data_ptr =
            (*(*txn).history).fnames.dst_filename.as_ptr() as *const u8;

        CF_CFDP_EncodeMd((*ph).penc, md);
        CF_CFDP_SetPduLength(ph);
        CF_CFDP_Send((*txn).chan_num, ph);
    }

    sret
}

/// C: `CFE_Status_t CF_CFDP_SendFd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_SendFd(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    /* NOTE: SendFd does not need a call to CF_CFDP_MsgOutGet, as the caller already has it */
    let ret: CFE_Status_t = CFE_SUCCESS;

    /* update PDU length */
    CF_CFDP_SetPduLength(ph);
    CF_CFDP_Send((*txn).chan_num, ph);

    ret
}

/// C: `void CF_CFDP_AppendTlv(CF_Logical_TlvList_t *ptlv_list, CF_CFDP_TlvType_t tlv_type)`
pub unsafe fn CF_CFDP_AppendTlv(ptlv_list: *mut CF_Logical_TlvList_t, tlv_type: CF_CFDP_TlvType_t) {
    let ptlv: *mut CF_Logical_Tlv_t;

    if (*ptlv_list).num_tlv < CF_PDU_MAX_TLV as u8 {
        ptlv = &mut (*ptlv_list).tlv[(*ptlv_list).num_tlv as usize];
        (*ptlv_list).num_tlv += 1;
    } else {
        ptlv = ptr::null_mut();
    }

    if !ptlv.is_null() {
        (*ptlv).r#type = tlv_type;

        if tlv_type == CF_CFDP_TlvType_t::CF_CFDP_TLV_TYPE_ENTITY_ID {
            (*ptlv).data.eid = (*CF_AppData.config_table).local_eid;
            (*ptlv).length = CF_CFDP_GetValueEncodedSize((*ptlv).data.eid);
        } else {
            (*ptlv).data.data_ptr = ptr::null();
            (*ptlv).length = 0;
        }
    }
}

/// C: `CFE_Status_t CF_CFDP_SendEof(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_SendEof(txn: *mut CF_Transaction_t) -> CFE_Status_t {
    let ph = CF_CFDP_ConstructPduHeader(
        txn,
        CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF,
        (*CF_AppData.config_table).local_eid,
        (*(*txn).history).peer_eid,
        false,
        (*(*txn).history).seq_num,
        false,
    );
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    if ph.is_null() {
        ret = CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    } else {
        let eof: *mut CF_Logical_PduEof_t = &mut (*ph).int_header.eof;

        (*eof).cc   = CF_TxnStatus_To_ConditionCode((*(*txn).history).txn_stat);
        (*eof).crc  = (*txn).crc.result;
        (*eof).size = (*txn).fsize;

        if (*eof).cc != CF_CFDP_ConditionCode_t::CF_CFDP_ConditionCode_NO_ERROR {
            CF_CFDP_AppendTlv(&mut (*eof).tlv_list, CF_CFDP_TlvType_t::CF_CFDP_TLV_TYPE_ENTITY_ID);
        }

        CF_CFDP_EncodeEof((*ph).penc, eof);
        CF_CFDP_SetPduLength(ph);
        CF_CFDP_Send((*txn).chan_num, ph);
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_SendAck(CF_Transaction_t *txn, CF_CFDP_FileDirective_t dir_code)`
pub unsafe fn CF_CFDP_SendAck(txn: *mut CF_Transaction_t, dir_code: CF_CFDP_FileDirective_t) -> CFE_Status_t {
    let ph: *mut CF_Logical_PduBuffer_t;
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_Assert!(
        dir_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF
            || dir_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN
    );

    if CF_CFDP_IsSender(txn) {
        ph = CF_CFDP_ConstructPduHeader(
            txn,
            CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_ACK,
            (*CF_AppData.config_table).local_eid,
            (*(*txn).history).peer_eid,
            false,
            (*(*txn).history).seq_num,
            false,
        );
    } else {
        ph = CF_CFDP_ConstructPduHeader(
            txn,
            CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_ACK,
            (*(*txn).history).peer_eid,
            (*CF_AppData.config_table).local_eid,
            true,
            (*(*txn).history).seq_num,
            false,
        );
    }

    if ph.is_null() {
        ret = CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    } else {
        let ack: *mut CF_Logical_PduAck_t = &mut (*ph).int_header.ack;

        (*ack).ack_directive_code = dir_code as u8;
        (*ack).ack_subtype_code  = (dir_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN) as u8;
        (*ack).cc = unsafe { std::mem::transmute((*txn).state_data.peer_cc) };
        (*ack).txn_status        = CF_CFDP_GetAckTxnStatus(txn);

        CF_CFDP_EncodeAck((*ph).penc, ack);
        CF_CFDP_SetPduLength(ph);
        CF_CFDP_Send((*txn).chan_num, ph);
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_SendFin(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_SendFin(txn: *mut CF_Transaction_t) -> CFE_Status_t {
    let ph = CF_CFDP_ConstructPduHeader(
        txn,
        CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN,
        (*(*txn).history).peer_eid,
        (*CF_AppData.config_table).local_eid,
        true,
        (*(*txn).history).seq_num,
        false,
    );
    let ret: CFE_Status_t;

    if ph.is_null() {
        ret = CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    } else {
        ret = CFE_SUCCESS;

        let fin: *mut CF_Logical_PduFin_t = &mut (*ph).int_header.fin;

        (*fin).cc            = CF_TxnStatus_To_ConditionCode((*(*txn).history).txn_stat);
        (*fin).delivery_code = (*txn).state_data.fin_dc;
        (*fin).file_status = unsafe { std::mem::transmute((*txn).state_data.fin_fs) };

        if (*fin).cc != CF_CFDP_ConditionCode_t::CF_CFDP_ConditionCode_NO_ERROR {
            CF_CFDP_AppendTlv(&mut (*fin).tlv_list, CF_CFDP_TlvType_t::CF_CFDP_TLV_TYPE_ENTITY_ID);
        }

        CF_CFDP_EncodeFin((*ph).penc, fin);
        CF_CFDP_SetPduLength(ph);
        CF_CFDP_Send((*txn).chan_num, ph);
    }

    ret
}

/// C: `void CF_CFDP_SendNak(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_SendNak(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) {
    CF_Assert!(CF_CFDP_GetClass(txn) == CF_CFDP_Class_t::CF_CFDP_CLASS_2);

    let nak: *mut CF_Logical_PduNak_t = &mut (*ph).int_header.nak;

    /* NOTE: the caller should have already initialized all the fields.
     * This does not need to add anything more to the NAK here */
    CF_CFDP_EncodeNak((*ph).penc, nak);
    CF_CFDP_SetPduLength(ph);
    CF_CFDP_Send((*txn).chan_num, ph);

    /* The timer needs to be armed after this, lack of response will need a re-nak */
    CF_CFDP_ArmAckTimer(txn);
}

// =====================================================================
// PDU receive / decode
// =====================================================================

/// C: `CFE_Status_t CF_CFDP_RecvPh(uint8 chan_num, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvPh(chan_num: u8, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_Assert!((chan_num as usize) < CF_NUM_CHANNELS);

    if CF_CFDP_DecodeHeader((*ph).pdec, &mut (*ph).pdu_header) != CFE_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_PDU_TRUNCATION_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: PDU rejected due to EID/seq number field truncation\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.channel_hk[chan_num as usize].counters.recv.error += 1;
        ret = CF_ERROR;
    } else if CF_CODEC_IS_OK_DEC(&*(*ph).pdec) && (*ph).pdu_header.large_flag != 0 {
        CFE_EVS_SendEvent!(
            CF_PDU_LARGE_FILE_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: PDU with large file bit received (unsupported)\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.channel_hk[chan_num as usize].counters.recv.error += 1;
        ret = CF_ERROR;
    } else {
        if CF_CODEC_IS_OK_DEC(&*(*ph).pdec) && (*ph).pdu_header.pdu_type == 0 {
            CF_CFDP_DecodeFileDirectiveHeader((*ph).pdec, &mut (*ph).fdirective);
        }

        if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
            CFE_EVS_SendEvent!(
                CF_PDU_SHORT_HEADER_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: PDU too short\0".as_ptr() as *const c_char,
            );
            CF_AppData.hk.Payload.channel_hk[chan_num as usize].counters.recv.error += 1;
            ret = CF_SHORT_PDU_ERROR;
        } else {
            /* PDU is ok, so continue processing */
            CF_AppData.hk.Payload.channel_hk[chan_num as usize].counters.recv.pdu += 1;
        }
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvMd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvMd(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let md: *const CF_Logical_PduMd_t = &(*ph).int_header.md;
    let mut lv_ret: i32;
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeMd((*ph).pdec, &mut (*ph).int_header.md);
    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_MD_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: metadata packet too short\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
        ret = CF_PDU_METADATA_ERROR;
    } else {
        (*txn).flags.com.close_req = (*md).close_req != 0;
        (*txn).fsize = (*md).size;

        lv_ret = CF_CFDP_CopyStringFromLV(
            (*(*txn).history).fnames.src_filename.as_mut_ptr() as *mut c_char,
            mem::size_of_val(&(*(*txn).history).fnames.src_filename),
            &(*md).source_filename,
        );
        if lv_ret < 0 {
            CFE_EVS_SendEvent!(
                CF_PDU_INVALID_SRC_LEN_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: md rejected, invalid source filename length\0".as_ptr() as *const c_char,
            );
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
            ret = CF_PDU_METADATA_ERROR;
        } else {
            lv_ret = CF_CFDP_CopyStringFromLV(
                (*(*txn).history).fnames.dst_filename.as_mut_ptr() as *mut c_char,
                mem::size_of_val(&(*(*txn).history).fnames.dst_filename),
                &(*md).dest_filename,
            );
            if lv_ret < 0 {
                CFE_EVS_SendEvent!(
                    CF_PDU_INVALID_DST_LEN_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF: md rejected, invalid dest filename length\0".as_ptr() as *const c_char,
                );
                CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
                ret = CF_PDU_METADATA_ERROR;
            } else {
                CFE_EVS_SendEvent!(
                    CF_PDU_MD_RECVD_INF_EID,
                    CFE_EVS_EventType_INFORMATION,
                    b"CF: md received\0".as_ptr() as *const c_char,
                );
            }
        }
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvFd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvFd(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeFileDataHeader((*ph).pdec, (*ph).pdu_header.segment_meta_flag != 0, &mut (*ph).int_header.fd);

    /* if the CRC flag is set, need to deduct the size of the CRC from the data - always 32 bits */
    if CF_CODEC_IS_OK_DEC(&*(*ph).pdec) && (*ph).pdu_header.crc_flag as u8 != 0 {
        if (*ph).int_header.fd.data_len < mem::size_of::<u32>() {
            CF_CODEC_SET_DONE_DEC(&mut *(*ph).pdec);
        } else {
            (*ph).int_header.fd.data_len -= mem::size_of::<u32>();
        }
    }

    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_FD_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: filedata PDU too short\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
        ret = CF_SHORT_PDU_ERROR;
    } else if (*ph).pdu_header.segment_meta_flag != 0 {
        CFE_EVS_SendEvent!(
            CF_PDU_FD_UNSUPPORTED_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: filedata PDU with segment metadata received\0".as_ptr() as *const c_char,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
        ret = CF_ERROR;
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvEof(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvEof(_txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeEof((*ph).pdec, &mut (*ph).int_header.eof);

    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_EOF_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: EOF PDU too short\0".as_ptr() as *const c_char,
        );
        ret = CF_SHORT_PDU_ERROR;
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvAck(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvAck(_txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeAck((*ph).pdec, &mut (*ph).int_header.ack);

    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_ACK_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: ACK PDU too short\0".as_ptr() as *const c_char,
        );
        ret = CF_SHORT_PDU_ERROR;
    }

    /* nothing to do for this one, as all fields are bytes */
    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvFin(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvFin(_txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeFin((*ph).pdec, &mut (*ph).int_header.fin);

    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_FIN_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: FIN PDU too short\0".as_ptr() as *const c_char,
        );
        ret = CF_SHORT_PDU_ERROR;
    }

    /* NOTE: right now we don't care about the fault location */
    ret
}

/// C: `CFE_Status_t CF_CFDP_RecvNak(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvNak(_txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    CF_CFDP_DecodeNak((*ph).pdec, &mut (*ph).int_header.nak);

    if !CF_CODEC_IS_OK_DEC(&*(*ph).pdec) {
        CFE_EVS_SendEvent!(
            CF_PDU_NAK_SHORT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: NAK PDU too short\0".as_ptr() as *const c_char,
        );
        ret = CF_SHORT_PDU_ERROR;
    }

    ret
}

/// C: `void CF_CFDP_RecvDrop(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvDrop(txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
    CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.dropped += 1;
}

/// C: `void CF_CFDP_RecvHold(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvHold(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) {
    use crate::cf_cfdp_s::CF_CFDP_S_SubstateRecvFin;
    use crate::cf_cfdp_r::CF_CFDP_R_SubstateRecvEof;

    /* anything received in this state is considered spurious */
    CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.spurious += 1;

    /* Re-ack the final directive if the remote side missed it */
    if (*(*txn).history).dir == CF_Direction_t::CF_Direction_TX {
        if (*ph).fdirective.directive_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN {
            CF_CFDP_S_SubstateRecvFin(txn, ph);
        }
    } else {
        if (*ph).fdirective.directive_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF {
            CF_CFDP_R_SubstateRecvEof(txn, ph);
        }
    }
}

/// C: `void CF_CFDP_RecvInit(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_RecvInit(txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
    /* if dispatching a txn that is still in INIT, it means something went wrong with
     * the early setup.  It is an error so free the transaction */
    CF_CFDP_FinishTransaction(txn, false);
}

// =====================================================================
// Chunk allocation
// =====================================================================

/// C: `CF_ChunkWrapper_t *CF_CFDP_FindUnusedChunks(CF_Channel_t *chan, CF_Direction_t dir)`
unsafe fn CF_CFDP_FindUnusedChunks(chan: *mut CF_Channel_t, dir: CF_Direction_t) -> *mut CF_ChunkWrapper_t {
    let ret: *mut CF_ChunkWrapper_t;
    let chunklist_head: *mut *mut CF_CListNode_t = CF_GetChunkListHead(chan, dir as u8);

    /* this should never be null */
    CF_Assert!(!chunklist_head.is_null());

    if (*chunklist_head).is_null() {
        ret = ptr::null_mut();
    } else {
        ret = container_of!(CF_CList_Pop(chunklist_head), CF_ChunkWrapper_t, cl_node);
    }

    ret
}

/// C: `void CF_CFDP_AllocChunkList(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_AllocChunkList(txn: *mut CF_Transaction_t) {
    /* all RX transactions will need a chunk list to track file segments */
    (*txn).chunks = CF_CFDP_FindUnusedChunks(CF_GetChannelFromTxn(txn), (*(*txn).history).dir);
    if (*txn).chunks.is_null() {
        CFE_EVS_SendEvent!(
            CF_CFDP_NO_CHUNKLIST_AVAIL_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: cannot get chunklist -- abandoning transaction %u\n\0".as_ptr() as *const c_char,
            (*(*txn).history).seq_num as u32,
        );
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_NO_RESOURCE);
    }
}

// =====================================================================
// Transaction setup / lifecycle
// =====================================================================

/// C: `void CF_CFDP_SetupTxTransaction(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_SetupTxTransaction(txn: *mut CF_Transaction_t) {
    /* to be processed this needs a chunklist, get one now */
    if (*txn).chunks.is_null() {
        CF_CFDP_AllocChunkList(txn);
    }

    /* if all is well then proceed to opening the file */
    if !CF_CFDP_TxnIsOK(txn) {
        crate::cf_cfdp_s::CF_CFDP_S_Init(txn);
    }

    /* For TX these txns are on the PEND queue, so they must be moved */
    if !CF_CFDP_TxnIsOK(txn) {
        /* Just clean up (the txn never started, no PDUs were sent, no need for holdover) */
        CF_CFDP_RecycleTransaction(txn);
    } else {
        /* move it to the active queue */
        CF_DequeueTransaction(txn);
        CF_InsertSortPrio(txn, core::mem::transmute::<u8, CF_QueueIdx_t>(CF_QueueIdx_TX));
        CF_CFDP_ArmInactTimer(txn);
    }
}

/// C: `void CF_CFDP_InitTxnTxFile(CF_Transaction_t *txn, ...)`
pub unsafe fn CF_CFDP_InitTxnTxFile(
    txn: *mut CF_Transaction_t,
    cfdp_class: CF_CFDP_Class_t,
    keep: u8,
    chan: u8,
    priority: u8,
) {
    (*txn).chan_num      = chan;
    (*txn).priority      = priority;
    (*txn).keep          = keep;
    (*txn).reliable_mode = cfdp_class != CF_CFDP_Class_t::CF_CFDP_CLASS_1;
    (*txn).state = if (*txn).reliable_mode {
        CF_TxnState_t::CF_TxnState_S2
    } else {
        CF_TxnState_t::CF_TxnState_S1
    };
}

/// C: `void CF_CFDP_TxFile_Initiate(CF_Transaction_t *txn, ...)`
unsafe fn CF_CFDP_TxFile_Initiate(
    txn: *mut CF_Transaction_t,
    cfdp_class: CF_CFDP_Class_t,
    keep: u8,
    chan: u8,
    priority: u8,
    dest_id: CF_EntityId_t,
) {
    CFE_EVS_SendEvent!(
        CF_CFDP_S_START_SEND_INF_EID,
        CFE_EVS_EventType_INFORMATION,
        b"CF: start class %d tx of file %lu:%s -> %lu:%s\0".as_ptr() as *const c_char,
        cfdp_class as i32 + 1,
        (*CF_AppData.config_table).local_eid as u32,
        (*(*txn).history).fnames.src_filename.as_ptr(),
        dest_id as u32,
        (*(*txn).history).fnames.dst_filename.as_ptr(),
    );

    CF_CFDP_InitTxnTxFile(txn, cfdp_class, keep, chan, priority);

    /* Increment sequence number for new transaction */
    CF_AppData.engine.seq_num += 1;

    /* Capture info for history */
    (*(*txn).history).seq_num = CF_AppData.engine.seq_num;
    (*(*txn).history).src_eid = (*CF_AppData.config_table).local_eid;
    (*(*txn).history).peer_eid = dest_id;

    CF_InsertSortPrio(txn, core::mem::transmute::<u8, CF_QueueIdx_t>(CF_QueueIdx_PEND));
}

/// C: `CFE_Status_t CF_CFDP_TxFile(const char *src, const char *dst, ...)`
pub unsafe fn CF_CFDP_TxFile(
    src_filename: *const c_char,
    dst_filename: *const c_char,
    cfdp_class: CF_CFDP_Class_t,
    keep: u8,
    chan_num: u8,
    priority: u8,
    dest_id: CF_EntityId_t,
) -> CFE_Status_t {
    let chan: *mut CF_Channel_t = &mut CF_AppData.engine.channels[chan_num as usize];
    CF_Assert!((chan_num as usize) < CF_NUM_CHANNELS);

    let mut ret: CFE_Status_t = CFE_SUCCESS;
    let txn: *mut CF_Transaction_t;

    if (*chan).num_cmd_tx < CF_MAX_COMMANDED_PLAYBACK_FILES_PER_CHAN as u32 {
        txn = CF_FindUnusedTransaction(&mut CF_AppData.engine.channels[chan_num as usize], CF_Direction_t::CF_Direction_TX);
    } else {
        txn = ptr::null_mut();
    }

    if txn.is_null() {
        CFE_EVS_SendEvent!(
            CF_CFDP_MAX_CMD_TX_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: max number of commanded files reached\0".as_ptr() as *const c_char,
        );
        ret = CF_ERROR;
    } else {
        /* NOTE: the caller ensures the provided src and dst filenames are NULL terminated */
        let src_dst = &mut (*(*txn).history).fnames.src_filename;
        let src_len = src_dst.len();
        libc_strncpy(src_dst.as_mut_ptr() as *mut c_char, src_filename, src_len - 1);
        src_dst[src_len - 1] = 0;

        let dst_dst = &mut (*(*txn).history).fnames.dst_filename;
        let dst_len = dst_dst.len();
        libc_strncpy(dst_dst.as_mut_ptr() as *mut c_char, dst_filename, dst_len - 1);
        dst_dst[dst_len - 1] = 0;

        CF_CFDP_TxFile_Initiate(txn, cfdp_class, keep, chan_num, priority, dest_id);

        (*chan).num_cmd_tx += 1;
        (*txn).flags.tx.cmd_tx = true;
    }

    ret
}

/// C: `CF_Transaction_t *CF_CFDP_StartRxTransaction(uint8 chan_num)`
pub unsafe fn CF_CFDP_StartRxTransaction(chan_num: u8) -> *mut CF_Transaction_t {
    let chan: *mut CF_Channel_t = &mut CF_AppData.engine.channels[chan_num as usize];
    let txn: *mut CF_Transaction_t;

    if CF_AppData.hk.Payload.channel_hk[chan_num as usize].q_size[CF_QueueIdx_RX as usize] < CF_MAX_SIMULTANEOUS_RX as u16 {
        txn = CF_FindUnusedTransaction(chan, CF_Direction_t::CF_Direction_RX);
    } else {
        txn = ptr::null_mut();
    }

    if !txn.is_null() {
        /* At this point all we know is that this is an RX transaction */
        (*txn).flags.com.q_index = CF_QueueIdx_RX;
        CF_CList_InsertBack_Ex(chan, (*txn).flags.com.q_index as u8, &mut (*txn).cl_node);
    }

    txn
}

/// C: `void CF_CFDP_SetupRxTransaction(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_SetupRxTransaction(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t) {
    /* only RX transactions dare tread here */
    (*(*txn).history).seq_num = (*ph).pdu_header.sequence_num;

    /* peer_eid is always the remote partner. src_eid is always the transaction source.
     * in this case, they are the same */
    (*(*txn).history).peer_eid = (*ph).pdu_header.source_eid;
    (*(*txn).history).src_eid  = (*ph).pdu_header.source_eid;

    /* all RX transactions will need a chunk list to track file segments */
    if (*txn).chunks.is_null() {
        CF_CFDP_AllocChunkList(txn);
    }

    /* NOTE: RX transactions are created on-demand by received PDUs, so these always
     * must be retained even if it has already failed. */
    if !CF_CFDP_TxnIsOK(txn) {
        (*txn).state = CF_TxnState_t::CF_TxnState_HOLD;
    } else {
        (*txn).reliable_mode = (*ph).pdu_header.txm_mode == 0;
        (*txn).state = if (*txn).reliable_mode {
            CF_TxnState_t::CF_TxnState_R2
        } else {
            CF_TxnState_t::CF_TxnState_R1
        };

        CF_CFDP_R_Init(txn);
    }

    /* this timer is always needed to eventually recycle this txn */
    CF_CFDP_ArmInactTimer(txn);
}

/// C: `void CF_CFDP_ReceivePdu(CF_Channel_t *chan, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_ReceivePdu(chan: *mut CF_Channel_t, ph: *mut CF_Logical_PduBuffer_t) {
    let chan_num: u8 = ((chan as usize - CF_AppData.engine.channels.as_ptr() as usize)
        / mem::size_of::<CF_Channel_t>()) as u8;

    /* This decodes the header in the PDU. If it fails it sends the relevant event
     * and increments any necessary counters. */
    if CF_CFDP_RecvPh(chan_num, ph) != CFE_SUCCESS {
        /* drop it, nothing more to do */
        return;
    }

    /* got a valid PDU -- look it up by sequence number */
    let mut txn: *mut CF_Transaction_t = CF_FindTransactionBySequenceNumber(
        chan,
        (*ph).pdu_header.sequence_num,
        (*ph).pdu_header.source_eid,
    );

    if txn.is_null() {
        /* if no match found, then check if we are the destination entity id.
         * If so then this would be the first PDU of an RX transaction */
        if (*ph).pdu_header.destination_eid == (*CF_AppData.config_table).local_eid {
            /* we didn't find a match, so assign it to a transaction */
            txn = CF_CFDP_StartRxTransaction(chan_num);
            if txn.is_null() {
                CFE_EVS_SendEvent!(
                    CF_CFDP_RX_DROPPED_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF: dropping packet from %lu transaction number 0x%08lx due max RX transactions reached\0".as_ptr() as *const c_char,
                    (*ph).pdu_header.source_eid as u32,
                    (*ph).pdu_header.sequence_num as u32,
                );
            } else {
                /* set up the new transaction according to fields in the PDU header */
                CF_CFDP_SetupRxTransaction(txn, ph);
            }
        } else {
            CFE_EVS_SendEvent!(
                CF_CFDP_INVALID_DST_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: dropping packet for invalid destination eid 0x%lx\0".as_ptr() as *const c_char,
                (*ph).pdu_header.destination_eid as u32,
            );
        }
    }

    if !txn.is_null() {
        /* found one! Send it to the transaction state processor */
        CF_Assert!((*txn).state != CF_TxnState_t::CF_TxnState_UNDEF);
        CF_CFDP_DispatchRecv(txn, ph);
    }
}

/// C: `CFE_Status_t CF_CFDP_PlaybackDir_Initiate(...)`
unsafe fn CF_CFDP_PlaybackDir_Initiate(
    pb: *mut CF_Playback_t,
    src_filename: *const c_char,
    dst_filename: *const c_char,
    cfdp_class: CF_CFDP_Class_t,
    keep: u8,
    chan: u8,
    priority: u8,
    dest_id: CF_EntityId_t,
) -> CFE_Status_t {
    /* make sure the directory can be open */
    let ret = OS_DirectoryOpen(&mut (*pb).dir_id, src_filename);
    if ret != OS_SUCCESS {
        CFE_EVS_SendEvent!(
            CF_CFDP_OPENDIR_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: failed to open playback directory %s, error=%ld\0".as_ptr() as *const c_char,
            src_filename,
            ret as i64,
        );
        CF_AppData.hk.Payload.channel_hk[chan as usize].counters.fault.directory_read += 1;
    } else {
        (*pb).diropen = true;
        (*pb).busy = true;
        (*pb).keep = keep != 0;
        (*pb).priority = priority;
        (*pb).dest_id = dest_id;
        (*pb).cfdp_class = cfdp_class;

        let src_dst = &mut (*pb).fnames.src_filename;
        let src_len = src_dst.len();
        libc_strncpy(src_dst.as_mut_ptr() as *mut c_char, src_filename, src_len - 1);
        src_dst[src_len - 1] = 0;

        let dst_dst = &mut (*pb).fnames.dst_filename;
        let dst_len = dst_dst.len();
        libc_strncpy(dst_dst.as_mut_ptr() as *mut c_char, dst_filename, dst_len - 1);
        dst_dst[dst_len - 1] = 0;
    }

    ret
}

/// C: `CFE_Status_t CF_CFDP_PlaybackDir(...)`
pub unsafe fn CF_CFDP_PlaybackDir(
    src_filename: *const c_char,
    dst_filename: *const c_char,
    cfdp_class: CF_CFDP_Class_t,
    keep: u8,
    chan_num: u8,
    priority: u8,
    dest_id: CF_EntityId_t,
) -> CFE_Status_t {
    let mut i: usize = 0;
    let mut pb: *mut CF_Playback_t = ptr::null_mut();

    while i < CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN {
        pb = &mut CF_AppData.engine.channels[chan_num as usize].playback[i];
        if !(*pb).busy {
            break;
        }
        i += 1;
    }

    if i == CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN {
        CFE_EVS_SendEvent!(
            CF_CFDP_DIR_SLOT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF: no playback dir slot available\0".as_ptr() as *const c_char,
        );
        return CF_ERROR;
    }

    CF_CFDP_PlaybackDir_Initiate(pb, src_filename, dst_filename, cfdp_class, keep, chan_num, priority, dest_id)
}

/// C: `void CF_CFDP_ProcessPlaybackDirectory(CF_Channel_t *chan, CF_Playback_t *pb)`
pub unsafe fn CF_CFDP_ProcessPlaybackDirectory(chan: *mut CF_Channel_t, pb: *mut CF_Playback_t) {
    let mut dirent: os_dirent_t = mem::zeroed();
    let mut status: i32;

    while (*pb).diropen && ((*pb).num_ts < CF_NUM_TRANSACTIONS_PER_PLAYBACK as u16) {
        if (*pb).pending_file[0] == 0 {
            CFE_ES_PerfLogEntry(CF_PERF_ID_DIRREAD);
            status = OS_DirectoryRead((*pb).dir_id, &mut dirent as *mut os_dirent_t as *mut c_void);
            CFE_ES_PerfLogExit(CF_PERF_ID_DIRREAD);

            if status != OS_SUCCESS {
                OS_DirectoryClose((*pb).dir_id);
                (*pb).diropen = false;
                break;
            }

            /* skip . and .. */
            let fname = dirent.FileName.as_ptr();
            if libc_strcmp(fname, b".\0".as_ptr() as *const c_char) == 0
                || libc_strcmp(fname, b"..\0".as_ptr() as *const c_char) == 0
            {
                continue;
            }

            let pf = &mut (*pb).pending_file;
            libc_strncpy(pf.as_mut_ptr() as *mut c_char, fname, pf.len() - 1);
            pf[pf.len() - 1] = 0;
        } else {
            let txn = CF_FindUnusedTransaction(chan, CF_Direction_t::CF_Direction_TX);
            if txn.is_null() {
                break;
            }

            let chan_index = (chan as usize - &CF_AppData.engine.channels[0] as *const _ as usize)
                / mem::size_of::<CF_Channel_t>();

            libc_snprintf!(
                (*(*txn).history).fnames.src_filename.as_mut_ptr() as *mut c_char,
                (*(*txn).history).fnames.src_filename.len(),
                b"%s/%s\0".as_ptr() as *const c_char,
                (*pb).fnames.src_filename.as_ptr(),
                (*pb).pending_file.as_ptr(),
            );
            libc_snprintf!(
                (*(*txn).history).fnames.dst_filename.as_mut_ptr() as *mut c_char,
                (*(*txn).history).fnames.dst_filename.len(),
                b"%s/%s\0".as_ptr() as *const c_char,
                (*pb).fnames.dst_filename.as_ptr(),
                (*pb).pending_file.as_ptr(),
            );

            CF_CFDP_TxFile_Initiate(
                txn,
                (*pb).cfdp_class,
                (*pb).keep as u8,
                chan_index as u8,
                (*pb).priority,
                (*pb).dest_id,
            );

            (*txn).pb = pb;
            (*pb).num_ts += 1;
            (*pb).pending_file[0] = 0; /* continue reading dir */
        }
    }

    if !(*pb).diropen && (*pb).num_ts == 0 {
        (*pb).busy = false;
    }
}

/// C: `void CF_CFDP_UpdatePollPbCounted(CF_Playback_t *pb, int up, uint8 *counter)`
unsafe fn CF_CFDP_UpdatePollPbCounted(pb: *mut CF_Playback_t, up: i32, counter: *mut u8) {
    let up_bool = up != 0;
    if (*pb).counted != up_bool {
        /* only handle on state change */
        (*pb).counted = up_bool;

        if up_bool {
            *counter += 1;
        } else {
            CF_Assert!(*counter != 0); /* sanity check it isn't zero */
            *counter -= 1;
        }
    }
}

/// C: `void CF_CFDP_ProcessPlaybackDirectories(CF_Channel_t *chan)`
pub unsafe fn CF_CFDP_ProcessPlaybackDirectories(chan: *mut CF_Channel_t) {
    let chan_index = (chan as usize - &CF_AppData.engine.channels[0] as *const _ as usize)
        / mem::size_of::<CF_Channel_t>();

    for i in 0..CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN {
        CF_CFDP_ProcessPlaybackDirectory(chan, &mut (*chan).playback[i]);
        CF_CFDP_UpdatePollPbCounted(
            &mut (*chan).playback[i],
            (*chan).playback[i].busy as i32,
            &mut CF_AppData.hk.Payload.channel_hk[chan_index].playback_counter,
        );
    }
}

/// C: `void CF_CFDP_ProcessPollingDirectories(CF_Channel_t *chan)`
pub unsafe fn CF_CFDP_ProcessPollingDirectories(chan: *mut CF_Channel_t) {
    for i in 0..CF_MAX_POLLING_DIR_PER_CHAN {
        let poll: *mut CF_Poll_t = &mut (*chan).poll[i];
        let chan_index = (chan as usize - &CF_AppData.engine.channels[0] as *const _ as usize)
            / mem::size_of::<CF_Channel_t>();
        let cc: *const CF_ChannelConfig_t = &(*CF_AppData.config_table).chan[chan_index];
        let pd: *const CF_PollDir_t = &(*cc).polldir[i];
        let mut count_check: i32 = 0;

        if (*pd).enabled != 0 {
            if !(*poll).pb.busy && (*poll).pb.num_ts == 0 {
                if !(*poll).timer_set && (*pd).interval_sec != 0 {
                    /* timer was not set, so set it now */
                    CF_Timer_InitRelSec(&mut (*poll).interval_timer, (*pd).interval_sec, (*CF_AppData.config_table).ticks_per_second);
                    (*poll).timer_set = true;
                } else if CF_Timer_Expired(&mut (*poll).interval_timer) {
                    /* the timer has expired */
                    let ret = CF_CFDP_PlaybackDir_Initiate(
                        &mut (*poll).pb,
                        (*pd).src_dir.as_ptr() as *const c_char,
                        (*pd).dst_dir.as_ptr() as *const c_char,
                        (*pd).cfdp_class,
                        0,
                        chan_index as u8,
                        (*pd).priority,
                        (*pd).dest_eid,
                    );
                    if ret == 0 {
                        (*poll).timer_set = false;
                    } else {
                        /* error occurred in playback directory, so reset the timer */
                        CF_Timer_InitRelSec(&mut (*poll).interval_timer, (*pd).interval_sec, (*CF_AppData.config_table).ticks_per_second);
                    }
                } else {
                    CF_Timer_Tick(&mut (*poll).interval_timer);
                }
            } else {
                /* playback is active, so step it */
                CF_CFDP_ProcessPlaybackDirectory(chan, &mut (*poll).pb);
            }

            count_check = 1;
        }

        CF_CFDP_UpdatePollPbCounted(
            &mut (*poll).pb,
            count_check,
            &mut CF_AppData.hk.Payload.channel_hk[chan_index].poll_counter,
        );
    }
}

/// C: `void CF_CFDP_S_Tick_NewData(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_Tick_NewData(txn: *mut CF_Transaction_t) {
    let chan: *mut CF_Channel_t;

    if !(*txn).flags.com.suspended
        && (*txn).state_data.sub_state == CF_TxSubState_t::CF_TxSubState_DATA_NORMAL as u8
    {
        /* this is a candidate for sending data */
        chan = CF_GetChannelFromTxn(txn);
    } else {
        chan = ptr::null_mut();
    }

    if !chan.is_null() {
        loop {
            let last_outgoing_counter = (*chan).outgoing_counter;

            CFE_ES_PerfLogEntry(CF_PERF_ID_PDUSENT);
            CF_CFDP_S_SubstateSendFileData(txn);
            CFE_ES_PerfLogExit(CF_PERF_ID_PDUSENT);

            if last_outgoing_counter == (*chan).outgoing_counter {
                break;
            }
        }
    }
}

/// C: `bool CF_CFDP_StartFirstPending(CF_Channel_t *chan)`
pub unsafe fn CF_CFDP_StartFirstPending(chan: *mut CF_Channel_t) -> bool {
    if (*chan).qs[CF_QueueIdx_PEND as usize].is_null() {
        /* nothing pending */
        return false;
    }

    let txn: *mut CF_Transaction_t =
        container_of!((*chan).qs[CF_QueueIdx_PEND as usize], CF_Transaction_t, cl_node);

    CF_CFDP_SetupTxTransaction(txn);

    /* this did something */
    true
}

/// C: `CF_CListTraverse_Status_t CF_CFDP_DoTick(CF_CListNode_t *node, void *context)`
pub unsafe fn CF_CFDP_DoTick(
    node: *mut CF_CListNode_t,
    context: *mut u8,
) -> CF_CListTraverse_Status_t {
    let args: *mut CF_CFDP_Tick_args_t = context as *mut CF_CFDP_Tick_args_t;
    let txn: *mut CF_Transaction_t = container_of!(node, CF_Transaction_t, cl_node);

    if !(*txn).flags.com.suspended {
        if let Some(fn_ptr) = (*args).fn_ptr {
            fn_ptr(txn);
        }
    }

    CF_CListTraverse_Status_t::CF_CListTraverse_Status_CONTINUE
}

/// C: `void CF_CFDP_CycleEngine(void)`
pub unsafe fn CF_CFDP_CycleEngine() {
    if CF_AppData.engine.enabled {
        for i in 0..CF_NUM_CHANNELS {
            let chan: *mut CF_Channel_t = &mut CF_AppData.engine.channels[i];

            (*chan).outgoing_counter = 0;
            (*chan).tx_blocked = false;

            /* consume all received messages, even if channel is frozen */
            crate::cf_cfdp_sbintf::CF_CFDP_ReceiveMessage(chan);

            if CF_AppData.hk.Payload.channel_hk[i].frozen == 0 {
                /* cycle all transactions (tick) */
                CF_CFDP_TickTransactions(chan);

                CF_CFDP_ProcessPlaybackDirectories(chan);
                CF_CFDP_ProcessPollingDirectories(chan);
            }
        }
    }
}

/// C: `void CF_CFDP_TickTransactions(CF_Channel_t *chan)`
pub unsafe fn CF_CFDP_TickTransactions(chan: *mut CF_Channel_t) {
    let mut targs: CF_CFDP_Tick_args_t = mem::zeroed();
    let mut q_id: usize;
    let mut last_counter: u32;
    let mut curr_state: u8;

    targs.chan = chan;
    targs.resume_point = (*chan).tick_resume as *mut CF_CListNode_t;
    (*chan).tick_resume = ptr::null_mut();

    curr_state = CF_TickState_INIT;

    while curr_state < CF_TickState_COMPLETE {
        last_counter = (*chan).outgoing_counter;

        match curr_state {
            CF_TickState_RX_STATE => {
                q_id = CF_QueueIdx_RX as usize;
                targs.fn_ptr = Some(CF_CFDP_R_Tick as unsafe fn(*mut CF_Transaction_t));
            }
            CF_TickState_TX_STATE => {
                q_id = CF_QueueIdx_TX as usize;
                targs.fn_ptr = Some(CF_CFDP_S_Tick as unsafe fn(*mut CF_Transaction_t));
            }
            CF_TickState_TX_NAK => {
                q_id = CF_QueueIdx_TX as usize;
                targs.fn_ptr = Some(CF_CFDP_S_Tick_Nak as unsafe fn(*mut CF_Transaction_t));
            }
            CF_TickState_TX_FILEDATA => {
                q_id = CF_QueueIdx_TX as usize;
                targs.fn_ptr = Some(CF_CFDP_S_Tick_NewData as unsafe fn(*mut CF_Transaction_t));
            }
            _ => {
                targs.fn_ptr = None;
                q_id = 0;
            }
        }

        if targs.fn_ptr.is_some() {
            CF_CList_Traverse(
                (*chan).qs[q_id],
                CF_CFDP_DoTick,
                &mut targs as *mut _ as *mut u8,
            );
        }

        /* If blocked, stop */
        if (*chan).tx_blocked {
            break;
        }

        /* transition to next state */
        match curr_state {
            CF_TickState_TX_NAK => {
                /* advance state only if the last pass sent nothing */
                if last_counter == (*chan).outgoing_counter {
                    curr_state += 1;
                }
            }
            CF_TickState_TX_PEND => {
                CF_CFDP_StartFirstPending(chan);
                /* always finish after this */
                curr_state = CF_TickState_COMPLETE;
            }
            _ => {
                curr_state += 1;
            }
        }
    }
}

/// C: `void CF_CFDP_FinishTransaction(CF_Transaction_t *txn, bool keep_history)`
pub unsafe fn CF_CFDP_FinishTransaction(txn: *mut CF_Transaction_t, keep_history: bool) {
    if (*txn).flags.com.q_index == CF_QueueIdx_FREE {
        CFE_EVS_SendEvent!(
            CF_RESET_FREED_XACT_DBG_EID,
            CFE_EVS_EventType_DEBUG,
            b"CF: attempt to reset a transaction that has already been freed\0".as_ptr() as *const c_char,
        );
        return;
    }

    let chan = CF_GetChannelFromTxn(txn);
    CF_Assert!(!chan.is_null());

    if OS_ObjectIdDefined((*txn).fd) {
        CF_WrappedClose((*txn).fd);
        (*txn).fd = OS_OBJECT_ID_UNDEFINED;
    }

    if !(*txn).history.is_null() {
        CF_CFDP_SendEotPkt(txn);

        /* extra bookkeeping for tx direction only */
        if (*(*txn).history).dir == CF_Direction_t::CF_Direction_TX && (*txn).flags.tx.cmd_tx {
            CF_Assert!((*chan).num_cmd_tx > 0);
            (*chan).num_cmd_tx -= 1;
        }

        (*txn).flags.com.keep_history = keep_history;
    }

    if !(*txn).pb.is_null() {
        /* a playback's transaction is now done, decrement the playback counter */
        CF_Assert!((*(*txn).pb).num_ts > 0);
        (*(*txn).pb).num_ts -= 1;
    }

    /* Put this transaction into the holdover state, inactivity timer will recycle it */
    (*txn).state = CF_TxnState_t::CF_TxnState_HOLD;
    CF_CFDP_ArmInactTimer(txn);
}

/// C: `void CF_CFDP_RecycleTransaction(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_RecycleTransaction(txn: *mut CF_Transaction_t) {
    /* If file still open, close it (not expected, log it) */
    if OS_ObjectIdDefined((*txn).fd) {
        CFE_ES_WriteToSysLog!(
            b"CF: RecycleTransaction closing dangling file handle: %lu\n\0".as_ptr() as *const c_char,
            OS_ObjectIdToInteger((*txn).fd) as u32,
        );
        CF_WrappedClose((*txn).fd);
        (*txn).fd = OS_OBJECT_ID_UNDEFINED;
    }

    CF_DequeueTransaction(txn); /* this makes it "float" (not in any queue) */

    let chan = CF_GetChannelFromTxn(txn);

    /* this should always be */
    if !chan.is_null() && !(*txn).history.is_null() {
        if !(*txn).chunks.is_null() {
            let chunklist_head = CF_GetChunkListHead(chan, (*(*txn).history).dir as u8);
            if !chunklist_head.is_null() {
                CF_CList_InsertBack(chunklist_head, &mut (*(*txn).chunks).cl_node);
                (*txn).chunks = ptr::null_mut();
            }
        }

        let hist_destq: u8;
        if (*txn).flags.com.keep_history {
            /* move transaction history to history queue */
            hist_destq = CF_QueueIdx_HIST;
        } else {
            hist_destq = CF_QueueIdx_HIST_FREE;
        }
        CF_CList_InsertBack_Ex(chan, hist_destq, &mut (*(*txn).history).cl_node);
        (*txn).history = ptr::null_mut();
    }

    /* this wipes it and puts it back onto the list to be found by
     * CF_FindUnusedTransaction().  Need to preserve the chan_num
     * and keep it associated with this channel, though. */
    CF_FreeTransaction(txn, (*txn).chan_num);
}

/// C: `void CF_CFDP_SetTxnStatus(CF_Transaction_t *txn, CF_TxnStatus_t txn_stat)`
pub unsafe fn CF_CFDP_SetTxnStatus(txn: *mut CF_Transaction_t, txn_stat: CF_TxnStatus_t) {
    (*(*txn).history).txn_stat = txn_stat;
}

/// C: `CF_TxnStatus_t CF_CFDP_GetTxnStatus(const CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_GetTxnStatus(txn: *const CF_Transaction_t) -> CF_TxnStatus_t {
    (*(*txn).history).txn_stat
}

/// C: `bool CF_CFDP_TxnIsOK(const CF_Transaction_t *txn)` — returns true if no error
pub unsafe fn CF_CFDP_TxnIsOK(txn: *const CF_Transaction_t) -> bool {
    (*(*txn).history).txn_stat == CF_TxnStatus_t::CF_TxnStatus_UNDEFINED
}

/// C: `void CF_CFDP_SendEotPkt(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_SendEotPkt(txn: *mut CF_Transaction_t) {
    let buf_ptr = CFE_SB_AllocateMessageBuffer(mem::size_of::<CF_EotPacket_t>());
    if !buf_ptr.is_null() {
        let eot: *mut CF_EotPacket_t = buf_ptr as *mut CF_EotPacket_t;

        CFE_MSG_Init(
            eot as *mut CFE_MSG_Message_t,
            CFE_SB_ValueToMsgId(CF_EOT_TLM_MID),
            mem::size_of::<CF_EotPacket_t>(),
        );

        (*eot).Payload.channel = (*txn).chan_num as u32;
        (*eot).Payload.direction = (*(*txn).history).dir as u32;
        (*eot).Payload.fnames = (*(*txn).history).fnames.clone();
        (*eot).Payload.state = (*txn).state as u32;
        (*eot).Payload.txn_stat = (*(*txn).history).txn_stat as u32;
        (*eot).Payload.src_eid = (*(*txn).history).src_eid;
        (*eot).Payload.peer_eid = (*(*txn).history).peer_eid;
        (*eot).Payload.seq_num = (*(*txn).history).seq_num;
        (*eot).Payload.fsize = (*txn).fsize;
        (*eot).Payload.crc_result = (*txn).crc.result;

        CFE_SB_TimeStampMsg(eot as *mut CFE_MSG_Message_t);
        CFE_SB_TransmitBuffer(buf_ptr, true);
    }
}

/// C: `int CF_CFDP_CopyStringFromLV(char *buf, size_t buf_maxsz, const CF_Logical_Lv_t *src_lv)`
pub unsafe fn CF_CFDP_CopyStringFromLV(
    buf: *mut c_char,
    buf_maxsz: usize,
    src_lv: *const CF_Logical_Lv_t,
) -> i32 {
    if ((*src_lv).length as usize) < buf_maxsz {
        ptr::copy_nonoverlapping((*src_lv).data_ptr as *const u8, buf as *mut u8, (*src_lv).length as usize);
        *buf.add((*src_lv).length as usize) = 0;
        return (*src_lv).length as i32;
    }

    /* ensure output is empty */
    *buf = 0;
    CF_ERROR /* invalid len in lv? */
}

/// C: `void CF_CFDP_CancelTransaction(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_CancelTransaction(txn: *mut CF_Transaction_t) {
    if !(*txn).flags.com.canceled {
        /* Just set the flag, state machine will close it out */
        (*txn).flags.com.canceled = true;
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_CANCEL_REQUEST_RECEIVED);
    }
}

/// C: `CF_CListTraverse_Status_t CF_CFDP_CloseFiles(CF_CListNode_t *node, void *context)`
pub unsafe fn CF_CFDP_CloseFiles(
    node: *mut CF_CListNode_t,
    _context: *mut c_void,
) -> CF_CListTraverse_Status_t {
    let txn: *mut CF_Transaction_t = container_of!(node, CF_Transaction_t, cl_node);
    if OS_ObjectIdDefined((*txn).fd) {
        CF_WrappedClose((*txn).fd);
    }
    CF_CLIST_CONT
}

/// C: `void CF_CFDP_DisableEngine(void)`
pub unsafe fn CF_CFDP_DisableEngine() {
    const CLOSE_QUEUES: [u8; 2] = [CF_QueueIdx_RX, CF_QueueIdx_TX];

    CF_AppData.engine.enabled = false;

    for i in 0..CF_NUM_CHANNELS {
        let chan: *mut CF_Channel_t = &mut CF_AppData.engine.channels[i];

        /* first, close all active files */
        for j in 0..CLOSE_QUEUES.len() {
            CF_CList_Traverse(
                (*chan).qs[CLOSE_QUEUES[j] as usize],
                core::mem::transmute::<_, CF_CListFn_t>(CF_CFDP_CloseFiles as unsafe fn(*mut CF_CListNode_t, *mut c_void) -> CF_CListTraverse_Status_t),
                ptr::null_mut(),
            );
        }

        /* any playback directories need to have their directory ids closed */
        for j in 0..CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN {
            if (*chan).playback[j].busy {
                OS_DirectoryClose((*chan).playback[j].dir_id);
            }
        }

        for j in 0..CF_MAX_POLLING_DIR_PER_CHAN {
            if (*chan).poll[j].pb.busy {
                OS_DirectoryClose((*chan).poll[j].pb.dir_id);
            }
        }

        /* finally all queue counters must be reset */
        ptr::write_bytes(
            &mut CF_AppData.hk.Payload.channel_hk[i].q_size as *mut _ as *mut u8,
            0,
            mem::size_of_val(&CF_AppData.hk.Payload.channel_hk[i].q_size),
        );

        CFE_SB_DeletePipe((*chan).pipe);
    }
}

/// C: `void CF_CFDP_GetTempName(const CF_History_t *hist, char *buf, size_t size)`
pub unsafe fn CF_CFDP_GetTempName(
    hist: *const CF_History_t,
    buf: *mut c_char,
    size: usize,
) {
    libc_snprintf!(
        buf,
        size,
        b"%.*s/%lu_%lu.tmp\0".as_ptr() as *const c_char,
        CF_FILENAME_MAX_PATH as i32 - 1,
        (*CF_AppData.config_table).tmp_dir.as_ptr(),
        (*hist).src_eid as u32,
        (*hist).seq_num as u32,
    );
}

/// C: `CFE_Status_t CF_CFDP_InitEngine(void)`
pub unsafe fn CF_CFDP_InitEngine() -> CFE_Status_t {
    let mut txn: *mut CF_Transaction_t = CF_AppData.engine.transactions.as_mut_ptr();
    let mut cw: *mut CF_ChunkWrapper_t = CF_AppData.engine.chunks.as_mut_ptr();
    let mut ret: CFE_Status_t = CFE_SUCCESS;
    let mut chunk_mem_offset: usize = 0;
    let mut nbuf: [u8; 64] = [0u8; 64];

    /* zero the engine struct */
    ptr::write_bytes(
        &mut CF_AppData.engine as *mut _ as *mut u8,
        0,
        mem::size_of::<CF_Engine_t>(),
    );

    /* Ensure that the temp directory exists */
    OS_mkdir((*CF_AppData.config_table).tmp_dir.as_ptr() as *const c_char, 0);

    'outer: for i in 0..CF_NUM_CHANNELS {
        libc_snprintf!(
            nbuf.as_mut_ptr() as *mut c_char,
            nbuf.len() - 1,
            b"%s%d\0".as_ptr() as *const c_char,
            CF_CHANNEL_PIPE_PREFIX.as_ptr(),
            i as i32,
        );
        ret = CFE_SB_CreatePipe(
            &mut CF_AppData.engine.channels[i].pipe,
            (*CF_AppData.config_table).chan[i].pipe_depth_input,
            nbuf.as_ptr() as *const c_char,
        );
        if ret != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_CR_CHANNEL_PIPE_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: failed to create pipe %s, returned 0x%08lx\0".as_ptr() as *const c_char,
                nbuf.as_ptr(),
                ret as u32,
            );
            break;
        }

        ret = CFE_SB_SubscribeLocal(
            CFE_SB_ValueToMsgId((*CF_AppData.config_table).chan[i].mid_input),
            CF_AppData.engine.channels[i].pipe,
            (*CF_AppData.config_table).chan[i].pipe_depth_input,
        );
        if ret != CFE_SUCCESS {
            CFE_EVS_SendEvent!(
                CF_INIT_SUB_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF: failed to subscribe to MID 0x%lx, returned 0x%08lx\0".as_ptr() as *const c_char,
                (*CF_AppData.config_table).chan[i].mid_input as u32,
                ret as u32,
            );
            break;
        }

        if (*CF_AppData.config_table).chan[i].sem_name[0] != 0 {
            ret = OS_ERR_NAME_NOT_FOUND;
            for _j in 0..CF_STARTUP_SEM_MAX_RETRIES {
                ret = OS_CountSemGetIdByName(
                    &mut CF_AppData.engine.channels[i].sem_id,
                    (*CF_AppData.config_table).chan[i].sem_name.as_ptr() as *const c_char,
                );
                if ret != OS_ERR_NAME_NOT_FOUND {
                    break;
                }
                OS_TaskDelay(CF_STARTUP_SEM_TASK_DELAY);
            }

            if ret != OS_SUCCESS {
                CFE_EVS_SendEvent!(
                    CF_INIT_SEM_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF: failed to get sem id for name %s, error=%ld\0".as_ptr() as *const c_char,
                    (*CF_AppData.config_table).chan[i].sem_name.as_ptr(),
                    ret as i64,
                );
                break;
            }
        }

        for _j in 0..CF_NUM_TRANSACTIONS_PER_CHANNEL {
            CF_FreeTransaction(txn, i as u8);

            for k in 0..CF_Direction_NUM {
                let list_head = CF_GetChunkListHead(&mut CF_AppData.engine.channels[i], k as u8);

                CF_Assert!((chunk_mem_offset + CF_DIR_MAX_CHUNKS as usize) <= CF_NUM_CHUNKS_ALL_CHANNELS);
                CF_ChunkListInit(
                    &mut (*cw).chunks,
                    CF_DIR_MAX_CHUNKS as u32,
                    &mut CF_AppData.engine.chunk_mem[chunk_mem_offset],
                );
                chunk_mem_offset += CF_DIR_MAX_CHUNKS as usize;
                CF_CList_InitNode(&mut (*cw).cl_node);
                CF_CList_InsertBack(list_head, &mut (*cw).cl_node);
                cw = cw.add(1);
            }
            txn = txn.add(1);
        }

        for j in 0..CF_NUM_HISTORIES_PER_CHANNEL {
            let history: *mut CF_History_t =
                &mut CF_AppData.engine.histories[(i * CF_NUM_HISTORIES_PER_CHANNEL) + j];
            CF_CList_InitNode(&mut (*history).cl_node);
            CF_CList_InsertBack_Ex(
                &mut CF_AppData.engine.channels[i],
                CF_QueueIdx_HIST_FREE,
                &mut (*history).cl_node,
            );
        }
    }

    if ret == CFE_SUCCESS {
        CF_AppData.engine.enabled = true;
    }

    ret
}

/// C: `void CF_CFDP_CompleteTick(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_CompleteTick(txn: *mut CF_Transaction_t) {
    /* Inactivity timer check — if expired, recycle the transaction */
    if CF_Timer_Expired(&(*txn).inactivity_timer) {
        CF_CFDP_RecycleTransaction(txn);
    } else {
        CF_Timer_Tick(&mut (*txn).inactivity_timer);
    }
}

/// C: `CF_CFDP_AckTxnStatus_t CF_CFDP_GetAckTxnStatus(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_GetAckTxnStatus(txn: *const CF_Transaction_t) -> CF_CFDP_AckTxnStatus_t {
    if (*(*txn).history).txn_stat == CF_TxnStatus_t::CF_TxnStatus_UNDEFINED {
        CF_CFDP_AckTxnStatus_t::CF_CFDP_AckTxnStatus_ACTIVE
    } else {
        CF_CFDP_AckTxnStatus_t::CF_CFDP_AckTxnStatus_TERMINATED
    }
}

/// C: `void CF_CFDP_SetPduLength(CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_SetPduLength(ph: *mut CF_Logical_PduBuffer_t) {
    /* final position of the encoder state should reflect the entire PDU length */
    let final_pos: u16 = CF_CODEC_GET_POSITION_ENC(&*(*ph).penc) as u16;

    if final_pos >= (*ph).pdu_header.header_encoded_length {
        /* the value that goes into the packet is length _after_ header */
        (*ph).pdu_header.data_encoded_length = final_pos - (*ph).pdu_header.header_encoded_length;
    }

    CF_CFDP_EncodeHeaderFinalSize((*ph).penc, &mut (*ph).pdu_header);
}

/// C: `const char *CF_CFDP_GetMoveTarget(...)`
pub unsafe fn CF_CFDP_GetMoveTarget(
    dest_dir: *const c_char,
    subject_file: *const c_char,
    dest_buf: *mut c_char,
    dest_size: usize,
) -> *const c_char {
    let mut result: *const c_char = ptr::null();

    if !dest_dir.is_null() && *dest_dir != 0 {
        let mut filename: *const c_char = libc_strrchr(subject_file, b'/' as i32);
        if filename.is_null() {
            filename = subject_file; /* not in a dir */
        } else {
            filename = filename.add(1);
        }

        let dest_path_len: i32 = libc_snprintf!(
            dest_buf,
            dest_size,
            b"%s/%s\0".as_ptr() as *const c_char,
            dest_dir,
            filename,
        );

        if (dest_path_len as usize) >= dest_size && dest_size > 2 {
            /* Mark character before zero terminator to indicate truncation */
            *dest_buf.add(dest_size - 2) = CF_FILENAME_TRUNCATED as c_char;

            /* Send event describing that the path would be truncated */
            CFE_EVS_SendEvent!(
                CF_EID_INF_CFDP_BUF_EXCEED,
                CFE_EVS_EventType_INFORMATION,
                b"CF: destination has been truncated to %s\0".as_ptr() as *const c_char,
                dest_buf,
            );
        }

        result = dest_buf;
    }

    result
}
