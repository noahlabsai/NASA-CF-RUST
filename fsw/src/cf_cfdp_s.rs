//! CF CFDP Send-File (S) transaction handlers.
//!
//! Translated from: cf_cfdp_s.c / cf_cfdp_s.h

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
use crate::cf_msg::*;
use crate::cf_eventids::*;
use crate::cf_app::CF_AppData;
use crate::cf_app_types::*;
use crate::cf_cfdp::*;
use crate::cf_utils::*;
use crate::cf_clist::*;
use crate::cf_timer::*;
use crate::cf_crc::*;
use crate::cf_cfdp_dispatch::*;
use crate::cf_cfdp_dispatch_types::*;
use crate::cf_chunk::*;

/// C: `CFE_Status_t CF_CFDP_S_SendFileData(CF_Transaction_t *txn, uint32 foffs, uint32 bytes_to_read, uint8 calc_crc)`
pub unsafe fn CF_CFDP_S_SendFileData(
    txn: *mut CF_Transaction_t,
    foffs: u32,
    bytes_to_read: u32,
    calc_crc: u8,
) -> CFE_Status_t {
    let mut status: i32;
    let mut ret: CFE_Status_t;
    let ph = CF_CFDP_ConstructPduHeader(
        txn,
        CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_INVALID_MIN, /* 0 = file data */
        (*CF_AppData.config_table).local_eid,
        (*(*txn).history).peer_eid,
        false,
        (*(*txn).history).seq_num,
        false,
    );

    if ph.is_null() {
        ret = CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    } else {
        ret = CFE_SUCCESS;
        let fd = &mut (*ph).int_header.fd;

        fd.offset = foffs;

        /* encode the file data header (just the offset) */
        CF_CFDP_EncodeFileDataHeader((*ph).penc, (*ph).pdu_header.segment_meta_flag != 0, fd);

        /* read the requested amount from the file */
        let actual_bytes: i32;
        if (*txn).state_data.cached_pos != foffs {
            status = CF_WrappedLseek((*txn).fd, foffs as i32, OS_SEEK_SET as i32);
            if status < 0 {
                CFE_EVS_SendEvent!(
                    CF_CFDP_S_SEEK_FD_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF S%d(%lu:%lu): error seeking to offset %lu, got %ld\0".as_ptr() as *const c_char,
                    CF_CFDP_GetPrintClass(txn),
                    (*(*txn).history).src_eid as u64,
                    (*(*txn).history).seq_num as u64,
                    foffs as u64,
                    status as i64,
                );
                CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek += 1;
                ret = CF_ERROR;
            }
        }

        if ret == CFE_SUCCESS {
            status = CF_WrappedRead((*txn).fd, fd.data_ptr as *mut u8, bytes_to_read as usize);
            if status != bytes_to_read as i32 {
                CFE_EVS_SendEvent!(
                    CF_CFDP_S_READ_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF S%d(%lu:%lu): error reading bytes: expected %lu, got %ld\0".as_ptr() as *const c_char,
                    CF_CFDP_GetPrintClass(txn),
                    (*(*txn).history).src_eid as u64,
                    (*(*txn).history).seq_num as u64,
                    bytes_to_read as u64,
                    status as i64,
                );
                CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_read += 1;
                ret = CF_ERROR;
            } else {
                actual_bytes = status;
                fd.data_len = actual_bytes as usize;
                (*txn).state_data.cached_pos += actual_bytes as u32;
            }
        }

        if ret == CFE_SUCCESS {
            CF_CFDP_SendFd(txn, ph);

            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes += fd.data_len as u64;
            if calc_crc != 0 {
                CF_CRC_Digest(&mut (*txn).crc, std::slice::from_raw_parts(fd.data_ptr, fd.data_len));
            }

            /* return actual chunk size sent */
            ret = fd.data_len as CFE_Status_t;
        }
    }

    ret
}

/// C: `void CF_CFDP_S_SubstateSendFileData(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_SubstateSendFileData(txn: *mut CF_Transaction_t) {
    if (*txn).foffs < (*txn).fsize {
        let sret = CF_CFDP_S_SendFileData(txn, (*txn).foffs, (*txn).fsize - (*txn).foffs, 1);
        if sret > 0 {
            (*txn).foffs += sret as u32;
        } else if sret != CF_SEND_PDU_NO_BUF_AVAIL_ERROR {
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_READ_FAILURE);
        }
    }
}

/// C: `void CF_CFDP_S_SubstateEarlyFin(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S_SubstateEarlyFin(
    txn: *mut CF_Transaction_t,
    _ph: *mut CF_Logical_PduBuffer_t,
) {
    CFE_EVS_SendEvent!(
        CF_CFDP_S_EARLY_FIN_ERR_EID,
        CFE_EVS_EventType_ERROR,
        b"CF S%d(%lu:%lu): got early FIN -- cancelling\0".as_ptr() as *const c_char,
        CF_CFDP_GetPrintClass(txn),
        (*(*txn).history).src_eid as u64,
        (*(*txn).history).seq_num as u64,
    );
    CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_EARLY_FIN);
    /* otherwise do normal fin processing */
    CF_CFDP_S_SubstateRecvFin(txn, _ph);
}

/// C: `void CF_CFDP_S_SubstateRecvFin(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S_SubstateRecvFin(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    if CF_CFDP_RecvFin(txn, ph) == CFE_SUCCESS {
        let fin = &(*ph).int_header.fin;

        if (*txn).flags.tx.fin_count == 0 {
            (*txn).state_data.peer_cc = fin.cc as u8;
            (*txn).state_data.fin_dc = fin.delivery_code as u8;
            (*txn).state_data.fin_fs = fin.file_status as u8;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_From_ConditionCode(fin.cc));
        }

        if !CF_CFDP_CheckAckNakCount(txn, &mut (*txn).flags.tx.fin_count) {
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_POS_ACK_LIMIT_REACHED);
        }
    }
}

/// C: `void CF_CFDP_S2_SubstateNak(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S2_SubstateNak(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let mut bad_sr: u8 = 0;

    if CF_CFDP_RecvNak(txn, ph) == CFE_SUCCESS {
        let nak = &(*ph).int_header.nak;

        for counter in 0..nak.segment_list.num_segments {
            let sr = &nak.segment_list.segments[counter as usize];

            if sr.offset_start == 0 && sr.offset_end == 0 {
                /* need to re-send metadata PDU */
                (*txn).flags.tx.send_md = true;
            } else {
                if sr.offset_end < sr.offset_start {
                    bad_sr += 1;
                    continue;
                }
                if sr.offset_end > (*txn).fsize {
                    bad_sr += 1;
                    continue;
                }
                /* insert gap data in chunks */
                CF_ChunkListAdd(&mut (*(*txn).chunks).chunks, sr.offset_start, sr.offset_end - sr.offset_start);
                (*txn).flags.tx.fd_nak_pending = true;
            }
        }

        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.nak_segment_requests +=
            nak.segment_list.num_segments as u32;
        if bad_sr != 0 {
            CFE_EVS_SendEvent!(
                CF_CFDP_S_INVALID_SR_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF S%d(%lu:%lu): received %d invalid NAK segment requests\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                bad_sr as i32,
            );
        }
    } else {
        CFE_EVS_SendEvent!(
            CF_CFDP_S_PDU_NAK_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF S%d(%lu:%lu): received invalid NAK PDU\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
    }
}

/// C: `void CF_CFDP_S2_SubstateEofAck(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S2_SubstateEofAck(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let ret = CF_CFDP_RecvAck(txn, ph);
    if ret == CFE_SUCCESS {
        let ack = &(*ph).int_header.ack;
        if ack.ack_directive_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF as u8 {
            (*txn).flags.tx.eof_ack_recv = true;
        }
    }

    if ret != CFE_SUCCESS || !(*txn).flags.tx.eof_ack_recv {
        CFE_EVS_SendEvent!(
            CF_CFDP_S_PDU_EOF_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF S%d(%lu:%lu): received invalid EOF-ACK PDU\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
    }
}

/// C: `void CF_CFDP_S1_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S1_Recv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    use crate::cf_cfdp_dispatch::CF_CFDP_TxStateDispatch;

    static S1_NORMAL: CF_CFDP_FileDirectiveDispatchTable_t = {
        let mut t = CF_CFDP_FileDirectiveDispatchTable_t { fdirective: [None; CF_CFDP_FileDirective_INVALID_MAX] };
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN as usize] = Some(CF_CFDP_S_SubstateEarlyFin as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t
    };
    static S1_EOF: CF_CFDP_FileDirectiveDispatchTable_t = {
        let mut t = CF_CFDP_FileDirectiveDispatchTable_t { fdirective: [None; CF_CFDP_FileDirective_INVALID_MAX] };
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN as usize] = Some(CF_CFDP_S_SubstateRecvFin as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t
    };
    static S1_SUBSTATE: SyncPtr<[*const CF_CFDP_FileDirectiveDispatchTable_t; CF_TxSubState_NUM_STATES]> = SyncPtr({
        let mut a = [ptr::null(); CF_TxSubState_NUM_STATES];
        a[CF_TxSubState_t::CF_TxSubState_DATA_NORMAL as usize] = &S1_NORMAL;
        a[CF_TxSubState_t::CF_TxSubState_DATA_EOF as usize] = &S1_EOF;
        a
    });
    /* ph is used implicitly via the dispatch table handlers */
    let _ = ph;
    CF_CFDP_TxStateDispatch(txn, &S1_SUBSTATE.0 as *const _ as *const CF_CFDP_TxnSendDispatchTable_t);
}

/// C: `void CF_CFDP_S2_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_S2_Recv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    use crate::cf_cfdp_dispatch::CF_CFDP_TxStateDispatch;

    static S2_NORMAL: CF_CFDP_FileDirectiveDispatchTable_t = {
        let mut t = CF_CFDP_FileDirectiveDispatchTable_t { fdirective: [None; CF_CFDP_FileDirective_INVALID_MAX] };
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN as usize] = Some(CF_CFDP_S_SubstateEarlyFin as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t
    };
    static S2_EOF: CF_CFDP_FileDirectiveDispatchTable_t = {
        let mut t = CF_CFDP_FileDirectiveDispatchTable_t { fdirective: [None; CF_CFDP_FileDirective_INVALID_MAX] };
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_ACK as usize] = Some(CF_CFDP_S2_SubstateEofAck as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_NAK as usize] = Some(CF_CFDP_S2_SubstateNak as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t.fdirective[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN as usize] = Some(CF_CFDP_S_SubstateRecvFin as unsafe fn(*mut CF_Transaction_t, *mut CF_Logical_PduBuffer_t));
        t
    };
    static S2_SUBSTATE: SyncPtr<[*const CF_CFDP_FileDirectiveDispatchTable_t; CF_TxSubState_NUM_STATES]> = SyncPtr({
        let mut a = [ptr::null(); CF_TxSubState_NUM_STATES];
        a[CF_TxSubState_t::CF_TxSubState_DATA_NORMAL as usize] = &S2_NORMAL;
        a[CF_TxSubState_t::CF_TxSubState_DATA_EOF as usize] = &S2_EOF;
        a
    });
    /* ph is used implicitly via the dispatch table handlers */
    let _ = ph;
    CF_CFDP_TxStateDispatch(txn, &S2_SUBSTATE.0 as *const _ as *const CF_CFDP_TxnSendDispatchTable_t);
}

/// C: `void CF_CFDP_S_Init(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_Init(txn: *mut CF_Transaction_t) {
    let mut PendingFd: osal_id_t = OS_OBJECT_ID_UNDEFINED;
    let mut OsStatus: i32;

    if (*(*txn).history).fnames.dst_filename[0] == 0 {
        /* no dest, use src */
        ptr::copy_nonoverlapping(
            (*(*txn).history).fnames.src_filename.as_ptr(),
            (*(*txn).history).fnames.dst_filename.as_mut_ptr(),
            CF_FILENAME_MAX_LEN,
        );
    }

    if (*(*txn).history).fnames.src_filename[0] == 0 {
        CFE_EVS_SendEvent!(
            CF_CFDP_S_NO_SRC_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF S%d(%lu:%lu): no source filename in transaction\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_open += 1;
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
    }

    if CF_CFDP_TxnIsOK(txn) {
        OsStatus = CF_WrappedOpenCreate(
            &mut PendingFd,
            (*(*txn).history).fnames.src_filename.as_ptr(),
            OS_FILE_FLAG_NONE,
            OS_READ_ONLY,
        );
        if OsStatus < 0 {
            CFE_EVS_SendEvent!(
                CF_CFDP_S_OPEN_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF S%d(%lu:%lu): failed to open file %s, error=%ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                (*(*txn).history).fnames.src_filename.as_ptr(),
                OsStatus as i64,
            );
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_open += 1;
            PendingFd = OS_OBJECT_ID_UNDEFINED;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
        }
    }

    if CF_CFDP_TxnIsOK(txn) {
        OsStatus = CF_WrappedLseek(PendingFd, 0i32, OS_SEEK_END as i32);
        if OsStatus < 0 {
            CFE_EVS_SendEvent!(
                CF_CFDP_S_SEEK_END_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF S%d(%lu:%lu): failed to seek end file %s, error=%ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                (*(*txn).history).fnames.src_filename.as_ptr(),
                OsStatus as i64,
            );
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek += 1;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
        } else {
            (*txn).fsize = OsStatus as u32;
        }
    }

    if CF_CFDP_TxnIsOK(txn) {
        OsStatus = CF_WrappedLseek(PendingFd, 0i32, OS_SEEK_SET as i32);
        if OsStatus < 0 {
            CFE_EVS_SendEvent!(
                CF_CFDP_S_SEEK_BEG_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF S%d(%lu:%lu): failed to seek begin file %s, got %ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                (*(*txn).history).fnames.src_filename.as_ptr(),
                OsStatus as i64,
            );
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek += 1;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
        }
    }

    if CF_CFDP_TxnIsOK(txn) {
        (*txn).fd = PendingFd;
        CF_CRC_Start(&mut (*txn).crc);
        (*txn).flags.tx.send_md = true;
    } else if OS_ObjectIdDefined(PendingFd) {
        CF_WrappedClose(PendingFd);
    }
}

/// C: `void CF_CFDP_S_HandleFileRetention(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_HandleFileRetention(txn: *mut CF_Transaction_t) {
    let mut temp_name = [0u8; CF_FILENAME_MAX_LEN];
    let subject_file: *const c_char = (*(*txn).history).fnames.src_filename.as_ptr() as *const c_char;
    let mut move_dest: *const c_char = ptr::null();
    let config = &(*CF_AppData.config_table).chan[(*txn).chan_num as usize];
    let mut allow_local_remove = false;

    if !CF_CFDP_TxnIsOK(txn) || !(*txn).flags.com.is_complete {
        if !(*txn).flags.tx.cmd_tx {
            move_dest = CF_CFDP_GetMoveTarget(
                (*CF_AppData.config_table).fail_dir.as_ptr() as *const c_char,
                subject_file, temp_name.as_mut_ptr() as *mut c_char, temp_name.len(),
            );
        }
    } else if (*txn).keep == 0 {
        if !(*txn).reliable_mode {
            allow_local_remove = true;
        } else {
            allow_local_remove = (*txn).state_data.fin_fs == CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_RETAINED as u8
                && (*txn).state_data.fin_dc == CF_CFDP_FinDeliveryCode_t::CF_CFDP_FinDeliveryCode_COMPLETE as u8;
        }
        if allow_local_remove {
            move_dest = CF_CFDP_GetMoveTarget(
                config.move_dir.as_ptr() as *const c_char,
                subject_file, temp_name.as_mut_ptr() as *mut c_char, temp_name.len(),
            );
        }
    }

    if !move_dest.is_null() {
        let os_status = OS_mv(subject_file, move_dest);
        CFE_EVS_SendEvent!(CF_CFDP_S_FILE_MOVED_EID, CFE_EVS_EventType_INFORMATION,
            b"CF S%d(%lu:%lu): moved %s -> %s, status=%d\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn), (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64, subject_file, move_dest, os_status);
    } else if allow_local_remove {
        let os_status = OS_remove(subject_file);
        CFE_EVS_SendEvent!(CF_CFDP_S_FILE_REMOVED_EID, CFE_EVS_EventType_INFORMATION,
            b"CF S%d(%lu:%lu): removed source file %s, status=%d\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn), (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64, subject_file, os_status);
    }
}

/// C: `void CF_CFDP_S_AckTimerTick(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_AckTimerTick(txn: *mut CF_Transaction_t) {
    if !(*txn).reliable_mode || !(*txn).flags.com.ack_timer_armed { return; }
    if !CF_Timer_Expired(&(*txn).ack_timer) {
        CF_Timer_Tick(&mut (*txn).ack_timer);
    } else {
        (*txn).flags.com.ack_timer_armed = false;
    }
}

/// C: `CF_TxSubState_t CF_CFDP_S_CheckState_DATA_NORMAL(CF_Transaction_t *txn)`
unsafe fn CF_CFDP_S_CheckState_DATA_NORMAL(txn: *mut CF_Transaction_t) -> CF_TxSubState_t {
    if (*txn).foffs >= (*txn).fsize {
        CF_TxSubState_t::CF_TxSubState_DATA_EOF
    } else if !CF_CFDP_TxnIsOK(txn) || (*txn).flags.tx.fin_count != 0 {
        CF_TxSubState_t::CF_TxSubState_FILESTORE
    } else {
        core::mem::transmute((*txn).state_data.sub_state)
    }
}

/// C: `CF_TxSubState_t CF_CFDP_S1_CheckState_DATA_EOF(CF_Transaction_t *txn)`
unsafe fn CF_CFDP_S1_CheckState_DATA_EOF(txn: *mut CF_Transaction_t) -> CF_TxSubState_t {
    let mut next_state: CF_TxSubState_t = core::mem::transmute((*txn).state_data.sub_state);
    if !(*txn).flags.com.close_req {
        (*txn).state_data.fin_dc = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_UNREPORTED as u8;
        (*txn).flags.com.is_complete = true;
        next_state = CF_TxSubState_t::CF_TxSubState_FILESTORE;
    } else if (*txn).flags.tx.fin_count != 0 {
        (*txn).flags.com.is_complete = true;
        next_state = CF_TxSubState_t::CF_TxSubState_FILESTORE;
    }
    next_state
}

/// C: `CF_TxSubState_t CF_CFDP_S2_CheckState_DATA_EOF(CF_Transaction_t *txn)`
unsafe fn CF_CFDP_S2_CheckState_DATA_EOF(txn: *mut CF_Transaction_t) -> CF_TxSubState_t {
    let mut next_state: CF_TxSubState_t = core::mem::transmute((*txn).state_data.sub_state);
    if (*txn).flags.tx.fin_count != (*txn).flags.tx.fin_ack_count {
        /* need to send FIN-ACK */
    } else if (*txn).flags.tx.eof_ack_recv {
        if (*txn).flags.tx.fin_count != 0 {
            (*txn).flags.com.is_complete = true;
            next_state = CF_TxSubState_t::CF_TxSubState_FILESTORE;
        }
    } else if !(*txn).flags.com.ack_timer_armed {
        if (*txn).flags.tx.fin_count != 0 {
            next_state = CF_TxSubState_t::CF_TxSubState_FILESTORE;
        } else if CF_CFDP_CheckAckNakCount(txn, &mut (*txn).state_data.acknak_count) {
            (*txn).flags.tx.send_eof = true;
        } else {
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_NAK_LIMIT_REACHED);
            next_state = CF_TxSubState_t::CF_TxSubState_FILESTORE;
        }
    }
    next_state
}

/// C: `CF_TxSubState_t CF_CFDP_S_CheckState_DATA_EOF(CF_Transaction_t *txn)`
unsafe fn CF_CFDP_S_CheckState_DATA_EOF(txn: *mut CF_Transaction_t) -> CF_TxSubState_t {
    if !CF_CFDP_TxnIsOK(txn) {
        CF_TxSubState_t::CF_TxSubState_FILESTORE
    } else if (*txn).flags.tx.send_eof {
        core::mem::transmute((*txn).state_data.sub_state) /* do nothing */
    } else if !(*txn).reliable_mode {
        CF_CFDP_S1_CheckState_DATA_EOF(txn)
    } else {
        CF_CFDP_S2_CheckState_DATA_EOF(txn)
    }
}

/// C: `CF_TxSubState_t CF_CFDP_S_CheckState_FILESTORE(CF_Transaction_t *txn)`
unsafe fn CF_CFDP_S_CheckState_FILESTORE(txn: *mut CF_Transaction_t) -> CF_TxSubState_t {
    CF_CFDP_S_HandleFileRetention(txn);
    CF_TxSubState_t::CF_TxSubState_COMPLETE
}

/// C: `void CF_CFDP_S_CheckState(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_CheckState(txn: *mut CF_Transaction_t) {
    let sub_state_enum: CF_TxSubState_t = core::mem::transmute((*txn).state_data.sub_state);
    let next_state = match sub_state_enum {
        CF_TxSubState_t::CF_TxSubState_DATA_NORMAL => CF_CFDP_S_CheckState_DATA_NORMAL(txn),
        CF_TxSubState_t::CF_TxSubState_DATA_EOF    => CF_CFDP_S_CheckState_DATA_EOF(txn),
        CF_TxSubState_t::CF_TxSubState_FILESTORE   => CF_CFDP_S_CheckState_FILESTORE(txn),
        _ => CF_TxSubState_t::CF_TxSubState_COMPLETE,
    };
    if next_state != sub_state_enum {
        (*txn).state_data.sub_state = next_state as u8;
        (*txn).flags.com.ack_timer_armed = false;
        match next_state {
            CF_TxSubState_t::CF_TxSubState_DATA_EOF => {
                (*txn).state_data.acknak_count = 0;
                CF_CRC_Finalize(&mut (*txn).crc);
                (*txn).flags.com.crc_complete = true;
                (*txn).flags.tx.send_eof = true;
            }
            CF_TxSubState_t::CF_TxSubState_COMPLETE => {
                CF_CFDP_FinishTransaction(txn, true);
            }
            _ => {}
        }
    }
}

/// C: `void CF_CFDP_S_Tick(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_Tick(txn: *mut CF_Transaction_t) {
    let curr_status = CF_CFDP_GetAckTxnStatus(txn);

    if !(*txn).flags.com.inactivity_fired
        && (*txn).state_data.sub_state != CF_TxSubState_t::CF_TxSubState_DATA_NORMAL as u8
    {
        if !CF_Timer_Expired(&(*txn).inactivity_timer) {
            CF_Timer_Tick(&mut (*txn).inactivity_timer);
        } else {
            (*txn).flags.com.inactivity_fired = true;
            if curr_status == CF_CFDP_AckTxnStatus_t::CF_CFDP_AckTxnStatus_ACTIVE {
                CFE_EVS_SendEvent!(
                    CF_CFDP_S_INACT_TIMER_ERR_EID, CFE_EVS_EventType_ERROR,
                    b"CF S(%lu:%lu): inactivity timer expired\0".as_ptr() as *const c_char,
                    (*(*txn).history).src_eid as u64, (*(*txn).history).seq_num as u64,
                );
                CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.inactivity_timer += 1;
                CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_INACTIVITY_DETECTED);
            }
        }
    }

    if curr_status == CF_CFDP_AckTxnStatus_t::CF_CFDP_AckTxnStatus_ACTIVE {
        CF_CFDP_S_AckTimerTick(txn);
        CF_CFDP_S_CheckState(txn);
    }

    CF_CFDP_S_Tick_Maintenance(txn);

    if (*txn).flags.com.inactivity_fired && (*txn).state == CF_TxnState_t::CF_TxnState_HOLD {
        CF_CFDP_RecycleTransaction(txn);
    } else {
        CF_CFDP_CompleteTick(txn);
    }
}

/// C: `void CF_CFDP_S_Tick_Maintenance(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_Tick_Maintenance(txn: *mut CF_Transaction_t) {
    if (*txn).flags.tx.send_md {
        let sret = CF_CFDP_SendMd(txn);
        if sret == CFE_SUCCESS {
            (*txn).flags.tx.send_md = false;
        }
    } else if (*txn).flags.tx.send_eof {
        let sret = CF_CFDP_SendEof(txn);
        if sret == CFE_SUCCESS {
            (*txn).flags.tx.send_eof = false;
            if (*txn).reliable_mode {
                CF_CFDP_ArmAckTimer(txn);
            }
        }
    } else if (*txn).reliable_mode && (*txn).flags.tx.fin_ack_count != (*txn).flags.tx.fin_count {
        let sret = CF_CFDP_SendAck(txn, CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN);
        if sret == CFE_SUCCESS {
            (*txn).flags.tx.fin_ack_count = (*txn).flags.tx.fin_count;
        }
    }
}

/// C: `void CF_CFDP_S_Tick_Nak(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_S_Tick_Nak(txn: *mut CF_Transaction_t) {
    if (*txn).flags.tx.fd_nak_pending {
        let chunk = CF_ChunkList_GetFirstChunk(&(*(*txn).chunks).chunks);
        if chunk.is_none() {
            (*txn).flags.tx.fd_nak_pending = false;
        } else {
            let c = chunk.unwrap();
            let sret = CF_CFDP_S_SendFileData(txn, c.offset, c.size, 0);
            if sret > 0 {
                CF_ChunkList_RemoveFromFirst(&mut (*(*txn).chunks).chunks, sret as u32);
            }
        }
    }
}
