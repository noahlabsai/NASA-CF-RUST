use std::mem;
use std::ptr;

// Test helper functions
fn ut_cfdp_s_setup_basic_rx_state(pdu_buffer: &mut CF_Logical_PduBuffer_t) {
    static mut UT_DECODER: CF_DecoderState_t = CF_DecoderState_t {
        base: ptr::null_mut(),
        codec_state: CF_CodecState_t {
            is_valid: false,
            max_size: 0,
            next_offset: 0,
        },
    };
    static mut BYTES: [u8; CF_CFDP_MAX_HEADER_SIZE] = [0; CF_CFDP_MAX_HEADER_SIZE];

    unsafe {
        BYTES.fill(0);
        
        UT_DECODER.base = BYTES.as_mut_ptr();
        UT_DECODER.codec_state.is_valid = true;
        UT_DECODER.codec_state.max_size = BYTES.len();
        UT_DECODER.codec_state.next_offset = 0;
        
        pdu_buffer.pdec = &mut UT_DECODER;
    }
}

fn ut_cfdp_s_setup_basic_tx_state(pdu_buffer: &mut CF_Logical_PduBuffer_t) {
    static mut UT_ENCODER: CF_EncoderState_t = CF_EncoderState_t {
        base: ptr::null_mut(),
        codec_state: CF_CodecState_t {
            is_valid: false,
            max_size: 0,
            next_offset: 0,
        },
    };
    static mut BYTES: [u8; CF_MAX_PDU_SIZE] = [0; CF_MAX_PDU_SIZE];

    unsafe {
        BYTES.fill(0);
        
        UT_ENCODER.base = BYTES.as_mut_ptr();
        UT_ENCODER.codec_state.is_valid = true;
        UT_ENCODER.codec_state.max_size = BYTES.len();
        UT_ENCODER.codec_state.next_offset = 0;
        
        pdu_buffer.penc = &mut UT_ENCODER;
        
        UT_SetHandlerFunction(UT_KEY(CF_CFDP_ConstructPduHeader), UT_AltHandler_GenericPointerReturn, pdu_buffer as *mut _ as *mut std::ffi::c_void);
    }
}

fn ut_cfdp_s_setup_basic_test_state(
    setup: UT_CF_Setup_t,
    pdu_buffer_p: Option<&mut *mut CF_Logical_PduBuffer_t>,
    channel_p: Option<&mut *mut CF_Channel_t>,
    history_p: Option<&mut *mut CF_History_t>,
    txn_p: Option<&mut *mut CF_Transaction_t>,
    config_table_p: Option<&mut *mut CF_ConfigTable_t>,
) {
    static mut UT_PDU_BUFFER: CF_Logical_PduBuffer_t = CF_Logical_PduBuffer_t {
        pdec: ptr::null_mut(),
        penc: ptr::null_mut(),
        int_header: CF_Logical_PduHeader_t::default(),
    };
    static mut UT_HISTORY: CF_History_t = CF_History_t::default();
    static mut UT_TRANSACTION: CF_Transaction_t = CF_Transaction_t::default();
    static mut UT_CONFIG_TABLE: CF_ConfigTable_t = CF_ConfigTable_t::default();

    unsafe {
        UT_PDU_BUFFER = mem::zeroed();
        UT_HISTORY = mem::zeroed();
        UT_TRANSACTION = mem::zeroed();
        UT_CONFIG_TABLE = mem::zeroed();

        UT_TRANSACTION.history = &mut UT_HISTORY;
        UT_HISTORY.txn_stat = CF_TxnStatus_UNDEFINED;
        CF_AppData.config_table = &mut UT_CONFIG_TABLE;

        if let Some(pdu_buffer_ref) = pdu_buffer_p {
            if setup == UT_CF_Setup_TX || setup == UT_CF_Setup_RX {
                *pdu_buffer_ref = &mut UT_PDU_BUFFER;
            } else {
                *pdu_buffer_ref = ptr::null_mut();
            }
        }

        if let Some(channel_ref) = channel_p {
            *channel_ref = &mut CF_AppData.engine.channels[UT_CFDP_CHANNEL];
        }

        if let Some(history_ref) = history_p {
            *history_ref = &mut UT_HISTORY;
        }

        if let Some(txn_ref) = txn_p {
            *txn_ref = &mut UT_TRANSACTION;
        }

        if let Some(config_ref) = config_table_p {
            *config_ref = &mut UT_CONFIG_TABLE;
        }

        if setup == UT_CF_Setup_TX {
            ut_cfdp_s_setup_basic_tx_state(&mut UT_PDU_BUFFER);
        } else if setup == UT_CF_Setup_RX {
            ut_cfdp_s_setup_basic_rx_state(&mut UT_PDU_BUFFER);
        }

        UT_CF_ResetEventCapture();
        UT_SetHandlerFunction(UT_KEY(CF_CFDP_SetTxnStatus), UT_AltHandler_CaptureTransactionStatus, &mut UT_HISTORY.txn_stat as *mut _ as *mut std::ffi::c_void);
    }
}

fn cf_cfdp_s_tests_setup() {
    cf_tests_Setup();
    unsafe {
        CF_AppData = mem::zeroed();
    }
}

fn cf_cfdp_s_tests_teardown() {
    cf_tests_Teardown();
}

fn ut_alt_handler_cf_wrapped_open_create(user_obj: *mut std::ffi::c_void, func_key: UT_EntryKey_t, context: &UT_StubContext_t) {
    let fd = UT_Hook_GetArgValueByName(context, "fd", ptr::null_mut::<osal_id_t>());
    let mut status: i32 = 0;

    UT_Stub_GetInt32StatusCode(context, &mut status);

    if status == 0 {
        unsafe {
            *fd = OS_ObjectIdFromInteger(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_cfdp_s1_recv() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CFDP_S1_Recv(txn, ph);
    }

    #[test]
    fn test_cf_cfdp_s2_recv() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CFDP_S2_Recv(txn, ph);
    }

    #[test]
    fn test_cf_cfdp_s_ack_timer_tick() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        // no-op if not in R2
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.com.ack_timer_armed = true;
        }
        CF_CFDP_S_AckTimerTick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 0);

        // no-op if not armed
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = false;
        }
        CF_CFDP_S_AckTimerTick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 0);

        // in CF_TxnState_S2, ack_timer_armed
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = true;
        }
        CF_CFDP_S_AckTimerTick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 1);

        // in CF_TxnState_S2, ack_timer_armed + expiry
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = true;
        }
        CF_CFDP_S_AckTimerTick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 1);
    }

    #[test]
    fn test_cf_cfdp_s_tick() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        // nominal, not in CF_TxSubState_DATA_NORMAL
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.com.inactivity_fired = false;
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
        }
        CF_CFDP_S_Tick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 1);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CompleteTick), 1);

        // nominal, with timer expiry
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_ACTIVE);
        unsafe {
            (*txn).flags.com.inactivity_fired = false;
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, 1);
        CF_CFDP_S_Tick(txn);
        unsafe {
            assert!((*txn).flags.com.inactivity_fired);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_RecycleTransaction), 0);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CompleteTick), 1);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.inactivity_timer, 1);
        }

        // nominal, in CF_TxnState_HOLD, with timer expiry
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_TERMINATED);
        unsafe {
            (*txn).flags.com.inactivity_fired = false;
            (*txn).state = CF_TxnState_HOLD;
            (*txn).state_data.sub_state = CF_TxSubState_COMPLETE;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, 1);
        CF_CFDP_S_Tick(txn);
        unsafe {
            assert!((*txn).flags.com.inactivity_fired);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_RecycleTransaction), 1);

        // nominal, in CF_TxnState_S2, with timer expiry
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, 1);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_ACTIVE);
        unsafe {
            (*txn).flags.com.inactivity_fired = false;
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
        }
        CF_CFDP_S_Tick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 0);
        UT_CF_AssertEventID(CF_CFDP_S_INACT_TIMER_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.inactivity_timer, 2);
            assert!((*txn).flags.com.inactivity_fired);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_RecycleTransaction), 0);

        // nominal, active transaction
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_ACTIVE);
        CF_CFDP_S_Tick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_Timer_Tick), 0);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CompleteTick), 1);

        // inactive transaction
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state = CF_TxnState_HOLD;
            (*txn).flags.com.inactivity_fired = true;
        }
        CF_CFDP_S_Tick(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_RecycleTransaction), 1);
    }

    #[test]
    fn test_cf_cfdp_s_tick_maintenance() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        // nominal, nothing pending
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
        }
        CF_CFDP_S_Tick_Maintenance(txn);

        // If send_md is pending but failed to send
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.tx.send_md = true;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_SendMd), 1, -1);
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert!((*txn).flags.tx.send_md); // remains pending
        }

        // second time it does send, clears the flag
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert!(!(*txn).flags.tx.send_md);
        }

        // If send_eof is pending but failed to send
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.tx.send_eof = true;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_SendEof), 1, -1);
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert!((*txn).flags.tx.send_eof); // remains pending
        }

        // second time it does send, clears the flag, S1 does not arm timer
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert!(!(*txn).flags.tx.send_eof);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_ArmAckTimer), 0);

        // In S2 it does arm the ack timer
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.tx.send_eof = true;
            (*txn).reliable_mode = true;
        }
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert!(!(*txn).flags.tx.send_eof);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_ArmAckTimer), 1);

        // If fin_count is pending but failed to send
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 0;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_SendAck), 1, -1);
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert_eq!((*txn).flags.tx.fin_ack_count, 0); // remains pending
        }

        // second time it does send, clears the flag
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert_eq!((*txn).flags.tx.fin_ack_count, (*txn).flags.tx.fin_count);
        }

        // If fin_count is pending in S1 (ignored)
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 0;
        }
        CF_CFDP_S_Tick_Maintenance(txn);
        unsafe {
            assert_eq!((*txn).flags.tx.fin_ack_count, 0); // remains pending
        }
    }

    #[test]
    fn test_cf_cfdp_s_send_file_data() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut config: *mut CF_ConfigTable_t = ptr::null_mut();
        let mut cumulative_read: u32 = 0;
        let read_size: u32 = 100;
        let mut offset: u32 = 0;

        // failure of CF_CFDP_ConstructPduHeader
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, true), CF_SEND_PDU_NO_BUF_AVAIL_ERROR);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
        }

        // nominal, smaller than chunk, no CRC
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).outgoing_file_chunk_size = 150;
            (*txn).fsize = 300;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, read_size as i32);
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, false), read_size as i32);
        cumulative_read += read_size;
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
        }

        // nominal, larger than PDU, no CRC
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).outgoing_file_chunk_size = CF_MAX_PDU_SIZE * 2;
            (*txn).fsize = CF_MAX_PDU_SIZE * 2;
        }
        let read_size = CF_MAX_PDU_SIZE;
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, read_size as i32);
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size * 2, false), read_size as i32);
        cumulative_read += read_size;
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CRC_Digest), 0);

        // nominal, larger than chunk, with CRC
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).outgoing_file_chunk_size = 50;
            (*txn).fsize = 300;
        }
        let read_size = 100;
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, unsafe { (*config).outgoing_file_chunk_size as i32 });
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, true), unsafe { (*config).outgoing_file_chunk_size as i32 });
        cumulative_read += unsafe { (*config).outgoing_file_chunk_size };
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CRC_Digest), 1);

        // read w/failure
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, -1);
        unsafe {
            (*config).outgoing_file_chunk_size = read_size;
            (*txn).fsize = 300;
        }
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, true), -1);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_read, 1);
        }
        UT_CF_AssertEventID(CF_CFDP_S_READ_ERR_EID);

        // require lseek
        offset = 25;
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedLseek), 1, offset as i32);
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, read_size as i32);
        unsafe {
            (*config).outgoing_file_chunk_size = read_size;
            (*txn).fsize = 300;
        }
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, true), read_size as i32);
        cumulative_read += read_size;
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
        }

        // lseek w/failure
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedLseek), 1, -1);
        unsafe {
            (*config).outgoing_file_chunk_size = read_size;
            (*txn).fsize = 300;
        }
        assert_eq!(CF_CFDP_S_SendFileData(txn, offset, read_size, true), -1);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.file_data_bytes, cumulative_read);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek, 1);
        }
        UT_CF_AssertEventID(CF_CFDP_S_SEEK_FD_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_s_substate_send_file_data() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut config: *mut CF_ConfigTable_t = ptr::null_mut();

        // nominal, zero bytes processed
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        CF_CFDP_S_SubstateSendFileData(txn);

        // nominal, whole file at once
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).outgoing_file_chunk_size = CF_MAX_PDU_SIZE;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
            (*txn).fsize = CF_MAX_PDU_SIZE / 2;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, unsafe { (*txn).fsize as i32 });
        CF_CFDP_S_SubstateSendFileData(txn);
        unsafe {
            assert_eq!((*txn).foffs, (*txn).fsize);
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_UNDEFINED);
        }

        // nominal, less than whole file at once
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).outgoing_file_chunk_size = CF_MAX_PDU_SIZE / 2;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
            (*txn).fsize = CF_MAX_PDU_SIZE;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, unsafe { (*config).outgoing_file_chunk_size as i32 });
        CF_CFDP_S_SubstateSendFileData(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_NORMAL);
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_UNDEFINED);
        }

        // error during read
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*txn).foffs = 0;
            (*txn).fsize = CF_MAX_PDU_SIZE;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, -1);
        CF_CFDP_S_SubstateSendFileData(txn);
        unsafe {
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_read_FAILURE);
        }

        // No buffers available
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*txn).foffs = 0;
            (*txn).fsize = CF_MAX_PDU_SIZE;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, -1);
        CF_CFDP_S_SubstateSendFileData(txn);
        unsafe {
            assert_eq!((*txn).foffs, 0);
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_UNDEFINED);
        }
    }

    #[test]
    fn test_cf_cfdp_s_substate_early_fin() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        // nominal
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CFDP_S_SubstateEarlyFin(txn, ph);
    }

    #[test]
    fn test_cf_cfdp_s_substate_recv_fin() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_CheckAckNakCount), true);

        // nominal, first FIN recv
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.tx.fin_count = 0;
        }
        CF_CFDP_S_SubstateRecvFin(txn, ph);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CheckAckNakCount), 1);

        // call again, should reject as dupe but still ack
        unsafe {
            (*txn).flags.tx.fin_count = 1;
        }
        CF_CFDP_S_SubstateRecvFin(txn, ph);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CheckAckNakCount), 2);

        // call again, at ack/nak limit
        unsafe {
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_NO_ERROR);
        }
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_CheckAckNakCount), false);
        CF_CFDP_S_SubstateRecvFin(txn, ph);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_CheckAckNakCount), 3);
        unsafe {
            assert_eq!((*txn).history.txn_stat, CF_TxnStatus_POS_ACK_LIMIT_REACHED);
        }

        // fail to decode
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_RecvFin), 1, -1);
        CF_CFDP_S_SubstateRecvFin(txn, ph);
    }

    #[test]
    fn test_cf_cfdp_s2_substate_nak() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        // no segments
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_RecvNak), 1, CF_ERROR);
        CF_CFDP_S2_SubstateNak(txn, ph);
        UT_CF_AssertEventID(CF_CFDP_S_PDU_NAK_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error, 1);
        }

        // nominal, re-send md request (0,0)
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            let nak = &mut (*ph).int_header.nak;
            nak.segment_list.num_segments = 1;
            nak.segment_list.segments[0] = CF_Logical_SegmentRequest_t { offset_start: 0, offset_end: 0 };
        }
        CF_CFDP_S2_SubstateNak(txn, ph);
        unsafe {
            assert!((*txn).flags.tx.send_md);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.nak_segment_requests, 1);
        }

        // nominal, nonzero offsets
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            let nak = &mut (*ph).int_header.nak;
            nak.segment_list.num_segments = 2;
            nak.segment_list.segments[0] = CF_Logical_SegmentRequest_t { offset_start: 0, offset_end: 200 };
            nak.segment_list.segments[1] = CF_Logical_SegmentRequest_t { offset_start: 200, offset_end: 300 };
            (*txn).fsize = 300;
        }
        CF_CFDP_S2_SubstateNak(txn, ph);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.nak_segment_requests, 3);
        }

        // bad segments
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            let nak = &mut (*ph).int_header.nak;
            nak.segment_list.num_segments = 3;
            nak.segment_list.segments[0] = CF_Logical_SegmentRequest_t { offset_start: 200, offset_end: 100 };
            nak.segment_list.segments[1] = CF_Logical_SegmentRequest_t { offset_start: 100, offset_end: 400 };
            nak.segment_list.segments[2] = CF_Logical_SegmentRequest_t { offset_start: 400, offset_end: 0 };
            (*txn).fsize = 300;
        }
        CF_CFDP_S2_SubstateNak(txn, ph);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.nak_segment_requests, 6);
        }
        UT_CF_AssertEventID(CF_CFDP_S_INVALID_SR_ERR_EID);

        // bad decode
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_RecvNak), 1, -1);
        unsafe {
            let nak = &mut (*ph).int_header.nak;
            nak.segment_list.num_segments = 1;
        }
        CF_CFDP_S2_SubstateNak(txn, ph);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error, 2);
        }
        UT_CF_AssertEventID(CF_CFDP_S_PDU_NAK_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_s2_substate_eof_ack() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = ptr::null_mut();

        // nominal
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.tx.eof_ack_recv = false;
            (*ph).int_header.ack.ack_directive_code = CF_CFDP_FileDirective_EOF;
        }
        CF_CFDP_S2_SubstateEofAck(txn, ph);
        unsafe {
            assert!((*txn).flags.tx.eof_ack_recv);
            assert!(!(*txn).flags.com.ack_timer_armed);
        }

        // failure of CF_CFDP_RecvAck
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.ack.ack_directive_code = CF_CFDP_FileDirective_EOF;
        }
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_RecvAck), 1, -1);
        CF_CFDP_S2_SubstateEofAck(txn, ph);
        UT_CF_AssertEventID(CF_CFDP_S_PDU_EOF_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error, 1);
        }

        // Ack not for EOF
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.ack.ack_directive_code = CF_CFDP_FileDirective_FIN;
            (*txn).flags.tx.eof_ack_recv = false;
        }
        CF_CFDP_S2_SubstateEofAck(txn, ph);
        UT_CF_AssertEventID(CF_CFDP_S_PDU_EOF_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error, 2);
        }
    }

    #[test]
    fn test_cf_cfdp_s_init() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        UT_SetDefaultReturnValue(UT_KEY(OS_FileOpenCheck), OS_ERROR); // this is the "good" code
        UT_SetHandlerFunction(UT_KEY(CF_WrappedOpenCreate), ut_alt_handler_cf_wrapped_open_create, ptr::null_mut()); // set FD on output

        // Nominal case, everything success
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        CF_CFDP_S_Init(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_CRC_Start), 1);
        unsafe {
            assert!((*txn).flags.tx.send_md);
        }

        // From here on is error checks
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), false);

        // Error condition set before call
        // This is really a no-op
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        CF_CFDP_S_Init(txn);

        // First status check passes, open check fails
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, true);
        UT_SetDeferredRetcode(UT_KEY(OS_FileOpenCheck), 1, OS_SUCCESS); // this is the "bad" code
        CF_CFDP_S_Init(txn);
        UT_CF_AssertEventID(CF_CFDP_S_ALREADY_OPEN_ERR_EID);

        // Second status check passes, open succeeds - confirms close of FD
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 2, true);
        CF_CFDP_S_Init(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_WrappedClose), 1);
        unsafe {
            assert!(!OS_ObjectIdDefined((*txn).fd));
        }

        // Second status check passes, open create fails
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 2, true);
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedOpenCreate), 1, OS_ERROR);
        CF_CFDP_S_Init(txn);
        UT_CF_AssertEventID(CF_CFDP_S_OPEN_ERR_EID);

        // Third status check passes, seek fails
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 3, true);
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedLseek), 1, OS_ERROR);
        CF_CFDP_S_Init(txn);
        UT_CF_AssertEventID(CF_CFDP_S_SEEK_END_ERR_EID);

        // Forth status check passes, seek fails
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 4, true);
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedLseek), 1, OS_ERROR);
        CF_CFDP_S_Init(txn);
        UT_CF_AssertEventID(CF_CFDP_S_SEEK_BEG_ERR_EID);

        assert_eq!(UtAssert_STUB_COUNT(CF_CRC_Start), 1); // nothing but the first nominal case should have invoked this
    }

    #[test]
    fn test_cf_cfdp_s_handle_file_retention() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let faildir = "f";

        // Nominal, keep flag set - nothing done
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, true);
        unsafe {
            (*txn).keep = true;
            (*txn).flags.com.is_complete = true;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);

        // Failed transfer, nothing done
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, true);
        unsafe {
            (*txn).flags.com.is_complete = false;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);

        // Failed transfer, nothing done
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, false);
        unsafe {
            (*txn).flags.com.is_complete = true;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);

        // Successful transfer, keep flag unset, S1
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = true;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 1);
        UT_CF_AssertEventID(CF_CFDP_S_FILE_REMOVED_EID);

        // Successful transfer, keep flag unset, S2, FIN from peer says not retained
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = true;
            (*txn).state_data.fin_fs = CF_CFDP_FinFileStatus_UNREPORTED;
            (*txn).state_data.fin_dc = CF_CFDP_FinDeliveryCode_COMPLETE;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);

        // Successful transfer, keep flag unset, S2, FIN from peer says not complete
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = true;
            (*txn).state_data.fin_fs = CF_CFDP_FinFileStatus_RETAINED;
            (*txn).state_data.fin_dc = CF_CFDP_FinDeliveryCode_INCOMPLETE;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);

        // Successful transfer, keep flag unset, S2, FIN from peer says complete + retained
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = true;
            (*txn).state_data.fin_fs = CF_CFDP_FinFileStatus_RETAINED;
            (*txn).state_data.fin_dc = CF_CFDP_FinDeliveryCode_COMPLETE;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 1);
        UT_CF_AssertEventID(CF_CFDP_S_FILE_REMOVED_EID);

        // Unsuccessful transfer, keep flag unset, not commanded (e.g. polling dir), but no fail_dir set
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = false;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_GetMoveTarget), 1);

        // Unsuccessful transfer, keep flag unset, not commanded (e.g. polling dir), fail_dir set
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_CFDP_GetMoveTarget), UT_AltHandler_GenericPointerReturn, faildir.as_ptr() as *mut std::ffi::c_void);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = false;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 1);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_GetMoveTarget), 1);
        UT_CF_AssertEventID(CF_CFDP_S_FILE_MOVED_EID);

        // Unsuccessful transfer, keep flag unset, commanded
        UT_ResetState(0);
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).keep = false;
            (*txn).flags.com.is_complete = false;
            (*txn).flags.tx.cmd_tx = true;
        }
        CF_CFDP_S_HandleFileRetention(txn);
        assert_eq!(UtAssert_STUB_COUNT(OS_mv), 0);
        assert_eq!(UtAssert_STUB_COUNT(OS_remove), 0);
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_GetMoveTarget), 0);
    }

    #[test]
    fn test_cf_cfdp_s_check_state_normal() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        // incomplete file, nominal
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).foffs = 10;
            (*txn).fsize = 20;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_NORMAL);
        }

        // complete file
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).foffs = 20;
            (*txn).fsize = 20;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // incomplete file with error state
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, false);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).foffs = 0;
            (*txn).fsize = 20;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
        }

        // incomplete file with early FIN received
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.tx.fin_count = 1;
            (*txn).foffs = 0;
            (*txn).fsize = 20;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
        }
    }

    #[test]
    fn test_cf_cfdp_s_check_state_eof() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        // nominal, send_eof still pending
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.send_eof = true;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // An error is pending
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_TxnIsOK), 1, false);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!(!(*txn).flags.com.is_complete);
        }

        // nominal, S1, send_eof done, no closure request
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.send_eof = false;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!((*txn).flags.com.is_complete);
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinFileStatus_UNREPORTED);
        }

        // nominal, S1, send_eof done, closure request still pending
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.send_eof = false;
            (*txn).flags.com.close_req = true;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
            assert!(!(*txn).flags.com.is_complete);
        }

        // nominal, S1, send_eof done, closure request and FIN recv
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.send_eof = false;
            (*txn).flags.com.close_req = true;
            (*txn).flags.tx.fin_count = 1;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!((*txn).flags.com.is_complete);
        }

        // nominal, S2, send_eof done, waiting on ack, timer still active
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_CheckAckNakCount), 1, true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.com.ack_timer_armed = true;
            (*txn).flags.tx.send_eof = false;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // nominal, S2, send_eof done, waiting on ack, timer expired, under ack limit
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_CheckAckNakCount), 1, true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.tx.send_eof = false;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
            assert!((*txn).flags.tx.send_eof);
        }

        // nominal, S2, send_eof done, waiting on ack, timer expired, over ack limit
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_CheckAckNakCount), 1, false);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.tx.send_eof = false;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!(!(*txn).flags.tx.send_eof);
            assert!(!(*txn).flags.com.is_complete);
        }

        // nominal, S2, send_eof done, got eof-ack but no FIN, under ack limit
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.eof_ack_recv = true;
            (*txn).flags.tx.send_eof = false;
            (*txn).flags.tx.fin_count = 0;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // nominal, S2, send_eof done, got eof-ack and fin, but fin-ack not sent yet
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.eof_ack_recv = true;
            (*txn).flags.tx.send_eof = false;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 0;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // nominal, S2, send_eof done, got eof-ack and fin, and fin-ack sent
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.eof_ack_recv = true;
            (*txn).flags.tx.send_eof = false;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 1;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!((*txn).flags.com.is_complete);
        }

        // nominal, got fin but no eof-ack, timer still active
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.eof_ack_recv = false;
            (*txn).flags.com.ack_timer_armed = true;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 1;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_DATA_EOF);
        }

        // nominal, got fin but no eof-ack, timer expired
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_TxnIsOK), true);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_TxSubState_DATA_EOF;
            (*txn).flags.tx.eof_ack_recv = false;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.tx.fin_count = 1;
            (*txn).flags.tx.fin_ack_count = 1;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_FILESTORE);
            assert!(!(*txn).flags.com.is_complete);
        }
    }

    #[test]
    fn test_cf_cfdp_s_check_state_filestore() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.sub_state = CF_TxSubState_FILESTORE;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_COMPLETE);
        }
        assert_eq!(UtAssert_STUB_COUNT(CF_CFDP_FinishTransaction), 1);
    }

    #[test]
    fn test_cf_cfdp_s_check_state_complete() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();

        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.sub_state = CF_TxSubState_COMPLETE;
        }
        CF_CFDP_S_CheckState(txn);
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_TxSubState_COMPLETE);
        }
    }

    #[test]
    fn test_cf_cfdp_s_check_state() {
        UT_ResetState(0);
        test_cf_cfdp_s_check_state_normal();
        UT_ResetState(0);
        test_cf_cfdp_s_check_state_eof();
        UT_ResetState(0);
        test_cf_cfdp_s_check_state_filestore();
        UT_ResetState(0);
        test_cf_cfdp_s_check_state_complete();
    }

    #[test]
    fn test_cf_cfdp_s_tick_nak() {
        let mut txn: *mut CF_Transaction_t = ptr::null_mut();
        let mut config: *mut CF_ConfigTable_t = ptr::null_mut();
        let mut chunk = CF_Chunk_t {
            offset: 0,
            size: 100,
            next: ptr::null_mut(),
        };

        // Nominal, nothing to do
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.tx.fd_nak_pending = false;
        }
        CF_CFDP_S_Tick_Nak(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_ChunkList_GetFirstChunk), 0);

        // Nominal, chunk list empty
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.tx.fd_nak_pending = true;
        }
        CF_CFDP_S_Tick_Nak(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_ChunkList_GetFirstChunk), 1);
        unsafe {
            assert!(!(*txn).flags.tx.fd_nak_pending);
        }

        // chunk list not empty, SendFileData fails
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_ChunkList_GetFirstChunk), UT_AltHandler_GenericPointerReturn, &mut chunk as *mut _ as *mut std::ffi::c_void);
        unsafe {
            (*txn).flags.tx.fd_nak_pending = true;
        }
        CF_CFDP_S_Tick_Nak(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_ChunkList_RemoveFromFirst), 0);
        unsafe {
            assert!((*txn).flags.tx.fd_nak_pending);
        }

        // Nominal, chunk list not empty, SendFileData works
        ut_cfdp_s_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), Some(&mut config));
        UT_SetHandlerFunction(UT_KEY(CF_ChunkList_GetFirstChunk), UT_AltHandler_GenericPointerReturn, &mut chunk as *mut _ as *mut std::ffi::c_void);
        UT_SetDeferredRetcode(UT_KEY(CF_WrappedRead), 1, chunk.size as i32);
        unsafe {
            (*config).outgoing_file_chunk_size = chunk.size;
            (*txn).flags.tx.fd_nak_pending = true;
        }
        CF_CFDP_S_Tick_Nak(txn);
        assert_eq!(UtAssert_STUB_COUNT(CF_ChunkList_RemoveFromFirst), 1);
        unsafe {
            assert!((*txn).flags.tx.fd_nak_pending);
        }
    }
}

fn ut_test_setup() {
    // Test setup function calls would go here
}