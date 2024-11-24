use std::ffi;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::marker;

#[repr(C)]
#[derive(Default)]
pub struct DLBasicString {
    inner: [u8; 0x10],
    pub length: usize,
    pub capacity: usize,
}

impl ToString for DLBasicString {
    fn to_string(&self) -> String {
        let slice_size = self.length * mem::size_of::<u16>();
        let bytes = if slice_size >= 16 {
            let ptr = usize::from_le_bytes(
                self.inner[0..8].try_into().unwrap()
            );

            unsafe {
                slice::from_raw_parts(ptr as *const u16, self.length).to_vec()
            }
        } else {
            self.inner[0..slice_size]
                .chunks_exact(2)
                .map(|c| ((c[1] as u16) << 8) | c[0] as u16)
                .collect::<Vec<u16>>()
        };

        String::from_utf16_lossy(bytes.as_slice())
    }
}

#[repr(C)]
#[derive(Default)]
pub struct DLString {
    allocator: usize,
    inner: DLBasicString,
    unk28: u32,
    unk2c: u32,
}

impl ToString for DLString {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

pub type DLAllocatedString = DLString;

#[repr(C)]
// Unsure what the const generic U is used for 
pub struct DLCodedString<const U: usize> {
    inner: DLBasicString,
}

impl<const U: usize> ToString for DLCodedString<U> {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

#[repr(C)]
// Unsure what the const generic U is used for 
pub struct DLInplaceStr<const U: usize, const N: usize> {
    vftable: usize,
    /// Inner string
    inner: DLBasicString,
    /// Buffer hosting the strings bytes.
    bytes: [u16; N],
}
