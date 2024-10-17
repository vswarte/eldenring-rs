#[repr(C)]
pub struct DoublyLinkedListNode<T> {
    pub next: *mut DoublyLinkedListNode<T>,
    pub previous: *mut DoublyLinkedListNode<T>,
    pub value: T,
}

#[repr(C)]
pub struct DoublyLinkedList<T> {
    pub allocator: usize,
    pub head: *mut DoublyLinkedListNode<T>,
    pub count: u32,
    _pad14: u32,
}

impl<T> DoublyLinkedList<T> {
    /// # Safety
    /// This will produce bad results if:
    /// - The list is projected onto something that isn't actually a list.
    /// - Access is not exclusive and the list gets updated while reading.
    pub unsafe fn iter(&self) -> impl Iterator<Item = &T> {
        let mut count = self.count;
        let mut current = self.head.as_ref().unwrap().next;

        std::iter::from_fn(move || {
            current = (*current).next;
            if count == 0 {
                None
            } else {
                count -= 1;
                current.as_ref()?.previous.as_ref().map(|f| &f.value)
            }
        })
    }

    /// # Safety
    /// This will produce bad results if:
    /// - The list is projected onto something that isn't actually a list.
    /// - Access is not exclusive and the list gets updated while reading.
    pub unsafe fn len(&self) -> usize {
        self.count as usize
    }
}

#[repr(C)]
pub struct Vector<T>
where
    T: Sized,
{
    pub allocator: usize,
    pub begin: *mut T,
    pub end: *mut T,
    pub capacity: *mut T,
}

impl<T> Vector<T>
where
    T: Sized,
{
    /// # Safety
    /// This will produce bad results if:
    /// - The vector is projected onto something that isn't actually a vector.
    /// - The size of T is incorrect.
    /// - Access is not exclusive and the vector gets updated while reading.
    pub unsafe fn iter(&self) -> impl Iterator<Item = &mut T> {
        let mut current = self.begin;
        let end = self.end;

        std::iter::from_fn(move || {
            let result = if current >= end {
                None
            } else {
                Some(current.as_mut().unwrap())
            };
            current = current.add(1);
            result
        })
    }

    /// # Safety
    /// This will produce bad results if:
    /// - The vector is projected onto something that isn't actually a vector.
    /// - The size of T is incorrect.
    /// - Access is not exclusive and the vector gets updated while reading.
    pub unsafe fn len(&self) -> usize {
        (self.end as usize - self.begin as usize) / size_of::<T>()
    }
}

#[repr(C)]
pub struct Tree<T> {
    pub allocator: usize,
    pub head: *const TreeNode<T>,
    pub size: usize,
}

impl<T> Tree<T> {
    /// # Safety
    /// This will produce bad results if:
    /// - The tree is projected onto something that isn't actually a tree.
    /// - Access is not exclusive and the tree gets updated while reading.
    pub unsafe fn len(&self) -> usize {
        self.size
    }
}

#[repr(C)]
pub struct TreeNode<T> {
    pub left: *const TreeNode<T>,
    pub parent: *const TreeNode<T>,
    pub right: *const TreeNode<T>,
    pub black_red: u8,
    pub is_nil: u8,
    _pad0x1c: u32,
    pub value: T,
}
