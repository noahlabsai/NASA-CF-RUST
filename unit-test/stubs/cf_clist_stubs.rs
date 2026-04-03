/*
 * NASA Docket No. GSC-18,447-1, and identified as "CFS CFDP (CF)
 * Application version 3.0.0"
 *
 * Copyright (c) 2019 United States Government as represented by the
 * Administrator of the National Aeronautics and Space Administration.
 * All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License. You may obtain
 * a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Auto-Generated stub implementations for functions defined in cf_clist header

#[cfg(test)]
mod cf_clist_stubs {
    use crate::cf_clist::{CF_CListNode_t, CF_CListFn_t};
    use std::ptr;

    /// Generated stub function for CF_CList_InitNode()
    pub fn cf_clist_init_node(_node: Option<&mut CF_CListNode_t>) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_InsertAfter()
    pub fn cf_clist_insert_after(
        _head: &mut Option<Box<CF_CListNode_t>>,
        _start: Option<&mut CF_CListNode_t>,
        _after: Option<&mut CF_CListNode_t>
    ) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_InsertBack()
    pub fn cf_clist_insert_back(
        _head: &mut Option<Box<CF_CListNode_t>>,
        _node: Option<&mut CF_CListNode_t>
    ) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_InsertFront()
    pub fn cf_clist_insert_front(
        _head: &mut Option<Box<CF_CListNode_t>>,
        _node: Option<&mut CF_CListNode_t>
    ) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_Pop()
    pub fn cf_clist_pop(_head: &mut Option<Box<CF_CListNode_t>>) -> Option<Box<CF_CListNode_t>> {
        None
    }

    /// Generated stub function for CF_CList_Remove()
    pub fn cf_clist_remove(
        _head: &mut Option<Box<CF_CListNode_t>>,
        _node: Option<&mut CF_CListNode_t>
    ) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_Traverse()
    pub fn cf_clist_traverse(
        _start: Option<&CF_CListNode_t>,
        _fn: Option<CF_CListFn_t>,
        _context: Option<&mut dyn std::any::Any>
    ) {
        // Stub implementation - no operation
    }

    /// Generated stub function for CF_CList_Traverse_R()
    pub fn cf_clist_traverse_r(
        _end: Option<&CF_CListNode_t>,
        _fn: Option<CF_CListFn_t>,
        _context: Option<&mut dyn std::any::Any>
    ) {
        // Stub implementation - no operation
    }
}

#[cfg(test)]
pub use cf_clist_stubs::*;