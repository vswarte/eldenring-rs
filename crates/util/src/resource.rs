use game::fd4::{FD4ResCap, FD4ResCapHolder};

pub trait FD4ResCapHolderExt<T> {
    unsafe fn entries<'a>(&'a self) -> impl Iterator<Item = &'a FD4ResCap<T>>
    where
        T: 'a;
}

impl<T> FD4ResCapHolderExt<T> for FD4ResCapHolder<T> {
    unsafe fn entries<'a>(&'a self) -> impl Iterator<Item = &'a FD4ResCap<T>>
    where
        T: 'a,
    {
        let bucket_base = self.buckets;
        let mut current_element = unsafe { *bucket_base };
        let bucket_count = self.bucket_count as isize;
        let mut current_bucket = 0isize;

        std::iter::from_fn(move || unsafe {
            // If we dont have an element but we haven't finished the map yet
            // we need to advance to the next bucket until we've found another
            // element.
            while current_element.is_null() && current_bucket < bucket_count - 1 {
                tracing::trace!("Seeking next slot. current_element = {current_element:x?}, current_bucket = {current_bucket}");
                current_bucket += 1;

                let current_bucket_base = bucket_base.offset(current_bucket);
                current_element = if !current_bucket_base.is_null() {
                    *bucket_base.offset(current_bucket)
                } else {
                    std::ptr::null()
                };
            }

            // Move down the bucket if there is an element
            if let Some(element) = current_element.as_ref() {
                tracing::trace!("Found element. current_element = {current_element:x?}");
                current_element = element.header.next_item;
                Some(element)
            } else {
                current_element = std::ptr::null();
                None
            }
        })
    }
}
