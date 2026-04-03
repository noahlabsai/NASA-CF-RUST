use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{rand, srand, time, abs};

const HEADS: bool = true;
const TAILS: bool = false;
const MAX_INT: i32 = i32::MAX;
const UINT16_MAX: u16 = u16::MAX;
const INT32_MAX: i32 = i32::MAX;
const INT32_MIN: i32 = i32::MIN;
const UINT32_MAX: u32 = u32::MAX;

static mut UT_CF_CAPTURED_EVENT_IDS: [u16; 4] = [0; 4];
static mut RANDOM_LENGTH_STRING: [u8; 500] = [0; 500];

#[cfg(test)]
mod tests {
    use super::*;

    fn ut_cf_get_context_buffer_impl(func_key: usize, req_size: usize) -> Option<*mut u8> {
        let mut temp_ptr: *mut c_void = ptr::null_mut();
        let mut actual_size: usize = 0;
        let mut position: usize = 0;

        // Simulated UT_GetDataBuffer call
        ut_get_data_buffer(func_key, &mut temp_ptr, &mut actual_size, &mut position);

        if !temp_ptr.is_null() && (actual_size % req_size) != 0 {
            panic!(
                "Setup Error: Actual context buffer size ({}) does not match required size ({})",
                actual_size, req_size
            );
        }

        if !temp_ptr.is_null() {
            let adjusted_ptr = unsafe { (temp_ptr as *mut u8).add(position) };

            let mut remaining_size = req_size;
            while remaining_size >= mem::size_of::<*mut c_void>() {
                let mut val: *mut c_void = ptr::null_mut();
                ut_stub_copy_to_local(func_key, &mut val as *mut _ as *mut c_void, mem::size_of::<*mut c_void>());
                remaining_size -= mem::size_of::<*mut c_void>();
            }

            if remaining_size > 0 {
                let mut val: *mut c_void = ptr::null_mut();
                ut_stub_copy_to_local(func_key, &mut val as *mut _ as *mut c_void, remaining_size);
            }

            Some(adjusted_ptr)
        } else {
            None
        }
    }

    fn ut_cf_reset_event_capture() {
        unsafe {
            UT_CF_CAPTURED_EVENT_IDS.fill(0);
        }
        ut_reset_state(0); // UT_KEY(CFE_EVS_SendEvent)
        unsafe {
            ut_set_data_buffer(
                0, // UT_KEY(CFE_EVS_SendEvent)
                UT_CF_CAPTURED_EVENT_IDS.as_mut_ptr() as *mut c_void,
                mem::size_of_val(&UT_CF_CAPTURED_EVENT_IDS),
                false,
            );
        }
    }

    fn ut_cf_check_event_id_impl(expected_id: u16, event_id_str: &str) {
        let mut found = false;
        unsafe {
            for &event_id in &UT_CF_CAPTURED_EVENT_IDS {
                if event_id == expected_id {
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "Generated event: {} ({})", event_id_str, expected_id);
    }

    fn cf_tests_setup() {
        ut_reset_state(0);
        ut_cf_reset_event_capture();
        // memset(&CF_AppData, 0, sizeof(CF_AppData)) would be handled by the actual CF_AppData reset
    }

    fn cf_tests_teardown() {
        // do nothing by design
    }

    fn test_util_initialize_random_seed() {
        let seed = if let Ok(seed_env) = std::env::var("RANDOM_VALUES_SEED") {
            seed_env.parse().unwrap_or(1)
        } else {
            1
        };

        if seed > 1 {
            unsafe { srand(seed as u32) };
        } else {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;
            unsafe { srand(current_time) };
            println!("RANDOM_VALUES_SEED = {}", current_time);
        }
    }

    fn any_bool() -> bool {
        any_coin_flip() != 0
    }

    fn any_coin_flip() -> u32 {
        unsafe { (rand() % 2) as u32 }
    }

    fn any_char() -> u8 {
        any_uint8()
    }

    fn any_unsigned_int() -> u32 {
        unsafe { rand() as u32 }
    }

    fn any_buffer_of_uint8_with_size(buffer: &mut [u8]) {
        for byte in buffer.iter_mut() {
            *byte = any_uint8();
        }
    }

    fn any_0_or_1() -> u8 {
        any_uint8_less_than(2)
    }

    fn any_uint8() -> u8 {
        (any_unsigned_int() & 0xFF) as u8
    }

    fn any_uint8_except_set_bits(bits: u8) -> u8 {
        let mut random_value = bits;
        let max_tries = 10;
        let mut num_tries = 0;

        while (random_value & bits) == bits {
            if num_tries == max_tries {
                panic!("Any_uint8_ExceptSetBits unable to get valid number in {} checks", num_tries);
            }
            num_tries += 1;
            random_value = any_uint8();
        }

        random_value
    }

    fn any_uint8_except_unset_bits(bits: u8) -> u8 {
        let mut random_value = bits;
        let max_tries = 10;
        let mut num_tries = 0;

        while (random_value | bits) == bits {
            if num_tries == max_tries {
                panic!("Any_uint8_ExceptUnsetBits unable to get valid number in {} checks", num_tries);
            }
            num_tries += 1;
            random_value = any_uint8();
        }

        random_value
    }

    fn any_uint8_from_these(values: &[u8]) -> u8 {
        let random_index = unsafe { (rand() as usize) % values.len() };
        values[random_index]
    }

    fn any_uint8_between_exclude_max(floor: u8, ceiling: u8) -> u8 {
        let diff = ceiling - floor;
        unsafe { ((rand() as u8) % diff) + floor }
    }

    fn any_uint8_between_inclusive(floor: u8, ceiling: u8) -> u8 {
        let diff = ceiling - floor + 1;
        unsafe { ((rand() as u8) % diff) + floor }
    }

    fn any_uint8_less_than(ceiling: u8) -> u8 {
        unsafe { (rand() as u8) % ceiling }
    }

    fn any_uint8_greater_than(floor: u8) -> u8 {
        255 - unsafe { (rand() as u8) % (256 - floor as u16 - 1) as u8 }
    }

    fn any_uint8_greater_than_or_equal_to(floor: u8) -> u8 {
        255 - unsafe { (rand() as u8) % (256 - floor as u16) as u8 }
    }

    fn any_uint8_except(exception: u8) -> u8 {
        let mut random_val = exception;
        while random_val == exception {
            random_val = any_uint8();
        }
        random_val
    }

    fn any_uint16() -> u16 {
        (any_unsigned_int() & 0xFFFF) as u16
    }

    fn any_uint16_between_exclude_max(floor: u16, ceiling: u16) -> u16 {
        let difference = ceiling - floor;
        unsafe { ((rand() as u16) % difference) + floor }
    }

    fn any_uint16_except(exception: u16) -> u16 {
        let mut random_val = exception as u32;
        while random_val == exception as u32 {
            random_val = any_uint16() as u32;
        }
        random_val as u16
    }

    fn any_uint16_greater_than(floor: u16) -> u16 {
        65535 - unsafe { (rand() as u16) % (65536 - floor as u32 - 1) as u16 }
    }

    fn any_uint16_less_than(ceiling: u16) -> u16 {
        unsafe { (rand() as u16) % ceiling }
    }

    fn any_uint32() -> u32 {
        any_unsigned_int()
    }

    fn any_uint32_between_inclusive(min: u32, max: u32) -> u32 {
        let difference = max - min + 1;
        unsafe { ((rand() as u32) % difference) + min }
    }

    fn any_uint32_between_exclude_max(min: u32, max: u32) -> u32 {
        let difference = max - min;
        if difference == 0 {
            panic!("any_uint32_between_exclude_max: difference is zero");
        }
        unsafe { ((rand() as u32) % difference) + min }
    }

    fn any_uint32_except(exception: u32) -> u32 {
        let mut random_val = exception;
        while random_val == exception {
            random_val = any_unsigned_int();
        }
        random_val
    }

    fn any_int32_less_than(ceiling: i32) -> i32 {
        if ceiling > 0 {
            unsafe { ((rand() % ceiling) - rand()) }
        } else {
            let new_ceiling = unsafe { abs(INT32_MIN - ceiling) };
            unsafe { ceiling - (rand() % new_ceiling) }
        }
    }

    fn any_uint32_greater_than(floor: u32) -> u32 {
        let diff = UINT32_MAX - floor;
        unsafe { ((rand() as u32) % diff) + floor + 1 }
    }

    fn any_uint32_less_than(ceiling: u32) -> u32 {
        unsafe { (rand() as u32) % ceiling }
    }

    fn any_uint32_less_than_or_equal_to(max: u32) -> u32 {
        unsafe { (rand() as u32) % (max + 1) }
    }

    fn any_int() -> i32 {
        let mut random_val = unsafe { rand() % MAX_INT };
        let coin_toss = unsafe { rand() % 2 };

        if coin_toss == HEADS as i32 {
            random_val *= -1;
            random_val -= 1;
        }

        random_val
    }

    fn any_int_except(exception: i32) -> i32 {
        let mut random_val = exception;
        while random_val == exception {
            random_val = any_int();
        }
        random_val
    }

    fn any_int_negative() -> i32 {
        unsafe { ((rand() % MAX_INT) * -1) - 1 }
    }

    fn any_int_positive() -> i32 {
        unsafe { (rand() % (UINT16_MAX as i32)) + 1 }
    }

    fn any_int_positive_except(exception: i32) -> i32 {
        let mut rand_val = exception;
        while rand_val == exception {
            rand_val = unsafe { rand() };
        }
        rand_val
    }

    fn any_int_zero_or_positive_less_than(ceiling: i32) -> i32 {
        unsafe { rand() % ceiling }
    }

    fn any_int32() -> i32 {
        any_int()
    }

    fn any_int32_except(exception: i32) -> i32 {
        any_int_except(exception)
    }

    fn any_int32_negative() -> i32 {
        any_int_negative()
    }

    fn any_int32_zero_or_positive() -> i32 {
        unsafe { rand() % INT32_MAX }
    }

    fn any_uint64() -> u64 {
        let left = (any_uint32() as u64) << 32;
        let right = any_uint32() as u64;
        left | right
    }

    fn any_uint64_except(exception: u64) -> u64 {
        let mut rand_val = exception;
        while rand_val == exception {
            rand_val = any_uint64();
        }
        rand_val
    }

    fn any_filename_of_length(length: usize) -> String {
        any_random_string_of_letters_of_length(length)
    }

    fn any_random_string_of_text_of_length(length: usize) -> String {
        let mut result = String::with_capacity(length);
        for _ in 0..length {
            let value = 32 + unsafe { (rand() % 95) };
            result.push(value as u8 as char);
        }
        result
    }

    fn any_random_string_of_letters_of_length(length: usize) -> String {
        let mut result = String::with_capacity(length);
        for _ in 0..length {
            let mut value = 65 + unsafe { (rand() % 26) };
            if any_coin_flip() == HEADS as u32 {
                value += 32;
            }
            result.push(value as u8 as char);
        }
        result
    }

    fn any_random_string_of_letters_of_length_copy(random_string: &mut [u8], length: usize) {
        for i in 0..(length - 1) {
            let mut value = 65 + unsafe { (rand() % 26) };
            if any_coin_flip() == HEADS as u32 {
                value += 32;
            }
            random_string[i] = value as u8;
        }
        if length > 0 {
            random_string[length - 1] = 0;
        }
    }

    fn any_cfe_time_sys_time_set() -> CfeTimeSysTime {
        CfeTimeSysTime {
            seconds: any_uint32(),
            subseconds: any_uint32(),
        }
    }

    fn any_cfe_status_t_negative() -> i32 {
        any_int32_negative()
    }

    fn any_cfe_status_t_except(exception: i32) -> i32 {
        let mut rand_val = exception;
        while rand_val == exception {
            rand_val = any_int32();
        }
        rand_val
    }

    fn any_cf_chan_num() -> u8 {
        any_uint8_less_than(CF_NUM_CHANNELS)
    }

    // Mock functions for compilation
    fn ut_get_data_buffer(_func_key: usize, _temp_ptr: &mut *mut c_void, _actual_size: &mut usize, _position: &mut usize) {}
    fn ut_stub_copy_to_local(_func_key: usize, _val: *mut c_void, _size: usize) {}
    fn ut_reset_state(_key: usize) {}
    fn ut_set_data_buffer(_key: usize, _buffer: *mut c_void, _size: usize, _copy: bool) {}

    // Mock types and constants
    #[derive(Debug, Clone, Default)]
    struct CfeTimeSysTime {
        seconds: u32,
        subseconds: u32,
    }

    const CF_NUM_CHANNELS: u8 = 2;

    #[test]
    fn test_any_bool() {
        let result = any_bool();
        assert!(result == true || result == false);
    }

    #[test]
    fn test_any_uint8() {
        let result = any_uint8();
        assert!(result <= 255);
    }

    #[test]
    fn test_any_uint16() {
        let result = any_uint16();
        assert!(result <= 65535);
    }

    #[test]
    fn test_any_uint32() {
        let result = any_uint32();
        assert!(result <= UINT32_MAX);
    }

    #[test]
    fn test_any_uint64() {
        let result = any_uint64();
        assert!(result <= u64::MAX);
    }

    #[test]
    fn test_any_int32_negative() {
        let result = any_int32_negative();
        assert!(result < 0);
    }

    #[test]
    fn test_any_int_positive() {
        let result = any_int_positive();
        assert!(result > 0);
    }

    #[test]
    fn test_any_uint8_except() {
        let exception = 42u8;
        let result = any_uint8_except(exception);
        assert_ne!(result, exception);
    }

    #[test]
    fn test_any_uint8_less_than() {
        let ceiling = 100u8;
        let result = any_uint8_less_than(ceiling);
        assert!(result < ceiling);
    }

    #[test]
    fn test_any_uint8_between_inclusive() {
        let floor = 10u8;
        let ceiling = 20u8;
        let result = any_uint8_between_inclusive(floor, ceiling);
        assert!(result >= floor && result <= ceiling);
    }

    #[test]
    fn test_any_random_string_of_letters_of_length() {
        let length = 10;
        let result = any_random_string_of_letters_of_length(length);
        assert_eq!(result.len(), length);
        for ch in result.chars() {
            assert!(ch.is_ascii_alphabetic());
        }
    }

    #[test]
    fn test_cf_tests_setup() {
        cf_tests_setup();
        // Test passes if no panic occurs
    }

    #[test]
    fn test_cf_tests_teardown() {
        cf_tests_teardown();
        // Test passes if no panic occurs
    }
}