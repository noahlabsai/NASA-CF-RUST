use crate::cf_test_utils::*;
use crate::cf_dispatch::*;
use crate::cf_cmd::*;
use crate::cf_msgids::*;
use crate::cf_eventids::*;
use std::mem;

#[cfg(test)]
mod tests {
    use super::*;

    fn cf_dispatch_tests_setup() {
        cf_tests_setup();
    }

    fn cf_dispatch_tests_teardown() {
        cf_tests_teardown();
    }

    #[test]
    fn test_cf_process_ground_command_when_cmd_eq_to_cf_num_commands_fail_and_send_event() {
        cf_dispatch_tests_setup();

        let mut utbuf = CFESBBuffer::default();
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = 24;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );

        cf_process_ground_command(&mut utbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 1);
        ut_cf_assert_event_id(CF_CC_ERR_EID);
        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_process_ground_command_when_cmd_greater_than_cf_num_commands_fail_and_send_event() {
        cf_dispatch_tests_setup();

        let mut utbuf = CFESBBuffer::default();
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = 123;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );

        cf_process_ground_command(&mut utbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 1);
        ut_cf_assert_event_id(CF_CC_ERR_EID);
        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_process_ground_command_receives_cmd_and_length_does_not_match_expected_for_that_command_send_event_and_failure() {
        cf_dispatch_tests_setup();

        let mut utbuf = CFESBBuffer::default();
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = CF_NOOP_CC;
        let forced_return_cfe_msg_get_size: CFEMsgSize = mem::size_of::<CFNoopCmd>() + 1;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );
        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_SIZE,
            &forced_return_cfe_msg_get_size as *const _ as *const u8,
            mem::size_of::<CFEMsgSize>(),
            false,
        );

        cf_process_ground_command(&mut utbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_SIZE), 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 1);
        ut_cf_assert_event_id(CF_CMD_LEN_ERR_EID);
        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_process_ground_command_receives_cmd_code_0x00_and_call_cf_noop_cmd_with_msg() {
        cf_dispatch_tests_setup();

        let mut utbuf = CFESBBuffer::default();
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = CF_NOOP_CC;
        let forced_return_cfe_msg_get_size: CFEMsgSize = mem::size_of::<CFNoopCmd>();

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );
        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_SIZE,
            &forced_return_cfe_msg_get_size as *const _ as *const u8,
            mem::size_of::<CFEMsgSize>(),
            false,
        );

        cf_process_ground_command(&mut utbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_SIZE), 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_process_ground_command_receives_cmd_code_0x0c_and_do_nothing_because_fns_12_is_null() {
        cf_dispatch_tests_setup();

        let mut utbuf = CFESBBuffer::default();
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = 0x0C;
        let forced_return_cfe_msg_get_size: CFEMsgSize = 0;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );
        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_SIZE,
            &forced_return_cfe_msg_get_size as *const _ as *const u8,
            mem::size_of::<CFEMsgSize>(),
            false,
        );

        cf_process_ground_command(&mut utbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_SIZE), 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 0);
        assert_eq!(CF_APP_DATA.hk.payload.counters.cmd, 0);
        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 0);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_app_pipe_process_ground_command() {
        cf_dispatch_tests_setup();

        let mut sbbuf = CFESBBuffer::default();
        let forced_msg_id = cfe_sb_value_to_msg_id(CF_CMD_MID);
        let forced_return_cfe_msg_get_fcn_code: CFEMsgFcnCode = 0;
        let forced_return_cfe_msg_get_size: CFEMsgSize = 0;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_MSG_ID,
            &forced_msg_id as *const _ as *const u8,
            mem::size_of::<CFESBMsgId>(),
            false,
        );
        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_FCN_CODE,
            &forced_return_cfe_msg_get_fcn_code as *const _ as *const u8,
            mem::size_of::<CFEMsgFcnCode>(),
            false,
        );
        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_SIZE,
            &forced_return_cfe_msg_get_size as *const _ as *const u8,
            mem::size_of::<CFEMsgSize>(),
            false,
        );

        cf_app_pipe(&mut sbbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_MSG_ID), 1);
        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_FCN_CODE), 1);
        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_SIZE), 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 1);
        assert_eq!(CF_APP_DATA.hk.payload.counters.cmd, 0);
        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_app_pipe_wake_up() {
        cf_dispatch_tests_setup();

        let mut sbbuf = CFESBBuffer::default();
        let forced_msg_id = cfe_sb_value_to_msg_id(CF_WAKE_UP_MID);

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_MSG_ID,
            &forced_msg_id as *const _ as *const u8,
            mem::size_of::<CFESBMsgId>(),
            false,
        );

        cf_app_pipe(&mut sbbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_MSG_ID), 1);
        assert_eq!(ut_assert_stub_count(CF_WAKEUP_CMD), 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_app_pipe_send_hk() {
        cf_dispatch_tests_setup();

        let mut sbbuf = CFESBBuffer::default();
        let forced_msg_id = cfe_sb_value_to_msg_id(CF_SEND_HK_MID);

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_MSG_ID,
            &forced_msg_id as *const _ as *const u8,
            mem::size_of::<CFESBMsgId>(),
            false,
        );

        cf_app_pipe(&mut sbbuf);

        assert_eq!(ut_assert_stub_count(CFE_MSG_GET_MSG_ID), 1);
        assert_eq!(ut_assert_stub_count(CF_SEND_HK_CMD), 1);

        cf_dispatch_tests_teardown();
    }

    #[test]
    fn test_cf_app_pipe_unrecognized_command_enter_default_path() {
        cf_dispatch_tests_setup();

        let forced_msg_id = CFE_SB_INVALID_MSG_ID;

        ut_set_data_buffer(
            UT_KEY_CFE_MSG_GET_MSG_ID,
            &forced_msg_id as *const _ as *const u8,
            mem::size_of::<CFESBMsgId>(),
            false,
        );

        cf_app_pipe(None);

        assert_eq!(CF_APP_DATA.hk.payload.counters.err, 1);
        assert_eq!(ut_assert_stub_count(CFE_EVS_SEND_EVENT), 1);
        ut_cf_assert_event_id(CF_MID_ERR_EID);

        cf_dispatch_tests_teardown();
    }
}