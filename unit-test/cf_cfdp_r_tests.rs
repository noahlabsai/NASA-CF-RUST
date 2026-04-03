use std::mem;

#[cfg(test)]
mod tests {
    use super::*;

    static mut UT_PDU_BUFFER: CF_Logical_PduBuffer_t = CF_Logical_PduBuffer_t {
        int_header: CF_Logical_PduHeader_t {
            fd: CF_Logical_PduFileDataHeader_t {
                data_len: 0,
                offset: 0,
            },
            eof: CF_Logical_PduEofHeader_t {
                size: 0,
            },
            ack: CF_Logical_PduAckHeader_t {
                ack_directive_code: 0,
            },
        },
    };
    static mut UT_HISTORY: CF_History_t = CF_History_t {
        txn_stat: CF_TxnStatus_UNDEFINED,
    };
    static mut UT_TRANSACTION: CF_Transaction_t = CF_Transaction_t {
        history: std::ptr::null_mut(),
        reliable_mode: false,
        state: CF_TxnState_IDLE,
        state_data: CF_TxnState_Data_t {
            sub_state: CF_RxSubState_IDLE,
            cached_pos: 0,
            eof_size: 0,
            eof_crc: 0,
            fin_dc: 0,
            fin_fs: 0,
            acknak_count: 0,
        },
        flags: CF_TxnFlags_t {
            com: CF_TxnCommonFlags_t {
                ack_timer_armed: false,
                inactivity_fired: false,
                close_req: false,
                crc_complete: false,
                is_complete: false,
            },
            rx: CF_TxnRxFlags_t {
                md_recv: false,
                eof_count: 0,
                eof_ack_count: 0,
                send_nak: false,
                send_fin: false,
                finack_recv: false,
                tempfile_created: false,
            },
        },
        fsize: 0,
        crc: CF_Crc_t {
            result: 0,
        },
        fd: OS_OBJECT_ID_UNDEFINED,
        chan_num: 0,
        chunks: std::ptr::null_mut(),
    };
    static mut UT_CONFIG_TABLE: CF_ConfigTable_t = CF_ConfigTable_t {
        rx_crc_calc_bytes_per_wakeup: 0,
    };

    fn ut_cfdp_r_setup_basic_rx_state(_pdu_buffer: &mut CF_Logical_PduBuffer_t) {
        // placeholder, nothing for now in this module
    }

    fn ut_cfdp_r_setup_basic_tx_state(pdu_buffer: &mut CF_Logical_PduBuffer_t) {
        // Make it so a call to CF_CFDP_ConstructPduBuffer returns the same PDU buffer
        ut_set_handler_function(UT_KEY_CF_CFDP_CONSTRUCT_PDU_HEADER, ut_alt_handler_generic_pointer_return, pdu_buffer as *mut _ as *mut std::ffi::c_void);
    }

    fn ut_cfdp_r_setup_basic_test_state(
        setup: UT_CF_Setup_t,
        pdu_buffer_p: Option<&mut *mut CF_Logical_PduBuffer_t>,
        channel_p: Option<&mut *mut CF_Channel_t>,
        history_p: Option<&mut *mut CF_History_t>,
        txn_p: Option<&mut *mut CF_Transaction_t>,
        config_table_p: Option<&mut *mut CF_ConfigTable_t>,
    ) {
        unsafe {
            // always clear all objects, regardless of what was asked for.
            // this helps ensure that a test does not depend on preexisting data
            // in the buffer (each test should set up its buffers in full)
            UT_PDU_BUFFER = mem::zeroed();
            UT_HISTORY = mem::zeroed();
            UT_TRANSACTION = mem::zeroed();
            UT_CONFIG_TABLE = mem::zeroed();

            // certain pointers should be connected even if they were not asked for,
            // as internal code may assume these are set (test cases may un-set)
            UT_TRANSACTION.history = &mut UT_HISTORY;
            CF_AppData.config_table = &mut UT_CONFIG_TABLE;

            if let Some(pdu_buffer_p) = pdu_buffer_p {
                if setup == UT_CF_Setup_TX || setup == UT_CF_Setup_RX {
                    *pdu_buffer_p = &mut UT_PDU_BUFFER;
                } else {
                    *pdu_buffer_p = std::ptr::null_mut();
                }
            }

            if let Some(channel_p) = channel_p {
                // note that for channels, many CF app functions assume
                // that when channel is passed as a pointer, that it is a member
                // of the array within CF_AppData, and the channel number can
                // be obtained by pointer arithmetic.
                // this arithmetic will break if the pointer is not actually
                // a member of that array, so for now it must be so.
                // This always uses the same channel for now.
                *channel_p = &mut CF_AppData.engine.channels[UT_CFDP_CHANNEL];
            }

            if let Some(history_p) = history_p {
                *history_p = &mut UT_HISTORY;
            }

            if let Some(txn_p) = txn_p {
                *txn_p = &mut UT_TRANSACTION;
            }

            if let Some(config_table_p) = config_table_p {
                *config_table_p = &mut UT_CONFIG_TABLE;
            }

            if setup == UT_CF_Setup_TX {
                ut_cfdp_r_setup_basic_tx_state(&mut UT_PDU_BUFFER);
            } else if setup == UT_CF_Setup_RX {
                ut_cfdp_r_setup_basic_rx_state(&mut UT_PDU_BUFFER);
            }

            // reset the event ID capture between each sub-case
            ut_cf_reset_event_capture();

            // Capture calls to CF_CFDP_SetTxnState() to capture transaction status
            ut_set_handler_function(UT_KEY_CF_CFDP_SET_TXN_STATUS, ut_alt_handler_capture_transaction_status, &mut UT_HISTORY.txn_stat as *mut _ as *mut std::ffi::c_void);
        }
    }

    fn cf_cfdp_r_tests_setup() {
        cf_tests_setup();

        // make sure global data is wiped between tests
        unsafe {
            CF_AppData = mem::zeroed();
        }
    }

    fn cf_cfdp_r_tests_teardown() {
        cf_tests_teardown();
    }

    #[test]
    fn test_cf_cfdp_r1_recv() {
        // Test case for:
        // void CF_CFDP_R1_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);

        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);

        unsafe {
            CF_CFDP_R1_Recv(txn, ph);
        }
    }

    #[test]
    fn test_cf_cfdp_r2_recv() {
        // Test case for:
        // void CF_CFDP_R2_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        // nominal, ack timer not armed
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            CF_CFDP_R2_Recv(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_ARM_ACK_TIMER), 0);

        // When ack timer is armed, this should reset it
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.com.ack_timer_armed = true;
            CF_CFDP_R2_Recv(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_ARM_ACK_TIMER), 1);
    }

    #[test]
    fn test_cf_cfdp_r_ack_timer_tick() {
        // Test case for:
        // void CF_CFDP_R_AckTimerTick(CF_Transaction_t *txn);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // no-op if not in R2
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.com.ack_timer_armed = true;
            CF_CFDP_R_AckTimerTick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 0);

        // no-op if not armed
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = false;
            CF_CFDP_R_AckTimerTick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 0);

        // in R2 state, ack_timer_armed set but not expired
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = true;
            CF_CFDP_R_AckTimerTick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 1);

        // in R2 state, ack_timer_armed set, timer expires
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = true;
        }
        ut_set_deferred_retcode(UT_KEY_CF_TIMER_EXPIRED, 1, 1);
        unsafe {
            CF_CFDP_R_AckTimerTick(txn);
        }
        unsafe {
            assert!(!(*txn).flags.com.ack_timer_armed);
        }
    }

    #[test]
    fn test_cf_cfdp_r_tick() {
        // Test case for:
        // void CF_CFDP_R_Tick(CF_Transaction_t *txn, int *cont);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // nominal, holdover - just ticks
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state = CF_TxnState_HOLD;
            CF_CFDP_R_Tick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 1);

        // nominal, active transaction
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_GET_ACK_TXN_STATUS, 1, CF_CFDP_AckTxnStatus_ACTIVE as i32);
        unsafe {
            CF_CFDP_R_Tick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 1);
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_COMPLETE_TICK), 1);

        // not in R2 state, timer expired
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state = CF_TxnState_HOLD;
        }
        ut_set_deferred_retcode(UT_KEY_CF_TIMER_EXPIRED, 1, 1);
        unsafe {
            CF_CFDP_R_Tick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_RECYCLE_TRANSACTION), 1);

        // nominal, in R2 state
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
            CF_CFDP_R_Tick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 1);

        // in HOLD state, timer expired now
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_TIMER_EXPIRED, 1, 1);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_GET_ACK_TXN_STATUS, 1, CF_CFDP_AckTxnStatus_TERMINATED as i32);
        unsafe {
            (*txn).state = CF_TxnState_HOLD;
            CF_CFDP_R_Tick(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 0);

        // in R2 state, timer expired now
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_GET_ACK_TXN_STATUS, 1, CF_CFDP_AckTxnStatus_ACTIVE as i32);
        unsafe {
            (*txn).reliable_mode = true;
        }
        ut_set_deferred_retcode(UT_KEY_CF_TIMER_EXPIRED, 1, 1);
        unsafe {
            CF_CFDP_R_Tick(txn);
        }
        unsafe {
            assert!((*txn).flags.com.inactivity_fired);
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_INACTIVITY_DETECTED);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_TIMER_TICK), 0);

        // timer already expired
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.inactivity_fired = true;
        }
        ut_set_deferred_retcode(UT_KEY_CF_TIMER_EXPIRED, 1, 1);
        unsafe {
            CF_CFDP_R_Tick(txn);
        }
        unsafe {
            assert!((*txn).flags.com.inactivity_fired);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_COMPLETE_TICK), 1);
    }

    #[test]
    fn test_cf_cfdp_r_tick_maintenance() {
        // Test case for:
        // void CF_CFDP_R_Tick_Maintenance(CF_Transaction_t *txn);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // in R1 state, nominal (does nothing, called for coverage)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            CF_CFDP_R_Tick_Maintenance(txn);
        }

        // in R1 state, send_fin set
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.rx.send_fin = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_SEND_FIN), 1);

        // in R2 state, send_ack set
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.eof_count = 1;
            (*txn).flags.rx.eof_ack_count = 0;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_SEND_ACK), 1);
        unsafe {
            assert_eq!((*txn).flags.rx.eof_ack_count, (*txn).flags.rx.eof_count);
        }

        // same as above, but SendAck fails
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_SEND_ACK, 1, CF_SEND_PDU_NO_BUF_AVAIL_ERROR as i32);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.eof_count = 1;
            (*txn).flags.rx.eof_ack_count = 0;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        unsafe {
            assert!((*txn).flags.rx.eof_ack_count < (*txn).flags.rx.eof_count);
        }

        // in R2 state, send_nak set
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.send_nak = true;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        unsafe {
            assert!(!(*txn).flags.rx.send_nak);
        }

        // same as above, but CF_CFDP_R_SendNak fails
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_SEND_NAK, 1, CF_SEND_PDU_NO_BUF_AVAIL_ERROR as i32);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.send_nak = true;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        unsafe {
            assert!((*txn).flags.rx.send_nak);
        }

        // in R2 state, send_fin set
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.send_fin = true;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        unsafe {
            assert!(!(*txn).flags.rx.send_fin);
        }

        // same as above, but CF_CFDP_R2_SubstateSendFin fails
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_SEND_FIN, 1, CF_SEND_PDU_NO_BUF_AVAIL_ERROR as i32);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.send_fin = true;
            (*txn).flags.com.inactivity_fired = true;
            CF_CFDP_R_Tick_Maintenance(txn);
        }
        unsafe {
            assert!((*txn).flags.rx.send_fin);
        }
    }

    #[test]
    fn test_cf_cfdp_r_init() {
        // Test case for:
        // void CF_CFDP_R_Init(CF_Transaction_t *txn);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // nominal
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.fin_dc = -1i8 as u8;
            (*txn).state_data.fin_fs = -1i8 as u8;
            CF_CFDP_R_Init(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_INVALID);
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_INVALID);
        }

        // nominal, R2 state, creates tempfile
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            CF_CFDP_R_Init(txn);
        }
        ut_cf_assert_event_id(CF_CFDP_R_TEMP_FILE_INF_EID);

        // failure of file open, class 1
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_OPEN_CREATE, 1, -1);
        unsafe {
            (*txn).reliable_mode = false;
            CF_CFDP_R_Init(txn);
        }
        ut_cf_assert_event_id(CF_CFDP_R_CREAT_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.fault.file_open, 1);
        }

        // failure of file open, class 2
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_OPEN_CREATE, 1, -1);
        unsafe {
            (*txn).reliable_mode = true;
            CF_CFDP_R_Init(txn);
        }
        ut_cf_assert_event_id(CF_CFDP_R_CREAT_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.fault.file_open, 2);
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_FILESTORE_REJECTION);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_crc() {
        // Test case for:
        // int CF_CFDP_R_CheckCrc(CF_Transaction_t *txn, uint32 expected_crc);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // CRC mismatch, class 1
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).crc.result = 0xdeadbeef;
            (*txn).state_data.eof_crc = 0x1badc0de;
        }
        assert_eq!(unsafe { CF_CFDP_R_CheckCrc(txn) }, CF_ERROR);
        ut_cf_assert_event_id(CF_CFDP_R_CRC_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.fault.crc_mismatch, 1);
        }

        // CRC mismatch, class 2
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).crc.result = 0xdeadbeef;
            (*txn).state_data.eof_crc = 0x2badc0de;
        }
        assert_eq!(unsafe { CF_CFDP_R_CheckCrc(txn) }, CF_ERROR);
        ut_cf_assert_event_id(CF_CFDP_R_CRC_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.fault.crc_mismatch, 2);
        }

        // CRC match
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).crc.result = 0xc0ffee;
            (*txn).state_data.eof_crc = 0xc0ffee;
        }
        assert_eq!(unsafe { CF_CFDP_R_CheckCrc(txn) }, CFE_SUCCESS);
        assert_eq!(ut_stub_count(UT_KEY_CFE_EVS_SEND_EVENT), 0);
    }

    #[test]
    fn test_cf_cfdp_r_process_fd() {
        // Test case for:
        // int CF_CFDP_R_ProcessFd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        // nominal
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.fd.data_len = 100;
        }
        ut_set_default_return_value(UT_KEY_CF_WRAPPED_WRITE, unsafe { (*ph).int_header.fd.data_len as i32 });
        assert_eq!(unsafe { CF_CFDP_R_ProcessFd(txn, ph) }, 0);
        unsafe {
            assert_eq!((*txn).state_data.cached_pos, 100);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.recv.file_data_bytes, 100);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_WRAPPED_LSEEK), 0);
        assert_eq!(ut_stub_count(UT_KEY_CF_WRAPPED_WRITE), 1);

        // call again, but for something at a different offset
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.fd.data_len = 100;
            (*ph).int_header.fd.offset = 200;
        }
        ut_set_default_return_value(UT_KEY_CF_WRAPPED_LSEEK, unsafe { (*ph).int_header.fd.offset as i32 });
        assert_eq!(unsafe { CF_CFDP_R_ProcessFd(txn, ph) }, 0);
        unsafe {
            assert_eq!((*txn).state_data.cached_pos, 300);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.recv.file_data_bytes, 200);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_WRAPPED_LSEEK), 1);
        assert_eq!(ut_stub_count(UT_KEY_CF_WRAPPED_WRITE), 2);
        unsafe {
            assert_eq!((*txn).state_data.cached_pos, 300);
        }

        // call again, but with a failed write
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.fd.data_len = 100;
            (*ph).int_header.fd.offset = 300;
            (*txn).state_data.cached_pos = 300;
        }
        ut_set_default_return_value(UT_KEY_CF_WRAPPED_WRITE, -1);
        assert_eq!(unsafe { CF_CFDP_R_ProcessFd(txn, ph) }, -1);
        unsafe {
            assert_eq!((*txn).state_data.cached_pos, 300);
        }
        ut_cf_assert_event_id(CF_CFDP_R_WRITE_ERR_EID);
        unsafe {
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_FILESTORE_REJECTION);
        }

        // call again, but with a failed lseek
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.fd.data_len = 100;
            (*ph).int_header.fd.offset = 200;
            (*txn).state_data.cached_pos = 300;
        }
        ut_set_default_return_value(UT_KEY_CF_WRAPPED_LSEEK, -1);
        assert_eq!(unsafe { CF_CFDP_R_ProcessFd(txn, ph) }, -1);
        unsafe {
            assert_eq!((*txn).state_data.cached_pos, 300);
        }
        ut_cf_assert_event_id(CF_CFDP_R_SEEK_FD_ERR_EID);
        unsafe {
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_FILE_SIZE_ERROR);
        }

        // these stats should have been updated during the course of this test
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].counters.fault.file_write, 1);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].counters.fault.file_seek, 1);
        }
    }

    #[test]
    fn test_cf_cfdp_r_substate_recv_eof() {
        // Test case for:
        // void CF_CFDP_R_SubstateRecvEof(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        ut_set_default_return_value(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 1);

        // nominal, accept EOF
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.eof_count = 0;
            (*txn).state_data.eof_size = 0;
            (*ph).int_header.eof.size = 10;
            CF_CFDP_R_SubstateRecvEof(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT), 1);
        unsafe {
            assert_eq!((*txn).state_data.eof_size, 10);
        }

        // repeat EOF
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.eof_count = 1;
            (*txn).state_data.eof_size = 10;
            (*ph).int_header.eof.size = 20;
            CF_CFDP_R_SubstateRecvEof(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT), 2);
        unsafe {
            assert_eq!((*txn).state_data.eof_size, 10);
        }

        // At limit
        ut_set_default_return_value(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 0);
        unsafe {
            CF_CFDP_R_SubstateRecvEof(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT), 3);
        unsafe {
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_POS_ACK_LIMIT_REACHED);
        }

        // with failure of CF_CFDP_RecvEof()
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_RECV_EOF, -1);
        unsafe {
            CF_CFDP_R_SubstateRecvEof(txn, ph);
        }
        ut_cf_assert_event_id(CF_CFDP_R_PDU_EOF_ERR_EID);

        // these counters should have been updated during the test
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].counters.recv.error, 1);
        }
    }

    #[test]
    fn test_cf_cfdp_r_substate_recv_file_data() {
        // Test case for:
        // void CF_CFDP_R_SubstateRecvFileData(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        // nominal
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.acknak_count = 1;
            CF_CFDP_R_SubstateRecvFileData(txn, ph);
        }
        unsafe {
            assert_eq!((*txn).state_data.acknak_count, 0); // this resets the counter
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CHUNK_LIST_ADD), 1); // called

        // failure in CF_CFDP_RecvFd (bad packet)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.acknak_count = 1;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_RECV_FD, 1, -1);
        unsafe {
            CF_CFDP_R_SubstateRecvFileData(txn, ph);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CHUNK_LIST_ADD), 1); // NOT called
        unsafe {
            assert_eq!((*txn).state_data.acknak_count, 1); // NOT reset
        }

        // failure in CF_CFDP_R_ProcessFd (via failure of CF_WrappedWrite)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.acknak_count = 1;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_TXN_IS_OK, 1, 0);
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_WRITE, 1, -1);
        unsafe {
            CF_CFDP_R_SubstateRecvFileData(txn, ph);
        }
        unsafe {
            assert_eq!((*txn).state_data.acknak_count, 1); // NOT reset
        }
    }

    #[test]
    fn test_cf_cfdp_r2_gap_compute() {
        // Test case for:
        // void CF_CFDP_R2_GapCompute(const CF_ChunkList_t *chunks, const CF_Chunk_t *chunk, void *opaque);
        let chunks = CF_ChunkList_t::default();
        let mut chunk = CF_Chunk_t::default();
        let mut args = CF_GapComputeArgs_t::default();
        let mut nak = CF_Logical_PduNak_t::default();

        args.nak = &mut nak;

        // nominal
        chunk.offset = 11000;
        chunk.size = 100;
        nak.scope_start = 10000;
        nak.scope_end = 20000;
        unsafe {
            CF_CFDP_R2_GapCompute(&chunks, &chunk, &mut args as *mut _ as *mut std::ffi::c_void);
        }
        assert_eq!(nak.segment_list.num_segments, 1);

        // the offset start/end should be normalized to the scope start/end
        assert_eq!(nak.segment_list.segments[0].offset_start, 1000);
        assert_eq!(nak.segment_list.segments[0].offset_end, 1100);

        // confirm that CF_PDU_MAX_SEGMENTS is not exceeded
        nak.segment_list.num_segments = CF_PDU_MAX_SEGMENTS;
        unsafe {
            CF_CFDP_R2_GapCompute(&chunks, &chunk, &mut args as *mut _ as *mut std::ffi::c_void);
        }
        assert_eq!(nak.segment_list.num_segments, CF_PDU_MAX_SEGMENTS);
    }

    #[test]
    fn test_cf_cfdp_r_substate_send_nak() {
        // Test case for:
        // int CF_CFDP_R_SendNak(CF_Transaction_t *txn);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();
        let mut chunks = CF_ChunkWrapper_t::default();

        // no packet available
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(unsafe { CF_CFDP_R_SendNak(txn) }, CF_SEND_PDU_NO_BUF_AVAIL_ERROR);

        // with md_recv flag false, this should request one by sending a blank NAK
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(unsafe { CF_CFDP_R_SendNak(txn) }, 0);
        ut_cf_assert_event_id(CF_CFDP_R_REQUEST_MD_INF_EID);
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_SEND_NAK), 1);

        // with md_recv flag true, this should call gap compute to assemble the NAK
        // this requires the chunks list to be set up, and by default compute_gaps will
        // return 0 (no gaps) so the transaction goes to complete
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.md_recv = true;
            (*txn).chunks = &mut chunks;
        }
        chunks.chunks.count = 1;
        chunks.chunks.max_chunks = 2;
        assert_eq!(unsafe { CF_CFDP_R_SendNak(txn) }, 0);
        assert_eq!(ut_stub_count(UT_KEY_CF_CHUNK_LIST_COMPUTE_GAPS), 1);
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_SEND_NAK), 1); // did not increment

        // same, but return nonzero number of gaps
        // this also should use the max chunks instead of count
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CHUNK_LIST_COMPUTE_GAPS, 1, 1);
        unsafe {
            (*txn).flags.rx.md_recv = true;
            (*txn).chunks = &mut chunks;
        }
        chunks.chunks.count = 3;
        chunks.chunks.max_chunks = 2;
        assert_eq!(unsafe { CF_CFDP_R_SendNak(txn) }, 0);
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_SEND_NAK), 2);
    }

    #[test]
    fn test_cf_cfdp_r_calc_crc_chunk() {
        // Test case for:
        // int CF_CFDP_R_CalcCrcChunk(CF_Transaction_t *txn);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut config: *mut CF_ConfigTable_t = std::ptr::null_mut();

        // nominal with zero size file
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.com.crc_complete = false;
            CF_CFDP_R_CalcCrcChunk(txn);
        }
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }

        // nominal with non zero size file, runs the loop
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*txn).flags.com.crc_complete = false;
            (*config).rx_crc_calc_bytes_per_wakeup = 100;
            (*txn).fsize = 70;
        }
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_READ, 1, unsafe { (*txn).fsize as i32 });
        unsafe {
            CF_CFDP_R_CalcCrcChunk(txn);
        }
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }

        // nominal with file larger than rx_crc_calc_bytes_per_wakeup
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).rx_crc_calc_bytes_per_wakeup = CF_R2_CRC_CHUNK_SIZE;
            (*txn).fsize = CF_R2_CRC_CHUNK_SIZE + 100;
        }
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_READ, 1, CF_R2_CRC_CHUNK_SIZE as i32);
        unsafe {
            CF_CFDP_R_CalcCrcChunk(txn);
        }
        unsafe {
            assert!(!(*txn).flags.com.crc_complete);
        }

        // nominal with file size larger than CF_R2_CRC_CHUNK_SIZE (this will do 2 reads)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).rx_crc_calc_bytes_per_wakeup = CF_R2_CRC_CHUNK_SIZE * 2;
            (*txn).fsize = CF_R2_CRC_CHUNK_SIZE + 100;
        }
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_READ, 1, CF_R2_CRC_CHUNK_SIZE as i32);
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_READ, 1, 100);
        unsafe {
            CF_CFDP_R_CalcCrcChunk(txn);
        }
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }

        // failure of read
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        unsafe {
            (*config).rx_crc_calc_bytes_per_wakeup = 100;
            (*txn).fsize = 50;
        }
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_READ, 1, -1);
        unsafe {
            CF_CFDP_R_CalcCrcChunk(txn);
        }
        ut_cf_assert_event_id(CF_CFDP_R_READ_ERR_EID);
        unsafe {
            assert!((*txn).flags.com.crc_complete);
            assert_eq!((*(*txn).history).txn_stat, CF_TxnStatus_FILE_SIZE_ERROR);
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.fault.file_read, 1);
        }
    }

    #[test]
    fn test_cf_cfdp_r2_recv_fin_ack() {
        // Test case for:
        // void CF_CFDP_R2_SubstateRecvFinAck(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph);
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        // nominal
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*ph).int_header.ack.ack_directive_code = CF_CFDP_FileDirective_FIN;
            CF_CFDP_R2_SubstateRecvFinAck(txn, ph);
        }
        unsafe {
            assert!((*txn).flags.rx.finack_recv);
        }

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.finack_recv = false;
            (*ph).int_header.ack.ack_directive_code = -1i8 as u8;
            CF_CFDP_R2_SubstateRecvFinAck(txn, ph);
        }
        unsafe {
            assert!(!(*txn).flags.rx.finack_recv);
        }

        // failure in CF_CFDP_RecvAck
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_RECV_ACK, 1, -1);
        unsafe {
            CF_CFDP_R2_SubstateRecvFinAck(txn, ph);
        }
        ut_cf_assert_event_id(CF_CFDP_R_PDU_FINACK_ERR_EID);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[(*txn).chan_num].counters.recv.error, 2);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_complete() {
        // Test Case for:
        // bool CF_CFDP_R_CheckComplete(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        // No flags set
        assert!(!unsafe { CF_CFDP_R_CheckComplete(txn) });

        // Only recv_md, no eof
        unsafe {
            (*txn).flags.rx.md_recv = true;
        }
        assert!(!unsafe { CF_CFDP_R_CheckComplete(txn) });

        // Only eof, no recv_md
        unsafe {
            (*txn).flags.rx.eof_count = 1;
            (*txn).flags.rx.md_recv = false;
        }
        assert!(!unsafe { CF_CFDP_R_CheckComplete(txn) });

        // Got EOF+MD but gaps in file
        unsafe {
            (*txn).flags.rx.eof_count = 1;
            (*txn).flags.rx.md_recv = true;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CHUNK_LIST_COMPUTE_GAPS, 1, 1);
        assert!(!unsafe { CF_CFDP_R_CheckComplete(txn) });

        // No gaps in file
        assert!(unsafe { CF_CFDP_R_CheckComplete(txn) });

        // NAK pending
        unsafe {
            (*txn).flags.rx.send_nak = true;
        }
        assert!(!unsafe { CF_CFDP_R_CheckComplete(txn) });
    }

    #[test]
    fn test_cf_cfdp_r_send_nak() {
        // Test Case for:
        // CF_CFDP_R_SendNak()
    }

    #[test]
    fn test_cf_cfdp_r_calc_crc_start() {
        // Test Case for:
        // void CF_CFDP_R_CalcCrcStart(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // Nominal success
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).fsize = 32;
            (*txn).state_data.cached_pos = (*txn).fsize;
            (*txn).state_data.eof_size = (*txn).fsize;
            CF_CFDP_R_CalcCrcStart(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CRC_START), 1);
        unsafe {
            assert!(!(*txn).flags.com.crc_complete);
            assert_eq!((*txn).state_data.cached_pos, 0);
        }

        // Size mismatch
        ut_reset_state(UT_KEY_CF_CRC_START);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).fsize = 32;
            (*txn).state_data.cached_pos = (*txn).fsize;
            (*txn).state_data.eof_size = (*txn).fsize - 1;
            CF_CFDP_R_CalcCrcStart(txn);
        }
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }
        ut_cf_assert_event_id(CF_CFDP_R_SIZE_MISMATCH_ERR_EID);
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CRC_START), 0);

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_WRAPPED_LSEEK, 1, -1);
        unsafe {
            (*txn).fsize = 32;
            (*txn).state_data.cached_pos = (*txn).fsize;
            (*txn).state_data.eof_size = (*txn).fsize;
            CF_CFDP_R_CalcCrcStart(txn);
        }
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }
        ut_cf_assert_event_id(CF_CFDP_R_SEEK_CRC_ERR_EID);
        unsafe {
            assert!((*txn).flags.com.crc_complete);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CRC_START), 0);
    }

    #[test]
    fn test_cf_cfdp_r2_substate_recv_fin_ack() {
        // Test Case for:
        // CF_CFDP_R2_SubstateRecvFinAck()
    }

    #[test]
    fn test_cf_cfdp_r_substate_recv_md() {
        // Test Case for:
        // void CF_CFDP_R_SubstateRecvMd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();
        let mut ph: *mut CF_Logical_PduBuffer_t = std::ptr::null_mut();

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            CF_CFDP_R_SubstateRecvMd(txn, ph);
        }
        unsafe {
            assert!((*txn).flags.rx.md_recv);
        }

        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.md_recv = true;
            CF_CFDP_R_SubstateRecvMd(txn, ph);
        }
        unsafe {
            assert!((*txn).flags.rx.md_recv);
        }

        // Failure in CF_CFDP_RecvMd
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_RECV_MD, 1, -1);
        unsafe {
            CF_CFDP_R_SubstateRecvMd(txn, ph);
        }
        unsafe {
            assert!(!(*txn).flags.rx.md_recv);
        }
    }

    #[test]
    fn test_cf_cfdp_r_handle_file_retention() {
        // Test Case for:
        // void CF_CFDP_R_HandleFileRetention(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // with no FD or tempfile, does nothing, reports invalid
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            CF_CFDP_R_HandleFileRetention(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_INVALID);
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_INVALID);
        }

        // Same but with FD
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).fd = os_object_id_from_integer(1);
            CF_CFDP_R_HandleFileRetention(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_WRAPPED_CLOSE), 1);
        unsafe {
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_INVALID);
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_INVALID);
        }

        // Incomplete file
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.tempfile_created = true;
            (*txn).flags.com.is_complete = false;
            CF_CFDP_R_HandleFileRetention(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_INCOMPLETE);
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_DISCARDED);
        }

        // Complete file, should move it (nominal success case)
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.tempfile_created = true;
            (*txn).flags.com.is_complete = true;
        }
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            CF_CFDP_R_HandleFileRetention(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_OS_MV), 1);
        unsafe {
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_RETAINED);
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_COMPLETE);
        }
        ut_cf_assert_event_id(CF_CFDP_R_FILE_RETAINED_EID);

        // Complete file, CRC check fails
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).flags.rx.tempfile_created = true;
            (*txn).flags.com.is_complete = true;
            (*txn).state_data.eof_crc = 0xac0ffee;
            (*txn).crc.result = 0xabadc0de;
            CF_CFDP_R_HandleFileRetention(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_OS_REMOVE), 1);
        unsafe {
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_DISCARDED);
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_COMPLETE);
        }
        ut_cf_assert_event_id(CF_CFDP_R_NOT_RETAINED_EID);

        // with tempfile_created, move fails
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.tempfile_created = true;
            (*txn).flags.com.is_complete = true;
        }
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        ut_set_deferred_retcode(UT_KEY_OS_MV, 1, OS_ERROR as i32);
        unsafe {
            CF_CFDP_R_HandleFileRetention(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_OS_MV), 1);
        assert_eq!(ut_stub_count(UT_KEY_OS_REMOVE), 1);
        unsafe {
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_DISCARDED_FILESTORE);
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_COMPLETE);
        }
        ut_cf_assert_event_id(CF_CFDP_R_NOT_RETAINED_EID);

        // transaction already in error state, not related to file storage (complete file but errored)
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).flags.rx.tempfile_created = true;
            (*txn).flags.com.is_complete = true;
            (*(*txn).history).txn_stat = CF_TxnStatus_POS_ACK_LIMIT_REACHED;
        }
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 0);
        unsafe {
            CF_CFDP_R_HandleFileRetention(txn);
        }
        assert_eq!(ut_stub_count(UT_KEY_OS_MV), 0);
        assert_eq!(ut_stub_count(UT_KEY_OS_REMOVE), 1);
        unsafe {
            assert_eq!((*txn).state_data.fin_fs, CF_CFDP_FinFileStatus_DISCARDED);
            assert_eq!((*txn).state_data.fin_dc, CF_CFDP_FinDeliveryCode_COMPLETE);
        }
        ut_cf_assert_event_id(CF_CFDP_R_NOT_RETAINED_EID);
    }

    #[test]
    fn test_cf_cfdp_r_check_state_normal() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // DATA_NORMAL state, no EOF
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_NORMAL;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_NORMAL);
        }

        // DATA_NORMAL state, with EOF
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_NORMAL;
            (*txn).flags.rx.eof_count = 1;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
        }

        // DATA_NORMAL state, with error
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_TXN_IS_OK, 1, 0);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_NORMAL;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FILESTORE);
        }

        // DATA_NORMAL state, complete file (R2 nominal)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.eof_count = 1;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_NORMAL;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
            assert!((*txn).flags.rx.send_nak);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_state_eof() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // DATA_EOF state, complete file (R1 nominal)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.rx.eof_count = 1;
            (*txn).flags.rx.md_recv = true;
            (*txn).flags.com.is_complete = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_VALIDATE);
            assert!((*txn).flags.com.is_complete);
            assert!(!(*txn).flags.rx.send_nak);
        }

        // DATA_EOF state, incomplete in R1 (no retry, will keep spinning in EOF until inactive timer)
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.rx.md_recv = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
        }

        // DATA_EOF state, with error
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_TXN_IS_OK, 1, 0);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FILESTORE);
        }

        // DATA_EOF state, incomplete in R2, NAK pending
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.rx.send_nak = true;
            (*txn).flags.rx.md_recv = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
        }

        // DATA_EOF state, incomplete in R2, NAK sent
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = true;
            (*txn).flags.rx.send_nak = false;
            (*txn).flags.rx.md_recv = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
        }

        // DATA_EOF state, incomplete in R2, within nak limit, re-nak
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.rx.send_nak = false;
            (*txn).flags.rx.md_recv = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 1, 1);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_DATA_EOF);
            assert!((*txn).flags.rx.send_nak);
        }

        // DATA_EOF state, incomplete in R2, reached nak limit
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).flags.rx.send_nak = false;
            (*txn).flags.rx.md_recv = false;
            (*txn).state_data.sub_state = CF_RxSubState_DATA_EOF;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 1, 0);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FILESTORE);
            assert!(!(*txn).flags.rx.send_nak);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_state_validate() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // VALIDATE state, nominal, not complete yet
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).fsize = 100;
            (*txn).state_data.cached_pos = 90;
            (*txn).state_data.sub_state = CF_RxSubState_VALIDATE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_VALIDATE);
        }

        // VALIDATE state, nominal, complete
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, 1);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).fsize = 100;
            (*txn).state_data.cached_pos = 100;
            (*txn).state_data.sub_state = CF_RxSubState_VALIDATE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FILESTORE);
        }

        // VALIDATE state, nominal, error
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).fsize = 200;
            (*txn).state_data.cached_pos = 100;
            (*txn).state_data.sub_state = CF_RxSubState_VALIDATE;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_TXN_IS_OK, 1, 0);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FILESTORE);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_state_filestore() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // FILESTORE state, R1, no fin requested
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.com.close_req = false;
            (*txn).state_data.sub_state = CF_RxSubState_FILESTORE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
            assert!(!(*txn).flags.rx.send_fin);
        }

        // FILESTORE state, R1, with fin requested
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.com.close_req = true;
            (*txn).state_data.sub_state = CF_RxSubState_FILESTORE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FINACK);
            assert!((*txn).flags.rx.send_fin);
        }

        // FILESTORE state, R2
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).state_data.sub_state = CF_RxSubState_FILESTORE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FINACK);
            assert!((*txn).flags.rx.send_fin);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_state_finack() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // FINACK state, R1, fin pending
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.rx.send_fin = true;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FINACK);
        }

        // FINACK state, R1, fin sent
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = false;
            (*txn).flags.rx.send_fin = false;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_FINISH_TRANSACTION), 1);

        // FINACK state, R2, fin sent, acked
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.finack_recv = true;
            (*txn).flags.rx.send_fin = false;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_FINISH_TRANSACTION), 1);

        // FINACK state, R2, fin sent, not acked, timer active
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.finack_recv = false;
            (*txn).flags.rx.send_fin = false;
            (*txn).flags.com.ack_timer_armed = true;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FINACK);
        }

        // FINACK state, R2, fin sent, not acked, timer expired, under limit
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.finack_recv = false;
            (*txn).flags.rx.send_fin = false;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 1, 1);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_FINACK);
            assert!((*txn).flags.rx.send_fin);
        }

        // FINACK state, R2, fin sent, not acked, timer expired, over limit
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.finack_recv = false;
            (*txn).flags.rx.send_fin = false;
            (*txn).flags.com.ack_timer_armed = false;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_CHECK_ACK_NAK_COUNT, 1, 0);
        unsafe {
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
            assert!(!(*txn).flags.rx.send_fin);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_FINISH_TRANSACTION), 1);

        // FINACK state, R2, fin sent, not acked, inactivity reached
        ut_reset_state(0);
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).reliable_mode = true;
            (*txn).flags.rx.finack_recv = false;
            (*txn).flags.rx.send_fin = false;
            (*txn).flags.com.inactivity_fired = true;
            (*txn).state_data.sub_state = CF_RxSubState_FINACK;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
            assert!(!(*txn).flags.rx.send_fin);
        }
        assert_eq!(ut_stub_count(UT_KEY_CF_CFDP_FINISH_TRANSACTION), 1);
    }

    #[test]
    fn test_cf_cfdp_r_check_state_complete() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)
        let mut txn: *mut CF_Transaction_t = std::ptr::null_mut();

        // COMPLETE state
        ut_cfdp_r_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        unsafe {
            (*txn).state_data.sub_state = CF_RxSubState_COMPLETE;
            CF_CFDP_R_CheckState(txn);
        }
        unsafe {
            assert_eq!((*txn).state_data.sub_state, CF_RxSubState_COMPLETE);
        }
    }

    #[test]
    fn test_cf_cfdp_r_check_state() {
        // Test Case for:
        // void CF_CFDP_R_CheckState(CF_Transaction_t *txn)

        // This is a state machine and thus has all the if/else and case logic
        // centered in this routine, and therefore needs a lot of cases to exercise
        // all the if/else conditions.  To make the test cases more manageable, each
        // state gets a separate test routine.

        ut_reset_state(0);
        test_cf_cfdp_r_check_state_normal();
        ut_reset_state(0);
        test_cf_cfdp_r_check_state_eof();
        ut_reset_state(0);
        test_cf_cfdp_r_check_state_validate();
        ut_reset_state(0);
        test_cf_cfdp_r_check_state_filestore();
        ut_reset_state(0);
        test_cf_cfdp_r_check_state_finack();
        ut_reset_state(0);
        test_cf_cfdp_r_check_state_complete();
    }
}