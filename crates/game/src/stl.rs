// Based on https://github.com/microsoft/STL/blob/8dc4faadafb52e3e0a627e046b41258032d9bc6a/stl/inc/list#L289-L292
#[repr(C)]
pub struct DoublyLinkedListNode<'a, T> {
    pub next: *mut DoublyLinkedListNode<'a, T>,
    pub prev: *mut DoublyLinkedListNode<'a, T>,
    pub value: T,
}

#[repr(C)]
pub struct DoublyLinkedList<'a, T> {
    pub allocator: usize,
    pub head: &'a DoublyLinkedListNode<'a, T>,
    pub count: u32,
    _pad14: u32,
}

#[repr(C)]
pub struct Vector<T> {
    pub allocator: usize,
    pub begin: *const T,
    pub end: *const T,
    pub cap: *const T,
}

impl<T> Vector<T> {
    /// # Safety
    /// This fn does not validate that the memory pointer at is a valid MSVC
    /// vector.
    pub unsafe fn iter(&self) -> impl Iterator<Item = &T> {
        let mut current = self.begin;
        let end = self.end;

        std::iter::from_fn(move || {
            current = current.add(1);
            if current.sub(1) == end {
                None
            } else {
                Some(current.as_ref().unwrap())
            }
        })
    }
}

#[repr(C)]
pub struct Tree<T> {
    pub allocator: usize,
    pub head: *const TreeNode<T>,
    pub size: usize,
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
