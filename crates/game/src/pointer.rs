use std::{ops::{Deref, DerefMut}, ptr::NonNull};

#[repr(C)]
pub struct OwningPtr<T>(NonNull<T>);

impl<T> OwningPtr<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Deref for OwningPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsRef<T> for OwningPtr<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> DerefMut for OwningPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> AsMut<T> for OwningPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

unsafe impl<T> Send for OwningPtr<T> {}
unsafe impl<T> Sync for OwningPtr<T> where T: Sync {}
