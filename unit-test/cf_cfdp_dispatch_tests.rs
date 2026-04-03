use crate::cf_test_utils::*;
use crate::cf_test_alt_handler::*;
use crate::cf_cfdp::*;
use crate::cf_app::*;
use crate::cf_eventids::*;
use crate::cf_cfdp_r::*;
use crate::cf_cfdp_s::*;
use crate::cf_cfdp_dispatch::*;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
enum UtCfSetup {
    Tx,
    Rx,
    None,
}

fn ut_cfdp_dispatch_setup_basic_test_state(
    setup: UtCfSetup,
    pdu_buffer_p: Option<&mut Option<&'static mut CfLogicalPduBuffer>>,
    channel_p: Option<&mut Option<&'static mut CfChannel>>,
    history_p: Option<&mut Option<&'static mut CfHistory>>,
    txn_p: Option<&mut Option<&'static mut CfTransaction>>,
    config_table_p: Option<&mut Option<&'static mut CfConfigTable>>,
) {
    static mut UT_PDU_BUFFER: CfLogicalPduBuffer = CfLogicalPduBuffer {
        pdu_header: CfPduHeader {
            pdu_type: 0,
            txm_mode: 0,
        },
        fdirective: CfFileDirective {
            directive_code: 0,
        },
    };
    static mut UT_HISTORY: CfHistory = CfHistory {};
    static mut UT_TRANSACTION: CfTransaction = CfTransaction {
        history: std::ptr::null_mut(),
        state: CfTxnState::Init,
        state_data: CfTxnStateData {
            sub_state: 0,
        },
        chan_num: 0,
        reliable_mode: false,
    };
    static mut UT_CONFIG_TABLE: CfConfigTable = CfConfigTable {};

    unsafe {
        UT_PDU_BUFFER = mem::zeroed();
        UT_HISTORY = mem::zeroed();
        UT_TRANSACTION = mem::zeroed();
        UT_CONFIG_TABLE = mem::zeroed();

        UT_TRANSACTION.history = &mut UT_HISTORY;
        CF_APP_DATA.config_table = &mut UT_CONFIG_TABLE;

        if let Some(pdu_buffer_ref) = pdu_buffer_p {
            match setup {
                UtCfSetup::Tx | UtCfSetup::Rx => {
                    *pdu_buffer_ref = Some(&mut UT_PDU_BUFFER);
                }
                _ => {
                    *pdu_buffer_ref = None;
                }
            }
        }

        if let Some(channel_ref) = channel_p {
            *channel_ref = Some(&mut CF_APP_DATA.engine.channels[UT_CFDP_CHANNEL]);
        }

        if let Some(history_ref) = history_p {
            *history_ref = Some(&mut UT_HISTORY);
        }

        if let Some(txn_ref) = txn_p {
            *txn_ref = Some(&mut UT_TRANSACTION);
        }

        if let Some(config_table_ref) = config_table_p {
            *config_table_ref = Some(&mut UT_CONFIG_TABLE);
        }
    }

    ut_cf_reset_event_capture();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cf_cfdp_dispatch_tests_setup() {
        cf_tests_setup();
        unsafe {
            CF_APP_DATA = mem::zeroed();
        }
    }

    fn cf_cfdp_dispatch_tests_teardown() {
        cf_tests_teardown();
    }

    #[test]
    fn test_cf_cfdp_r_dispatch_recv() {
        cf_cfdp_dispatch_tests_setup();

        let mut txn: Option<&mut CfTransaction> = None;
        let mut ph: Option<&mut CfLogicalPduBuffer> = None;
        let mut dispatch = CfCfdpRSubstateDispatchTable::default();
        let mut fddt = CfCfdpFileDirectiveDispatchTable::default();

        ut_set_default_return_value(UT_KEY_CF_CFDP_TXN_IS_OK, true);

        fddt.fdirective[CfCfdpFileDirective::Metadata as usize] = Some(cf_cfdp_r1_recv);
        dispatch.state[CfRxSubState::DataEof as usize] = Some(&fddt);

        // nominal (file directive)
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.pdu_type = 0;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, None);
        }

        // nominal (file data)
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.pdu_type = 1;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, None);
        }

        // directive code beyond range
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.fdirective.directive_code = CfCfdpFileDirective::InvalidMax as u8;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, None);
        }
        unsafe {
            assert_eq!(
                CF_APP_DATA.hk.payload.channel_hk[txn.as_ref().unwrap().chan_num as usize]
                    .counters
                    .recv
                    .spurious,
                1
            );
        }
        ut_cf_assert_event_id(CF_CFDP_R_DC_INV_ERR_EID);

        // file data with error
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.pdu_type = 1;
        }
        ut_set_deferred_retcode(UT_KEY_CF_CFDP_TXN_IS_OK, 1, false);
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, None);
        }
        unsafe {
            assert_eq!(
                CF_APP_DATA.hk.payload.channel_hk[txn.as_ref().unwrap().chan_num as usize]
                    .counters
                    .recv
                    .dropped,
                1
            );
        }

        // test actual dispatch
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.fdirective.directive_code = CfCfdpFileDirective::Metadata as u8;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.state_data.sub_state = CfRxSubState::DataEof as u8;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, Some(cf_cfdp_r2_recv));
        }
        ut_assert_stub_count(CF_CFDP_R1_RECV, 1);
        ut_assert_stub_count(CF_CFDP_R2_RECV, 0);

        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.pdu_type = 1;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.state_data.sub_state = CfRxSubState::DataEof as u8;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_r_dispatch_recv(txn_ref, ph_ref, &dispatch, Some(cf_cfdp_r2_recv));
        }
        ut_assert_stub_count(CF_CFDP_R1_RECV, 1);
        ut_assert_stub_count(CF_CFDP_R2_RECV, 1);

        cf_cfdp_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_cfdp_s_dispatch_recv() {
        cf_cfdp_dispatch_tests_setup();

        let mut txn: Option<&mut CfTransaction> = None;
        let mut ph: Option<&mut CfLogicalPduBuffer> = None;
        let mut dispatch = CfCfdpSSubstateRecvDispatchTable::default();
        let mut fddt = CfCfdpFileDirectiveDispatchTable::default();

        fddt.fdirective[CfCfdpFileDirective::Metadata as usize] = Some(cf_cfdp_s1_recv);
        dispatch.substate[CfTxSubState::DataEof as usize] = Some(&fddt);

        // nominal, no handler
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_s_dispatch_recv(txn_ref, ph_ref, &dispatch);
        }

        // directive code beyond range
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.fdirective.directive_code = CfCfdpFileDirective::InvalidMax as u8;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_s_dispatch_recv(txn_ref, ph_ref, &dispatch);
        }
        unsafe {
            assert_eq!(
                CF_APP_DATA.hk.payload.channel_hk[txn.as_ref().unwrap().chan_num as usize]
                    .counters
                    .recv
                    .spurious,
                1
            );
        }
        ut_cf_assert_event_id(CF_CFDP_S_DC_INV_ERR_EID);

        // file data PDU, not expected in this type of txn
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.pdu_type = 1;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_s_dispatch_recv(txn_ref, ph_ref, &dispatch);
        }

        // test actual dispatch
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.fdirective.directive_code = CfCfdpFileDirective::Metadata as u8;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.state_data.sub_state = CfTxSubState::DataEof as u8;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_s_dispatch_recv(txn_ref, ph_ref, &dispatch);
        }
        ut_assert_stub_count(CF_CFDP_S1_RECV, 1);

        cf_cfdp_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_cfdp_rx_state_dispatch() {
        cf_cfdp_dispatch_tests_setup();

        let mut txn: Option<&mut CfTransaction> = None;
        let mut ph: Option<&mut CfLogicalPduBuffer> = None;
        let mut dispatch = CfCfdpTxnRecvDispatchTable::default();

        dispatch.rx[CfTxnState::R1 as usize] = Some(cf_cfdp_r1_recv);
        dispatch.rx[CfTxnState::Drop as usize] = Some(cf_cfdp_recv_drop);

        // nominal, no handler
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.txm_mode = 1;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.state = CfTxnState::Init;
            txn_ref.reliable_mode = false;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_rx_state_dispatch(txn_ref, ph_ref, &dispatch);
        }

        // nominal, with handler
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.txm_mode = 1;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.state = CfTxnState::R1;
            txn_ref.reliable_mode = false;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_rx_state_dispatch(txn_ref, ph_ref, &dispatch);
        }
        ut_assert_stub_count(CF_CFDP_R1_RECV, 1);

        // Got txm_mode = 0 in an R1
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.txm_mode = 0;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.reliable_mode = false;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_rx_state_dispatch(txn_ref, ph_ref, &dispatch);
        }
        ut_assert_stub_count(CF_CFDP_RECV_DROP, 1);

        // Got txm_mode = 1 in an R2
        ut_cfdp_dispatch_setup_basic_test_state(
            UtCfSetup::Rx,
            Some(&mut ph),
            None,
            None,
            Some(&mut txn),
            None,
        );
        if let Some(ph_ref) = ph.as_mut() {
            ph_ref.pdu_header.txm_mode = 1;
        }
        if let Some(txn_ref) = txn.as_mut() {
            txn_ref.reliable_mode = true;
        }
        if let (Some(txn_ref), Some(ph_ref)) = (txn.as_ref(), ph.as_ref()) {
            cf_cfdp_rx_state_dispatch(txn_ref, ph_ref, &dispatch);
        }
        ut_assert_stub_count(CF_CFDP_RECV_DROP, 2);

        cf_cfdp_dispatch_tests_teardown();
    }
}