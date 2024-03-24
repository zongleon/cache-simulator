use crate::Addr;
use fxhash::FxHashMap;
use std::ptr::NonNull;

struct SplayNode {
    parent: Option<NonNull<Self>>,
    children: [Option<NonNull<Self>>; 2],
    key: Addr,
}

impl SplayNode {
    fn new(key: Addr) -> Self {
        Self {
            parent: None,
            children: [None, None],
            key,
        }
    }
    #[inline(always)]
    unsafe fn get_child(node: NonNull<Self>, dir: usize) -> Option<NonNull<Self>> {
        node.as_ref().children[dir]
    }
    #[inline(always)]
    unsafe fn set_child(mut node: NonNull<Self>, dir: usize, child: Option<NonNull<Self>>) {
        node.as_mut().children[dir] = child;
    }
    #[inline(always)]
    unsafe fn is_right_child(node: NonNull<Self>) -> bool {
        node.as_ref().parent.unwrap_unchecked().as_ref().children[1] == Some(node)
    }
    unsafe fn rotate(mut node: NonNull<Self>) {
        debug_assert!(node.as_ref().parent.is_some());

        let mut parent = node.as_ref().parent.unwrap_unchecked();
        let is_right = Self::is_right_child(node);

        // update grandparent
        // there is no need to maintain node, the count is not changed
        if let Some(grandparent) = parent.as_ref().parent {
            Self::set_child(
                grandparent,
                Self::is_right_child(parent) as usize,
                Some(node),
            );
        }
        node.as_mut().parent = parent.as_ref().parent;

        // hand over child
        let target_child = Self::get_child(node, (!is_right) as usize);
        Self::set_child(parent, is_right as usize, target_child);
        if let Some(mut child) = target_child {
            child.as_mut().parent = Some(parent);
        }

        // adopt parent
        Self::set_child(node, (!is_right) as usize, Some(parent));
        parent.as_mut().parent = Some(node);
    }
    unsafe fn splay(node: NonNull<Self>) {
        while let Some(parent) = node.as_ref().parent {
            if parent.as_ref().parent.is_some() {
                if Self::is_right_child(node) == Self::is_right_child(parent) {
                    Self::rotate(parent);
                } else {
                    Self::rotate(node);
                }
            }
            Self::rotate(node);
        }
    }
    unsafe fn find_leftmost(mut node: NonNull<Self>) -> NonNull<Self> {
        while let Some(child) = Self::get_child(node, 0) {
            node = child;
        }
        node
    }
    unsafe fn find_rightmost(mut node: NonNull<Self>) -> NonNull<Self> {
        while let Some(child) = Self::get_child(node, 1) {
            node = child;
        }
        node
    }
    unsafe fn remove_root(node: NonNull<Self>) -> Option<NonNull<Self>> {
        let left = Self::get_child(node, 0);
        let right = Self::get_child(node, 1);
        if let Some(mut right) = right {
            // separate right
            right.as_mut().parent = None;
            let leftmost = Self::find_leftmost(right);
            Self::splay(leftmost);
            Self::set_child(leftmost, 0, left);
            if let Some(mut left) = left {
                left.as_mut().parent = Some(leftmost);
            }
            Some(leftmost)
        } else {
            // separate left
            if let Some(mut left) = left {
                left.as_mut().parent = None;
            }
            left
        }
    }
    unsafe fn insert_front(node: NonNull<Self>, root: Option<NonNull<Self>>) -> NonNull<Self> {
        Self::set_child(node, 1, root);
        if let Some(mut root) = root {
            root.as_mut().parent = Some(node);
        }
        node
    }
}

pub struct LRUSplay {
    root: Option<NonNull<SplayNode>>,
    handles: FxHashMap<Addr, NonNull<SplayNode>>,
    size: usize,
}

impl LRUSplay {
    pub fn new(size: usize) -> Self {
        Self {
            root: None,
            handles: FxHashMap::default(),
            size,
        }
    }
    pub fn access(&mut self, key: Addr) -> bool {
        unsafe {
            if let Some(node) = self.handles.get(&key) {
                SplayNode::splay(*node);
                let new_root = SplayNode::remove_root(*node);
                let mut node = *node;
                node.as_mut().children = [None, None];
                self.root = Some(SplayNode::insert_front(node, new_root));
                return true;
            } else if self.handles.len() < self.size {
                let node = NonNull::new_unchecked(Box::into_raw(Box::new(SplayNode::new(key))));
                self.handles.insert(key, node);
                self.root = Some(SplayNode::insert_front(node, self.root));
            } else if self.size != 0 {
                let mut rightmost = SplayNode::find_rightmost(self.root.unwrap_unchecked());
                SplayNode::splay(rightmost);
                let new_root = SplayNode::remove_root(rightmost);
                self.handles.remove(&rightmost.as_ref().key);
                *rightmost.as_mut() = SplayNode::new(key);
                self.root = Some(SplayNode::insert_front(rightmost, new_root));
                self.handles.insert(key, rightmost);
            }
            false
        }
    }
}

impl Drop for LRUSplay {
    fn drop(&mut self) {
        for (_, node) in self.handles.drain() {
            unsafe {
                let _ = Box::from_raw(node.as_ptr());
            }
        }
    }
}
