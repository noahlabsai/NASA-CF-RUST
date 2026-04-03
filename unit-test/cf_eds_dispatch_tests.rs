use crate::cf_test_utils::*;
use crate::cf_dispatch::*;
use crate::cf_eds_dispatcher::*;
use crate::cf_cmd::*;
use crate::cf_msgids::*;
use crate::cf_eventids::*;
use crate::utstubs::*;

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
    fn test_cf_app_pipe() {
        /*
         * Test Case For:
         * void CF_AppPipe
         */
        let mut ut_buf = CFE_SB_Buffer_t::default();

        ut_set_deferred_retcode(UT_KEY_CFE_EDSMSG_DISPATCH, 1, CFE_SUCCESS);

        cf_dispatch_tests_setup();
        
        cf_app_pipe(&mut ut_buf);
        
        cf_dispatch_tests_teardown();
    }
}