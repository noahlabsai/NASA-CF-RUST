//! CF Application circular doubly-linked list implementation.
//!
//! This is a circular doubly-linked list implementation. It is used for
//! all data structures in CF.
//!
//! Translated from: cf_clist.c / cf_clist.h
//!
//! # Safety
//! This module uses raw pointers throughout to match the C original exactly.
//! All public functions are `unsafe` because they operate on raw pointers
//! with the same preconditions as the C originals.

use std::ptr;
use crate::cf_clist_types::*;

/// Initialize a clist node — sets next and prev to point to itself.
///
/// C original: `void CF_CList_InitNode(CF_CListNode_t *node)`
/// which does `node->next = node; node->prev = node;`
///
/// # Safety
/// `node` must be a valid, non-null, properly aligned pointer.
pub unsafe fn CF_CList_InitNode(node: *mut CF_CListNode_t) {
    assert!(!node.is_null(), "CF_Assert: node must not be NULL");
    (*node).next = node;
    (*node).prev = node;
}

/// Insert the given node into the front of a list.
///
/// C original: `void CF_CList_InsertFront(CF_CListNode_t **head, CF_CListNode_t *node)`
///
/// # Safety
/// - `head` must be a valid, non-null pointer to a (possibly null) head pointer.
/// - `node` must be a valid, non-null pointer to an initialized node (next==node, prev==node).
pub unsafe fn CF_CList_InsertFront(head: *mut *mut CF_CListNode_t, node: *mut CF_CListNode_t) {
    assert!(!head.is_null(), "CF_Assert: head must not be NULL");
    assert!(!node.is_null(), "CF_Assert: node must not be NULL");
    assert!((*node).next == node, "CF_Assert: node->next == node");
    assert!((*node).prev == node, "CF_Assert: node->prev == node");

    if !(*head).is_null() {
        let last: *mut CF_CListNode_t = (**head).prev;

        (*node).next = *head;
        (*node).prev = last;

        (*last).next = node;
        (**head).prev = node;
    }

    *head = node;
}

/// Insert the given node into the back of a list.
///
/// C original: `void CF_CList_InsertBack(CF_CListNode_t **head, CF_CListNode_t *node)`
///
/// # Safety
/// - `head` must be a valid, non-null pointer to a (possibly null) head pointer.
/// - `node` must be a valid, non-null pointer to an initialized node (next==node, prev==node).
pub unsafe fn CF_CList_InsertBack(head: *mut *mut CF_CListNode_t, node: *mut CF_CListNode_t) {
    assert!(!head.is_null(), "CF_Assert: head must not be NULL");
    assert!(!node.is_null(), "CF_Assert: node must not be NULL");
    assert!((*node).next == node, "CF_Assert: node->next == node");
    assert!((*node).prev == node, "CF_Assert: node->prev == node");

    if (*head).is_null() {
        *head = node;
    } else {
        let last: *mut CF_CListNode_t = (**head).prev;

        (*node).next = *head;
        (**head).prev = node;
        (*node).prev = last;
        (*last).next = node;
    }
}

/// Remove the first node from a list and return it.
///
/// C original: `CF_CListNode_t *CF_CList_Pop(CF_CListNode_t **head)`
///
/// # Safety
/// `head` must be a valid, non-null pointer.
///
/// Returns null if the list was empty.
pub unsafe fn CF_CList_Pop(head: *mut *mut CF_CListNode_t) -> *mut CF_CListNode_t {
    assert!(!head.is_null(), "CF_Assert: head must not be NULL");

    let ret: *mut CF_CListNode_t = *head;
    if !ret.is_null() {
        CF_CList_Remove(head, ret);
    }

    ret
}

/// Remove the given node from the list.
///
/// C original: `void CF_CList_Remove(CF_CListNode_t **head, CF_CListNode_t *node)`
///
/// # Safety
/// - `head` must be a valid, non-null pointer to a non-null head.
/// - `node` must be a valid, non-null pointer to a node that is in the list.
pub unsafe fn CF_CList_Remove(head: *mut *mut CF_CListNode_t, node: *mut CF_CListNode_t) {
    assert!(!head.is_null(), "CF_Assert: head must not be NULL");
    assert!(!node.is_null(), "CF_Assert: node must not be NULL");
    assert!(!(*head).is_null(), "CF_Assert: *head must not be NULL");

    if (*node).next == node {
        // Only node in the list, so this one is easy.
        assert!(*head == node, "CF_Assert: node == *head (sanity check)");
        *head = ptr::null_mut();
    } else if *head == node {
        // Removing the first node in the list, so make the second node the first.
        // C: (*head)->prev->next = node->next;
        //    *head = node->next;
        //    (*head)->prev = node->prev;
        (**head).prev.as_mut().unwrap_unchecked().next = (*node).next;
        *head = (*node).next;
        (**head).prev = (*node).prev;
    } else {
        // Removing a middle or last node.
        (*(*node).next).prev = (*node).prev;
        (*(*node).prev).next = (*node).next;
    }

    CF_CList_InitNode(node);
}

/// Insert the given node after the specified start node.
///
/// C original: `void CF_CList_InsertAfter(CF_CListNode_t **head, CF_CListNode_t *start, CF_CListNode_t *after)`
///
/// # Safety
/// - `head` must be a valid, non-null pointer to a non-null head.
/// - `start` must be a valid, non-null pointer to a node in the list.
/// - `after` must be a valid, non-null pointer to a node NOT in the list.
/// - `start` and `after` must be different nodes.
pub unsafe fn CF_CList_InsertAfter(
    head: *mut *mut CF_CListNode_t,
    start: *mut CF_CListNode_t,
    after: *mut CF_CListNode_t,
) {
    assert!(!head.is_null(), "CF_Assert: head must not be NULL");
    assert!(!(*head).is_null(), "CF_Assert: *head must not be NULL");
    assert!(!start.is_null(), "CF_Assert: start must not be NULL");
    assert!(start != after, "CF_Assert: start != after");

    (*after).next = (*start).next;
    (*start).next = after;
    (*after).prev = start;
    (*(*after).next).prev = after;
}

/// Traverse the entire list, calling the given function on all nodes.
///
/// C original: `void CF_CList_Traverse(CF_CListNode_t *start, CF_CListFn_t fn, void *context)`
///
/// It is OK to delete the current node during traversal, but do NOT delete
/// other nodes in the same list.
///
/// # Safety
/// - If `start` is non-null, it must point to a valid node in a circular list.
/// - `fn_cb` must be a valid function pointer.
/// - `context` must be valid for the duration of the traversal.
pub unsafe fn CF_CList_Traverse(
    start: *mut CF_CListNode_t,
    fn_cb: CF_CListFn_t,
    context: *mut u8,
) {
    let mut node: *mut CF_CListNode_t = start;

    if !node.is_null() {
        let mut start_cur = start;
        let mut last = false;

        loop {
            // Save next in case callback removes this node from the list.
            let node_next: *mut CF_CListNode_t = (*node).next;

            if node_next == start_cur {
                last = true;
            }

            if !CF_CListTraverse_Status_IS_CONTINUE(fn_cb(node, context)) {
                break;
            }

            // List traversal is robust against an item deleting itself during
            // traversal, but there is a special case if that item is the starting
            // node. Since this is a circular list, start is remembered so we know
            // when to stop. Must set start to the next node in this case.
            if (start_cur == node) && ((*node).next != node_next) {
                start_cur = node_next;
            }

            node = node_next;

            if last {
                break;
            }
        }
    }
}

/// Reverse list traversal, starting from end, calling given function on all nodes.
///
/// C original: `void CF_CList_Traverse_R(CF_CListNode_t *end, CF_CListFn_t fn, void *context)`
///
/// traverse_R will work backwards from the parameter's prev, and end on param.
///
/// # Safety
/// - If `end` is non-null, it must point to a valid node in a circular list.
/// - `fn_cb` must be a valid function pointer.
/// - `context` must be valid for the duration of the traversal.
pub unsafe fn CF_CList_Traverse_R(
    end: *mut CF_CListNode_t,
    fn_cb: CF_CListFn_t,
    context: *mut u8,
) {
    if !end.is_null() {
        let mut node: *mut CF_CListNode_t = (*end).prev;

        if !node.is_null() {
            // C: end = node; (end becomes end->prev, the "last" node to visit)
            let mut end_cur: *mut CF_CListNode_t = node;
            let mut last = false;

            loop {
                // Save prev in case callback removes this node from the list.
                let node_next: *mut CF_CListNode_t = (*node).prev;

                if node_next == end_cur {
                    last = true;
                }

                if !CF_CListTraverse_Status_IS_CONTINUE(fn_cb(node, context)) {
                    break;
                }

                // Handle case where node deletes itself during traversal.
                if (end_cur == node) && ((*node).prev != node_next) {
                    end_cur = node_next;
                }

                node = node_next;

                if last {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: create an initialized node on the heap
    fn make_node() -> Box<CF_CListNode_t> {
        let mut node = Box::new(CF_CListNode_t {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        });
        unsafe { CF_CList_InitNode(&mut *node) };
        node
    }

    #[test]
    fn test_init_node() {
        let mut node = make_node();
        let p = &mut *node as *mut CF_CListNode_t;
        assert_eq!(node.next, p);
        assert_eq!(node.prev, p);
    }

    #[test]
    fn test_insert_front_empty() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        unsafe { CF_CList_InsertFront(&mut head, &mut *n1) };
        assert_eq!(head, &mut *n1 as *mut _);
        assert_eq!(n1.next, &mut *n1 as *mut _);
        assert_eq!(n1.prev, &mut *n1 as *mut _);
    }

    #[test]
    fn test_insert_front_two() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        let mut n2 = make_node();
        let p1 = &mut *n1 as *mut CF_CListNode_t;
        let p2 = &mut *n2 as *mut CF_CListNode_t;
        unsafe {
            CF_CList_InsertFront(&mut head, p1);
            CF_CList_InsertFront(&mut head, p2);
        }
        // head -> n2 -> n1 -> (back to n2)
        assert_eq!(head, p2);
        assert_eq!(n2.next, p1);
        assert_eq!(n1.next, p2);
        assert_eq!(n2.prev, p1);
        assert_eq!(n1.prev, p2);
    }

    #[test]
    fn test_insert_back_two() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        let mut n2 = make_node();
        let p1 = &mut *n1 as *mut CF_CListNode_t;
        let p2 = &mut *n2 as *mut CF_CListNode_t;
        unsafe {
            CF_CList_InsertBack(&mut head, p1);
            CF_CList_InsertBack(&mut head, p2);
        }
        // head -> n1 -> n2 -> (back to n1)
        assert_eq!(head, p1);
        assert_eq!(n1.next, p2);
        assert_eq!(n2.next, p1);
        assert_eq!(n1.prev, p2);
        assert_eq!(n2.prev, p1);
    }

    #[test]
    fn test_pop_single() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        let p1 = &mut *n1 as *mut CF_CListNode_t;
        unsafe {
            CF_CList_InsertFront(&mut head, p1);
            let popped = CF_CList_Pop(&mut head);
            assert_eq!(popped, p1);
            assert!(head.is_null());
        }
    }

    #[test]
    fn test_pop_empty() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        unsafe {
            let popped = CF_CList_Pop(&mut head);
            assert!(popped.is_null());
        }
    }

    #[test]
    fn test_remove_middle() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        let mut n2 = make_node();
        let mut n3 = make_node();
        let p1 = &mut *n1 as *mut CF_CListNode_t;
        let p2 = &mut *n2 as *mut CF_CListNode_t;
        let p3 = &mut *n3 as *mut CF_CListNode_t;
        unsafe {
            CF_CList_InsertBack(&mut head, p1);
            CF_CList_InsertBack(&mut head, p2);
            CF_CList_InsertBack(&mut head, p3);
            // head -> n1 -> n2 -> n3 -> (n1)
            CF_CList_Remove(&mut head, p2);
        }
        // head -> n1 -> n3 -> (n1)
        assert_eq!(head, p1);
        assert_eq!(n1.next, p3);
        assert_eq!(n3.next, p1);
        assert_eq!(n1.prev, p3);
        assert_eq!(n3.prev, p1);
        // n2 should be re-initialized (self-pointing)
        assert_eq!(n2.next, p2);
        assert_eq!(n2.prev, p2);
    }

    unsafe fn count_cb(
        _node: *mut CF_CListNode_t,
        context: *mut u8,
    ) -> CF_CListTraverse_Status_t {
        let count = &mut *(context as *mut u32);
        *count += 1;
        CF_CLIST_CONT
    }

    #[test]
    fn test_traverse_three() {
        let mut head: *mut CF_CListNode_t = ptr::null_mut();
        let mut n1 = make_node();
        let mut n2 = make_node();
        let mut n3 = make_node();
        unsafe {
            CF_CList_InsertBack(&mut head, &mut *n1);
            CF_CList_InsertBack(&mut head, &mut *n2);
            CF_CList_InsertBack(&mut head, &mut *n3);
            let mut count: u32 = 0;
            CF_CList_Traverse(head, count_cb, &mut count as *mut u32 as *mut u8);
            assert_eq!(count, 3);
        }
    }

    #[test]
    fn test_traverse_empty() {
        let mut count: u32 = 0;
        unsafe {
            CF_CList_Traverse(
                ptr::null_mut(),
                count_cb,
                &mut count as *mut u32 as *mut u8,
            );
        }
        assert_eq!(count, 0);
    }
}
