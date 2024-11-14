use std::{collections::VecDeque, marker::PhantomData, ptr::NonNull};

use crate::pointer::OwningPtr;

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
    pub allocator: usize,
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<T> Vector<T>
where
    T: Sized,
{
    pub fn iter(&self) -> impl Iterator<Item = &mut T> {
        let mut current = self.begin;
        let end = self.end;

        std::iter::from_fn(move || {
            let result = if current?.as_ptr() >= end?.as_ptr() {
                None
            } else {
                Some(unsafe { current?.as_mut() })
            };

            current = Some(unsafe { current?.add(1) });
            result
        })
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
