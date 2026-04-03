/// Type for a timer tick count.
///
/// We expect ticks to be 100/sec, so using u32 for sec could have a bounds
/// condition with u32. But, we don't expect to use more than 400,000,000
/// seconds for any reason so let's just live with it.
pub type CF_Timer_Ticks_t = u32;

/// Type for a timer number of seconds.
pub type CF_Timer_Seconds_t = u32;

/// Basic CF timer object — matches C `CF_Timer_t`.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Timer {
    /// Expires when reaches 0.
    pub tick: CF_Timer_Ticks_t,
}

/// Type alias matching the C typedef `CF_Timer_t`.
pub type CF_Timer_t = CF_Timer;