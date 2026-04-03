use std::ptr;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(i32)]
    pub enum CFCListTraverseStatus {
        Cont = 0,
        Exit = 1,
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct CFCListNode {
        pub next: *mut CFCListNode,
        pub prev: *mut CFCListNode,
    }

    impl Default for CFCListNode {
        fn default() -> Self {
            Self {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }
        }
    }

    pub fn cf_clist_init_node(node: &mut CFCListNode) {
        node.next = node as *mut CFCListNode;
        node.prev = node as *mut CFCListNode;
    }

    pub fn cf_clist_insert_front(head: &mut *mut CFCListNode, node: &mut CFCListNode) {
        if head.is_null() {
            cf_clist_init_node(node);
            *head = node as *mut CFCListNode;
        } else {
            unsafe {
                let old_head = *head;
                let tail = (*old_head).prev;
                
                node.next = old_head;
                node.prev = tail;
                (*old_head).prev = node as *mut CFCListNode;
                (*tail).next = node as *mut CFCListNode;
                *head = node as *mut CFCListNode;
            }
        }
    }

    pub fn cf_clist_insert_back(head: &mut *mut CFCListNode, node: &mut CFCListNode) {
        if head.is_null() {
            cf_clist_init_node(node);
            *head = node as *mut CFCListNode;
        } else {
            unsafe {
                let head_node = *head;
                let tail = (*head_node).prev;
                
                node.next = head_node;
                node.prev = tail;
                (*head_node).prev = node as *mut CFCListNode;
                (*tail).next = node as *mut CFCListNode;
            }
        }
    }

    pub fn cf_clist_pop(head: &mut *mut CFCListNode) -> *mut CFCListNode {
        if head.is_null() {
            return ptr::null_mut();
        }

        unsafe {
            let old_head = *head;
            let next = (*old_head).next;
            
            if next == old_head {
                *head = ptr::null_mut();
            } else {
                let tail = (*old_head).prev;
                (*next).prev = tail;
                (*tail).next = next;
                *head = next;
            }
            
            old_head
        }
    }

    pub fn cf_clist_remove(head: &mut *mut CFCListNode, node: &mut CFCListNode) {
        unsafe {
            let node_ptr = node as *mut CFCListNode;
            let next = node.next;
            let prev = node.prev;
            
            if next == node_ptr {
                *head = ptr::null_mut();
            } else {
                (*next).prev = prev;
                (*prev).next = next;
                if *head == node_ptr {
                    *head = next;
                }
            }
        }
    }

    pub fn cf_clist_insert_after(head: &mut *mut CFCListNode, start: &mut CFCListNode, after: &mut CFCListNode) {
        unsafe {
            let next = start.next;
            
            after.next = next;
            after.prev = start as *mut CFCListNode;
            start.next = after as *mut CFCListNode;
            (*next).prev = after as *mut CFCListNode;
        }
    }

    pub fn cf_clist_traverse<F>(start: *mut CFCListNode, func: F, context: &mut i32)
    where
        F: Fn(*mut CFCListNode, &mut i32) -> CFCListTraverseStatus,
    {
        if start.is_null() {
            return;
        }

        unsafe {
            let mut current = start;
            loop {
                let next = (*current).next;
                let status = func(current, context);
                
                if status == CFCListTraverseStatus::Exit {
                    break;
                }
                
                if next == start || (*current).next == current {
                    break;
                }
                
                current = next;
            }
        }
    }

    pub fn cf_clist_traverse_r<F>(start: *mut CFCListNode, func: F, context: &mut i32)
    where
        F: Fn(*mut CFCListNode, &mut i32) -> CFCListTraverseStatus,
    {
        if start.is_null() {
            return;
        }

        unsafe {
            if (*start).prev.is_null() {
                return;
            }

            let mut current = (*start).prev;
            loop {
                let prev = (*current).prev;
                let status = func(current, context);
                
                if status == CFCListTraverseStatus::Exit {
                    break;
                }
                
                if prev == (*start).prev || (*current).prev == current {
                    break;
                }
                
                current = prev;
            }
        }
    }

    fn ut_clist_fn(node: *mut CFCListNode, context: &mut i32) -> CFCListTraverseStatus {
        *context += 1;
        if *context == 0 {
            CFCListTraverseStatus::Exit
        } else {
            CFCListTraverseStatus::Cont
        }
    }

    fn ut_clist_fn_rm(node: *mut CFCListNode, context: &mut i32) -> CFCListTraverseStatus {
        *context -= 1;
        unsafe {
            (*node).next = node;
            (*node).prev = node;
        }
        CFCListTraverseStatus::Cont
    }

    #[test]
    fn test_cf_clist_init_node() {
        let mut node = CFCListNode::default();

        cf_clist_init_node(&mut node);
        assert_eq!(node.next as *const CFCListNode, &node as *const CFCListNode);
        assert_eq!(node.prev as *const CFCListNode, &node as *const CFCListNode);
    }

    #[test]
    fn test_cf_clist_insert_front() {
        let mut node = [CFCListNode::default(); 3];
        let mut head: *mut CFCListNode = ptr::null_mut();

        cf_clist_init_node(&mut node[0]);
        cf_clist_init_node(&mut node[1]);
        cf_clist_init_node(&mut node[2]);

        cf_clist_insert_front(&mut head, &mut node[0]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);

        cf_clist_insert_front(&mut head, &mut node[1]);
        assert_eq!(head, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);

        cf_clist_insert_front(&mut head, &mut node[2]);
        assert_eq!(head, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[0] as *mut CFCListNode);
    }

    #[test]
    fn test_cf_clist_insert_back() {
        let mut node = [CFCListNode::default(); 3];
        let mut head: *mut CFCListNode = ptr::null_mut();

        cf_clist_init_node(&mut node[0]);
        cf_clist_init_node(&mut node[1]);
        cf_clist_init_node(&mut node[2]);

        cf_clist_insert_back(&mut head, &mut node[0]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);

        cf_clist_insert_back(&mut head, &mut node[1]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);

        cf_clist_insert_back(&mut head, &mut node[2]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[1] as *mut CFCListNode);
    }

    #[test]
    fn test_cf_clist_pop() {
        let mut node = [CFCListNode::default(); 3];
        let mut head: *mut CFCListNode = ptr::null_mut();

        cf_clist_init_node(&mut node[0]);
        cf_clist_init_node(&mut node[1]);
        cf_clist_init_node(&mut node[2]);
        cf_clist_insert_back(&mut head, &mut node[0]);
        cf_clist_insert_back(&mut head, &mut node[1]);
        cf_clist_insert_back(&mut head, &mut node[2]);

        assert_eq!(cf_clist_pop(&mut head), &mut node[0] as *mut CFCListNode);
        assert_eq!(head, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[1] as *mut CFCListNode);

        assert_eq!(cf_clist_pop(&mut head), &mut node[1] as *mut CFCListNode);
        assert_eq!(head, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[2] as *mut CFCListNode);

        assert_eq!(cf_clist_pop(&mut head), &mut node[2] as *mut CFCListNode);
        assert_eq!(head, ptr::null_mut());

        assert_eq!(cf_clist_pop(&mut head), ptr::null_mut());
    }

    #[test]
    fn test_cf_clist_remove() {
        let mut node = [CFCListNode::default(); 3];
        let mut head: *mut CFCListNode = ptr::null_mut();

        cf_clist_init_node(&mut node[0]);
        cf_clist_init_node(&mut node[1]);
        cf_clist_init_node(&mut node[2]);
        cf_clist_insert_back(&mut head, &mut node[0]);
        cf_clist_insert_back(&mut head, &mut node[1]);
        cf_clist_insert_back(&mut head, &mut node[2]);

        cf_clist_remove(&mut head, &mut node[1]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[0] as *mut CFCListNode);

        cf_clist_remove(&mut head, &mut node[2]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[0] as *mut CFCListNode);
    }

    #[test]
    fn test_cf_clist_insert_after() {
        let mut node = [CFCListNode::default(); 4];
        let mut head = &mut node[0] as *mut CFCListNode;

        cf_clist_init_node(&mut node[0]);
        cf_clist_init_node(&mut node[1]);
        cf_clist_init_node(&mut node[2]);
        cf_clist_init_node(&mut node[3]);

        cf_clist_insert_after(&mut head, &mut node[0], &mut node[1]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);

        cf_clist_insert_after(&mut head, &mut node[1], &mut node[2]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[1] as *mut CFCListNode);

        cf_clist_insert_after(&mut head, &mut node[1], &mut node[3]);
        assert_eq!(head, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[0].next, &mut node[1] as *mut CFCListNode);
        assert_eq!(node[0].prev, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[1].next, &mut node[3] as *mut CFCListNode);
        assert_eq!(node[1].prev, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].next, &mut node[0] as *mut CFCListNode);
        assert_eq!(node[2].prev, &mut node[3] as *mut CFCListNode);
        assert_eq!(node[3].next, &mut node[2] as *mut CFCListNode);
        assert_eq!(node[3].prev, &mut node[1] as *mut CFCListNode);
    }

    #[test]
    fn test_cf_clist_traverse() {
        let mut node = [CFCListNode::default(); 2];
        let mut context: i32;

        context = 0;
        cf_clist_traverse(ptr::null_mut(), ut_clist_fn, &mut context);
        assert_eq!(context, 0);

        context = 0;
        node[0].next = &mut node[0] as *mut CFCListNode;
        cf_clist_traverse(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 1);

        context = 0;
        node[0].next = &mut node[1] as *mut CFCListNode;
        node[1].next = &mut node[0] as *mut CFCListNode;
        cf_clist_traverse(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 2);

        context = -1;
        node[0].next = &mut node[1] as *mut CFCListNode;
        cf_clist_traverse(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 0);

        context = 0;
        node[0].next = &mut node[1] as *mut CFCListNode;
        node[1].next = &mut node[1] as *mut CFCListNode;
        cf_clist_traverse(&mut node[0] as *mut CFCListNode, ut_clist_fn_rm, &mut context);
        assert_eq!(context, -2);
    }

    #[test]
    fn test_cf_clist_traverse_r() {
        let mut node = [CFCListNode::default(); 3];
        let mut context: i32;

        context = 0;
        cf_clist_traverse_r(ptr::null_mut(), ut_clist_fn, &mut context);
        assert_eq!(context, 0);

        context = 0;
        cf_clist_traverse_r(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 0);

        context = 0;
        node[0].prev = &mut node[0] as *mut CFCListNode;
        cf_clist_traverse_r(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 1);

        context = 0;
        node[0].prev = &mut node[1] as *mut CFCListNode;
        node[1].prev = &mut node[0] as *mut CFCListNode;
        cf_clist_traverse_r(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 2);

        context = -1;
        node[0].prev = &mut node[1] as *mut CFCListNode;
        cf_clist_traverse_r(&mut node[0] as *mut CFCListNode, ut_clist_fn, &mut context);
        assert_eq!(context, 0);

        context = 0;
        node[0].prev = &mut node[2] as *mut CFCListNode;
        node[1].prev = &mut node[1] as *mut CFCListNode;
        node[2].prev = &mut node[1] as *mut CFCListNode;
        cf_clist_traverse_r(&mut node[0] as *mut CFCListNode, ut_clist_fn_rm, &mut context);
        assert_eq!(context, -2);
    }
}