//! CF Application timer module.
//!
//! A timer in CF is really just a structure that holds a counter that
//! indicates the timer expired when it reaches 0. The goal is that any
//! timer is driven by the scheduler ticks. There is no reason we need
//! any finer grained resolution than this for CF.
//!
//! Translated from: cf_timer.c / cf_timer.h
//!
//! NOTE: The C original accesses `CF_AppData.config_table->ticks_per_second`
//! via a global. In this Rust translation, `ticks_per_second` is passed as
//! a parameter to avoid unsafe global state.

use crate::cf_timer_types::{CF_Timer_Seconds_t, CF_Timer_t};

/// Converts seconds into scheduler ticks.
///
/// C original: `uint32 CF_Timer_Sec2Ticks(CF_Timer_Seconds_t sec)`
/// which does `return sec * CF_AppData.config_table->ticks_per_second;`
///
/// Uses wrapping multiplication to match C unsigned overflow semantics.
pub fn CF_Timer_Sec2Ticks(sec: CF_Timer_Seconds_t, ticks_per_second: u32) -> u32 {
    sec.wrapping_mul(ticks_per_second)
}

/// Initialize a timer with a relative number of seconds.
///
/// C original: `void CF_Timer_InitRelSec(CF_Timer_t *txn, uint32 rel_sec)`
/// which does `txn->tick = CF_Timer_Sec2Ticks(rel_sec);`
pub fn CF_Timer_InitRelSec(txn: &mut CF_Timer_t, rel_sec: CF_Timer_Seconds_t, ticks_per_second: u32) {
    txn.tick = CF_Timer_Sec2Ticks(rel_sec, ticks_per_second);
}

/// Check if a timer has expired.
///
/// C original: `bool CF_Timer_Expired(const CF_Timer_t *txn)`
/// which does `return !txn->tick;`
///
/// Returns `true` if the timer has expired (tick count is 0).
pub fn CF_Timer_Expired(txn: &CF_Timer_t) -> bool {
    txn.tick == 0
}

/// Notify a timer object a tick has occurred.
///
/// C original: `void CF_Timer_Tick(CF_Timer_t *txn)`
/// which does `CF_Assert(txn->tick); --txn->tick;`
///
/// # Panics
/// Panics (via CF_Assert equivalent) if the timer tick count is already 0.
pub fn CF_Timer_Tick(txn: &mut CF_Timer_t) {
    assert!(txn.tick > 0, "CF_Assert: timer tick must be > 0");
    txn.tick -= 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TICKS_PER_SEC: u32 = 100;

    #[test]
    fn test_sec2ticks() {
        assert_eq!(CF_Timer_Sec2Ticks(0, TICKS_PER_SEC), 0);
        assert_eq!(CF_Timer_Sec2Ticks(1, TICKS_PER_SEC), 100);
        assert_eq!(CF_Timer_Sec2Ticks(10, TICKS_PER_SEC), 1000);
    }

    #[test]
    fn test_init_rel_sec() {
        let mut t = CF_Timer_t::default();
        CF_Timer_InitRelSec(&mut t, 5, TICKS_PER_SEC);
        assert_eq!(t.tick, 500);
    }

    #[test]
    fn test_expired() {
        let t0 = CF_Timer_t { tick: 0 };
        let t1 = CF_Timer_t { tick: 1 };
        assert!(CF_Timer_Expired(&t0));
        assert!(!CF_Timer_Expired(&t1));
    }

    #[test]
    fn test_tick_decrement() {
        let mut t = CF_Timer_t { tick: 3 };
        CF_Timer_Tick(&mut t);
        assert_eq!(t.tick, 2);
        CF_Timer_Tick(&mut t);
        assert_eq!(t.tick, 1);
        CF_Timer_Tick(&mut t);
        assert_eq!(t.tick, 0);
        assert!(CF_Timer_Expired(&t));
    }

    #[test]
    #[should_panic(expected = "CF_Assert")]
    fn test_tick_at_zero_panics() {
        let mut t = CF_Timer_t { tick: 0 };
        CF_Timer_Tick(&mut t);
    }
}