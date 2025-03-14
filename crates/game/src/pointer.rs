use std::{ops::{Deref, DerefMut}, ptr::NonNull};

/// Pointer to a structure that the containing structure owns.
#[repr(C)]
pub struct OwnedPtr<T>(NonNull<T>);

impl<T> OwnedPtr<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Deref for OwnedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsRef<T> for OwnedPtr<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> DerefMut for OwnedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> AsMut<T> for OwnedPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

unsafe impl<T> Send for OwnedPtr<T> {}
unsafe impl<T> Sync for OwnedPtr<T> where T: Sync {}
