use crate::cf_test_utils::*;
use crate::cf_timer::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn cf_timer_tests_setup() {
        cf_tests_setup();
    }

    fn cf_timer_tests_teardown() {
        cf_tests_teardown();
    }

    #[test]
    fn test_cf_timer_sec2ticks_return_expected_value() {
        cf_timer_tests_setup();
        
        // Arrange
        let arg_sec = any_uint32();
        let ticks_per_second = any_uint32();
        let mut config_table = CfConfigTable::default();
        
        config_table.ticks_per_second = ticks_per_second;
        unsafe {
            CF_APP_DATA.config_table = &config_table as *const CfConfigTable;
        }

        // Act & Assert
        assert_eq!(cf_timer_sec2ticks(arg_sec), arg_sec * ticks_per_second);
        
        cf_timer_tests_teardown();
    }

    #[test]
    fn test_cf_timer_init_rel_sec_receive_expected_value() {
        cf_timer_tests_setup();
        
        // Arrange
        let arg_rel_sec = any_uint32();
        let mut timer = CfTimer::default();
        
        // Arrange unstubbalbe: CF_Timer_Sec2Ticks in same file
        let ticks_per_second = any_uint32();
        let mut config_table = CfConfigTable::default();
        
        config_table.ticks_per_second = ticks_per_second;
        unsafe {
            CF_APP_DATA.config_table = &config_table as *const CfConfigTable;
        }
        
        timer.tick = ticks_per_second;

        // Act
        cf_timer_init_rel_sec(&mut timer, arg_rel_sec);

        // Assert
        assert_eq!(timer.tick, arg_rel_sec * ticks_per_second);
        
        cf_timer_tests_teardown();
    }

    #[test]
    fn test_cf_timer_expired_when_t_tick_is_0_return_1() {
        cf_timer_tests_setup();
        
        // Arrange
        let mut timer = CfTimer::default();
        timer.tick = 0;
        let expected_result = 1;

        // Act & Assert
        assert_eq!(cf_timer_expired(&timer), expected_result);
        
        cf_timer_tests_teardown();
    }

    #[test]
    fn test_cf_timer_expired_when_t_tick_is_1_return_0() {
        cf_timer_tests_setup();
        
        // Arrange
        let mut timer = CfTimer::default();
        timer.tick = 1;
        let expected_result = 0;

        // Act & Assert
        assert_eq!(cf_timer_expired(&timer), expected_result);
        
        cf_timer_tests_teardown();
    }

    #[test]
    fn test_cf_timer_expired_when_t_tick_is_any_integer_except_0_return_0() {
        cf_timer_tests_setup();
        
        // Arrange
        let mut timer = CfTimer::default();
        timer.tick = any_int_except(0);
        let expected_result = 0;

        // Act & Assert
        assert_eq!(cf_timer_expired(&timer), expected_result);
        
        cf_timer_tests_teardown();
    }

    #[test]
    fn test_cf_timer_tick_when_t_tick_is_non0_decrement_t_tick() {
        cf_timer_tests_setup();
        
        // Arrange
        let initial_tick = any_uint32_except(0);
        let mut timer = CfTimer::default();
        
        timer.tick = initial_tick;

        // Act
        cf_timer_tick(&mut timer);

        // Assert
        assert_eq!(timer.tick, initial_tick - 1);
        
        cf_timer_tests_teardown();
    }
}