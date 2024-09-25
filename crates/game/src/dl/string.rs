use std::ffi;
use std::mem;
use std::slice;
use std::marker;

#[repr(C)]
pub struct DLBasicString<T> {
    union: [u8; 16],
    pub length: usize,
    pub capacity: usize,
    phantom_data: marker::PhantomData<T>,
}

pub type DLWString = DLBasicString<u16>;

impl ToString for DLWString {
    fn to_string(&self) -> String {
        let slice_size = self.length * mem::size_of::<u16>();
        let bytes = if slice_size >= 16 {
            let ptr = usize::from_le_bytes(
                self.union[0..8].try_into().unwrap()
            );

            unsafe {
                slice::from_raw_parts(ptr as *const u16, self.length).to_vec()
            }
        } else {
            self.union[0..slice_size]
                .chunks_exact(2)
                .map(|c| ((c[1] as u16) << 8) | c[0] as u16)
                .collect::<Vec<u16>>()
        };

        String::from_utf16_lossy(bytes.as_slice())
    }
}

pub type DLString = DLBasicString<u8>;

impl ToString for DLString {
    fn to_string(&self) -> String {
        let bytes = if self.length >= 16 {
            let ptr = usize::from_le_bytes(
                self.union[0..8].try_into().unwrap()
            );

            unsafe {
                slice::from_raw_parts(ptr as *const u8, self.length).to_vec()
            }
        } else {
            self.union[0..self.length].to_vec()
        };

        String::from_utf8_lossy(bytes.as_slice()).to_string()
    }
}

