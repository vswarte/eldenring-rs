use std::{
    collections::VecDeque,
    marker::PhantomData,
    ptr::{copy_nonoverlapping, NonNull},
};

use crate::{dlkr::DLAllocatorBase, pointer::OwnedPtr};

#[repr(C)]
pub struct DoublyLinkedListNode<T> {
    pub next: NonNull<DoublyLinkedListNode<T>>,
    pub previous: NonNull<DoublyLinkedListNode<T>>,
    pub value: T,
}

#[repr(C)]
pub struct DoublyLinkedList<T> {
    pub allocator: usize,
    pub head: NonNull<DoublyLinkedListNode<T>>,
    pub count: u32,
    _pad14: u32,
}

impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut count = self.count;
        let mut current = unsafe { self.head.as_ref() };

        std::iter::from_fn(move || {
            current = unsafe { current.next.as_ref() };
            if count == 0 {
                None
            } else {
                count -= 1;
                Some(&current.value)
            }
        })
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
}

#[repr(C)]
pub struct Vector<T>
where
    T: Sized,
{
    allocator: NonNull<DLAllocatorBase>,
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<T> Vector<T>
where
    T: Sized,
{
    pub fn items(&self) -> &mut [T] {
        let Some(start) = self.begin else {
            return &mut [];
        };

        let end = self.end.unwrap();
        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        unsafe { std::slice::from_raw_parts_mut(start.as_ptr(), count) }
    }

    pub fn len(&self) -> usize {
        let Some(end) = self.end else {
            return 0;
        };

        let Some(start) = self.begin else {
            return 0;
        };

        (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>()
    }

    // TODO: setup CXX for this shit
    pub fn push(&mut self, item: T) {
        let end = self.end.unwrap();
        let new_end = end.as_ptr() as usize + size_of::<T>();

        // Check if we're not going oob, otherwise reloc
        if new_end > self.capacity.unwrap().as_ptr() as usize {
            todo!("Implement vector relocs");
        }

        // Write data to tail
        unsafe { copy_nonoverlapping(&item as _, end.as_ptr(), 1) };

        // Up the end
        self.end = Some(NonNull::new(new_end as *mut T).unwrap());
    }

    // TODO: setup CXX for this shit
    pub fn push_front(&mut self, item: T) {
        let end = self.end.unwrap();
        let start = self.begin.unwrap();
        let new_end = end.as_ptr() as usize + size_of::<T>();

        // Check if we're not going oob, otherwise reloc
        if new_end > self.capacity.unwrap().as_ptr() as usize {
            todo!("Implement vector relocs");
        }

        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        // Copy existing items back one slot
        unsafe { copy_nonoverlapping(start.as_ptr(), start.add(1).as_ptr(), count) }
        // Write data to start
        unsafe { copy_nonoverlapping(&item as _, self.begin.unwrap().as_ptr(), 1) };
    }
}

#[repr(C)]
pub struct Tree<T> {
    allocator: usize,
    head: NonNull<TreeNode<T>>,
    size: usize,
}

impl<T> Tree<T> {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> impl Iterator<Item = &mut T> {
        let mut pending = VecDeque::new();

        let start = unsafe { self.head.as_ref().parent };
        pending.push_back(start);

        // TODO: clean up this code
        std::iter::from_fn(move || {
            if self.size == 0 {
                None
            } else if let Some(mut entry) = pending.pop_front() {
                let entry = unsafe { entry.as_mut() };

                if unsafe { entry.left.as_ref() }.is_nil == 0 {
                    pending.push_back(entry.left);
                }

                if unsafe { entry.right.as_ref() }.is_nil == 0 {
                    pending.push_back(entry.right);
                }

                Some(&mut entry.value)
            } else {
                None
            }
        })
    }
}

#[repr(C)]
pub struct TreeNode<T> {
    left: NonNull<TreeNode<T>>,
    parent: NonNull<TreeNode<T>>,
    right: NonNull<TreeNode<T>>,
    black_red: u8,
    is_nil: u8,
    _pad1a: [u8; 6],
    value: T,
}
