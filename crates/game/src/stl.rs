use std::{collections::VecDeque, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct DoublyLinkedListNode<'a, T> {
    pub next: &'a DoublyLinkedListNode<'a, T>,
    pub previous: &'a DoublyLinkedListNode<'a, T>,
    pub value: T,
}

#[repr(C)]
pub struct DoublyLinkedList<'a, T> {
    pub allocator: usize,
    pub head: &'a DoublyLinkedListNode<'a, T>,
    pub count: u32,
    _pad14: u32,
}

impl<'a, T> DoublyLinkedList<'a, T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut count = self.count;
        let mut current = self.head;

        std::iter::from_fn(move || {
            current = current.next;
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
pub struct Vector<'a, T>
where
    T: Sized,
{
    _phantom: PhantomData<&'a mut [T]>,
    pub allocator: usize,
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<'a, T> Vector<'a, T>
where
    T: Sized,
{
    pub fn iter(&self) -> impl Iterator<Item = &mut T> {
        let mut current = self.begin;
        let end = self.end;

        std::iter::from_fn(move || {
            let result = if current? >= end? {
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
pub struct Tree<'a, T> {
    allocator: usize,
    head: &'a mut TreeNode<'a, T>,
    size: usize,
}

impl<'a, T> Tree<'a, T> {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a mut T> {
        let mut pending = VecDeque::new();

        let start = self.head.parent;
        pending.push_back(start);

        std::iter::from_fn(move || {
            if let Some(entry) = pending.pop_front() {
                let entry = unsafe { entry.as_mut() }?;

                if unsafe { entry.left.as_ref() }
                    .map(|e| e.is_nil == 0)
                    .unwrap_or_default()
                {
                    pending.push_back(entry.left);
                }

                if unsafe { entry.right.as_ref() }
                    .map(|e| e.is_nil == 0)
                    .unwrap_or_default()
                {
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
pub struct TreeNode<'a, T> {
    left: *mut TreeNode<'a, T>,
    parent: *mut TreeNode<'a, T>,
    right: *mut TreeNode<'a, T>,
    black_red: u8,
    is_nil: u8,
    _pad1a: [u8; 6],
    value: T,
}
