//! CF Application circular list type definitions.
//!
//! Translated from: cf_clist.h

use std::ptr;

/// Status returned by list traversal callbacks.
///
/// Matches C enum `CF_CListTraverse_Status_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CF_CListTraverse_Status_t {
    CF_CListTraverse_Status_CONTINUE = 0,
    CF_CListTraverse_Status_EXIT = 1,
}

/// Constant indicating to continue traversal.
pub const CF_CLIST_CONT: CF_CListTraverse_Status_t =
    CF_CListTraverse_Status_t::CF_CListTraverse_Status_CONTINUE;

/// Constant indicating to stop traversal.
pub const CF_CLIST_EXIT: CF_CListTraverse_Status_t =
    CF_CListTraverse_Status_t::CF_CListTraverse_Status_EXIT;

/// Checks if the list traversal should continue.
///
/// Matches C inline: `CF_CListTraverse_Status_IS_CONTINUE`.
#[inline]
pub fn CF_CListTraverse_Status_IS_CONTINUE(stat: CF_CListTraverse_Status_t) -> bool {
    stat == CF_CListTraverse_Status_t::CF_CListTraverse_Status_CONTINUE
}

/// Node link structure for circular doubly-linked lists.
///
/// Matches C `struct CF_CListNode`. Uses raw pointers to match C semantics
/// exactly — a node points to itself when initialized (not NULL).
#[repr(C)]
pub struct CF_CListNode {
    pub next: *mut CF_CListNode,
    pub prev: *mut CF_CListNode,
}

/// Type alias matching the C typedef `CF_CListNode_t`.
pub type CF_CListNode_t = CF_CListNode;

/// Callback function type for use with `CF_CList_Traverse`.
///
/// Matches C: `typedef CF_CListTraverse_Status_t (*CF_CListFn_t)(CF_CListNode_t *node, void *context);`
///
/// # Safety
/// The function will be called with raw pointers. The implementation must
/// ensure the pointers are valid.
pub type CF_CListFn_t =
    unsafe fn(node: *mut CF_CListNode_t, context: *mut u8) -> CF_CListTraverse_Status_t;

/// `container_of` macro equivalent.
///
/// Given a pointer to a `CF_CListNode_t` member within a larger struct,
/// returns a pointer to the containing struct.
///
/// # Safety
/// - `ptr` must point to a valid `CF_CListNode_t` that is a member of `T` at `offset`.
/// - The resulting pointer must be valid and properly aligned.
#[inline]
pub unsafe fn container_of<T>(ptr: *const CF_CListNode_t, offset: usize) -> *const T {
    (ptr as *const u8).sub(offset) as *const T
}

/// Mutable version of `container_of`.
///
/// # Safety
/// Same as `container_of`, plus no other mutable references may exist.
#[inline]
pub unsafe fn container_of_mut<T>(ptr: *mut CF_CListNode_t, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}
