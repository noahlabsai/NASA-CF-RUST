//! CF Command handler type definitions.
//!
//! Translated from: cf_cmd.h (type definitions only)

use crate::cf_clist_types::CF_CListTraverse_Status_t;
use crate::cf_msg::CF_UnionArgs_Payload_t;

/// Channel action status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CF_ChanAction_Status_t {
    CF_ChanAction_Status_SUCCESS = 0,
    CF_ChanAction_Status_ERROR   = -1,
}

/// Checks if the channel action was successful
#[inline]
pub fn CF_ChanAction_Status_IS_SUCCESS(stat: CF_ChanAction_Status_t) -> bool {
    stat == CF_ChanAction_Status_t::CF_ChanAction_Status_SUCCESS
}

/// A callback function for use with CF_DoChanAction()
pub type CF_ChanActionFn_t = fn(chan_num: u8, context: *mut u8) -> CF_ChanAction_Status_t;

/// An object to use with channel-scope actions requiring only a boolean argument
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChanAction_BoolArg_t {
    pub barg: bool,
}

/// An object to use with channel-scope actions for suspend/resume
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChanAction_SuspResArg_t {
    pub same: i32,
    pub action: bool,
}

/// An object to use with channel-scope actions that require the message value
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_ChanAction_BoolMsgArg_t {
    pub data: *const CF_UnionArgs_Payload_t,
    pub barg: bool,
}

/// An object to use with channel-scope actions that require the message value
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_ChanAction_MsgArg_t {
    pub data: *const CF_UnionArgs_Payload_t,
}
