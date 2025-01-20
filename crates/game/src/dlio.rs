// Most of the

use std::fmt::Display;
use std::io::{Cursor, SeekFrom};
use std::{
    ffi::c_void,
    io::{Read, Seek},
    ptr::NonNull,
};

use vtable_rs::VPtr;

use crate::dlkr::DLAllocatorVmt;
use crate::{
    dlkr::{DLAllocatorBase, DLPlainLightMutex},
    dltx::{DLBasicString, DLString},
    pointer::OwnedPtr,
    Vector,
};

#[vtable_rs::vtable]
pub trait DLInputStreamVmt {
    // Sets unk28 on DLMemoryInputStream, sets unk10 on DLFileInputStream, sets unk20 on
    // DLSharedMemoryInputStream, does nothing DLBufferedInputStream.
    fn unk1(&mut self, param_2: u32);

    fn destructor(&mut self, param_2: u32);

    // Gets unk28 on DLMemoryInputStream, gets unk10 on DLFileInputStream, gets unk20 on
    // DLSharedMemoryInputStream, does a bunch of stuff on DLBufferedInputStream.
    fn unk3(&self) -> u32;

    /// Reads length amount of bytes from the stream into output. Returns -1 if the read failed,
    /// returns read amount of bytes otherwise.
    fn read_bytes(&mut self, output: *mut u8, length: usize) -> i32;

    /// Indicates if there's bytes left for reading.
    fn has_bytes_left(&self) -> bool;

    /// Indicates the amount of bytes left in the reader.
    fn get_bytes_left(&self) -> usize;

    /// Skips count amount of bytes, returns the amount of bytes skipped. Will be less than count if
    /// position + count exceeds the streams length.
    fn skip_bytes(&mut self, count: usize) -> usize;

    /// Closes the stream causing read functions to stop yielding bytes.
    fn close_stream(&mut self);

    /// Returns true if the stream is open.
    fn stream_open(&self) -> bool;

    fn unk10(&self) -> u32;

    fn unk11(&self) -> u32;

    fn unk12(&self) -> u32;

    /// Calls read_bytes and returns true if the requested read length matches the resulting read
    /// length.
    fn try_read_bytes(&mut self, output: *mut u8, length: usize) -> bool;
}

#[repr(u32)]
#[derive(Debug)]
/// Determines the starting position for passed-in offsets.
pub enum SeekMode {
    /// Seek from current position.
    CurrentPos = 0x1,
    /// Seek from start of stream.
    Start = 0x2,
}

impl Display for SeekMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeekMode::CurrentPos => write!(f, "CurrentPos"),
            SeekMode::Start => write!(f, "Start"),
        }
    }
}

#[vtable_rs::vtable]
pub trait DLSeekableInputStreamVmt: DLInputStreamVmt {
    fn seek(&mut self, offset: usize, mode: SeekMode) -> bool;

    /// Returns the current position in the stream (from the start).
    fn current_position(&self) -> usize;

    /// Checks if the position has reached the end of the stream.
    fn reached_end(&self) -> bool;
}

#[repr(u32)]
pub enum DLFileDeviceDriveType {
    Unknown = 0x0,
    CdRom = 0x1,
    Default = 0x2,
}

#[repr(C)]
pub struct DLFileDeviceBase {
    pub vftable: VPtr<dyn DLFileDeviceVmt, Self>,
    unk8: bool,
    ref_count: u32,
    pub mutex: DLPlainLightMutex,
}

/// Represents a source of files.
#[vtable_rs::vtable]
pub trait DLFileDeviceVmt {
    fn destructor(&mut self);

    fn load_file(
        &mut self,
        name_dlstring: &DLString,
        name_u16: *const u16,
        operator_container: &mut DLFileOperatorContainer,
        allocator: &mut DLAllocatorBase,
        param_6: bool,
    ) -> *const u8;

    fn file_enumerator(&self) -> *const u8;

    fn drive_type(&self) -> DLFileDeviceDriveType {
        DLFileDeviceDriveType::Default
    }

    fn unk5(&self) -> bool {
        false
    }
}

#[repr(C)]
pub struct DLFileEnumeratorSPIBase {
    pub vftable: VPtr<dyn DLFileEnumeratorSPIVmt, Self>,
}

/// Used to locate files from a given device.
#[vtable_rs::vtable]
pub trait DLFileEnumeratorSPIVmt {
    fn destructor(&mut self);

    /// Starts a search using the given search parameter. Writes a handle to search_handle that
    /// can be passed to the rest of the methods. Also writes the first found path to the
    /// found parameter. No results have been found if the found output parameter is empty.
    fn start_search(&mut self, search: &[u16], search_handle: &mut u64, found: &mut DLBasicString);

    /// Stops a search using the search_handle.
    fn close_search(&self, search_handle: &u64);

    /// Attempts to find the next file that matches the search specified when creating the
    /// search_handle. If found is empty after calling no further results have been found.
    fn search_next(&mut self, search_handle: &mut u64, found: &mut DLBasicString);
}

#[repr(i32)]
pub enum DLFileOperatorResult {
    DirNotEmpty = -17,
    OutOfMemory = -13,
    DiskFull = -12,
    NotStreamed = -9,
    AlreadyOpen = -6,
    IsNotOpen = -5,
    NotFound = -4,
    AccessDenied = -3,
    OperationUnsupported = -2,
    Invalid = -1,
    Success = 0,
    NoMoreFiles = 1,
}

/// Represents a remote file abstracting away the storage.
#[vtable_rs::vtable]
pub trait DLFileOperatorVmt {
    fn destructor(&mut self);

    /// Copies the data from the source DLFileOperator into itself.
    fn copy_from(&mut self, source: &DLFileOperatorBase) -> bool;

    fn set_path(&mut self, path: &DLString, param_3: bool) -> bool;

    /// Duplicate of set_path, believed to be for other DLString variants.
    fn set_path_other_1(&mut self, path: &DLString, param_3: bool) -> bool;

    /// Duplicate of set_path, believed to be for other DLString variants.
    fn set_path_other_2(&mut self, path: &DLString, param_3: bool) -> bool;

    /// Duplicate of set_path, believed to be for other DLString variants.
    fn set_path_other_3(&mut self, path: &DLString, param_3: bool) -> bool;

    fn close_file(&mut self) -> bool;

    fn get_virtual_disk_operator(&self) -> OwnedPtr<DLFileOperatorBase>;

    fn bind_device_image(
        &mut self,
        image_spi: &DLFileDeviceImageSPIBase,
    ) -> OwnedPtr<DLFileDeviceImageSPIBase>;

    fn populate_dir_info(&mut self) -> bool;

    fn populate_file_info(&mut self) -> bool;

    fn last_access_time(&self) -> u64;

    fn last_modify_time(&self) -> u64;

    fn file_size(&mut self) -> usize;

    fn remaining_size(&mut self) -> usize;

    fn max_non_streamed_size(&self) -> usize;

    fn truncate_file(&mut self);

    fn has_file_control_0x4(&self) -> bool;

    fn is_directory(&self) -> bool;

    fn is_open(&self) -> bool;

    fn open_file(&mut self, open_mode: u32) -> bool;

    fn try_close_file(&mut self) -> bool;

    fn set_control_unk(&mut self) -> bool;

    fn seek(&mut self, is_stream: bool, offset: i64, seek_mode: SeekMode) -> bool;

    fn cursor_position(&self) -> usize;

    fn read_file(&mut self, output: *mut u8, length: usize) -> usize;

    fn write_file(&mut self, input: *const u8, length: usize) -> usize;

    fn stream_complete_operation(&mut self, handle: *const c_void, length: usize) -> bool;

    fn file_creation_flags(&self) -> u32;

    fn delete_file(&mut self) -> bool;

    fn unk1(&mut self) -> bool;

    fn populate_file_info_2(&mut self) -> bool;

    fn unk2(&mut self) -> bool;

    fn move_file_w(&mut self, path: *const u16) -> bool;

    fn move_file(&mut self, path: *const u8) -> bool;

    fn create_directory(&mut self) -> bool;
}

#[repr(C)]
pub struct DLFileOperatorBase {
    pub vftable: VPtr<dyn DLFileOperatorVmt, Self>,
}

#[vtable_rs::vtable]
pub trait DLFileDeviceImageSPIVmt {
    fn destructor(&mut self);
}

#[repr(C)]
pub struct DLFileDeviceImageSPIBase {
    pub vftable: VPtr<dyn DLFileDeviceImageSPIVmt, Self>,
}

#[repr(C)]
pub struct BndEntry {
    pub name: DLString,
    pub device: NonNull<DLFileDeviceBase>,
    pub file_size: u64,
}

#[repr(C)]
pub struct DLFileOperatorContainer {
    pub allocator: OwnedPtr<DLAllocatorBase>,
    read_file_operator: OwnedPtr<DLFileOperatorBase>,
    write_file_operator: OwnedPtr<DLFileOperatorBase>,
    flags: u32,
}

#[repr(C)]
pub struct DLFileDeviceManager {
    pub devices: Vector<NonNull<DLFileDeviceBase>>,
    pub service_providers: Vector<NonNull<DLFileDeviceImageSPIBase>>,
    pub msvc_file_device: OwnedPtr<DLFileDeviceBase>,
    pub virtual_roots: Vector<DLString>,
    pub bnd3_files: Vector<BndEntry>,
    pub bnd4_files: Vector<BndEntry>,
    pub bnd3_service_provider: OwnedPtr<DLFileDeviceImageSPIBase>,
    pub bnd4_service_provider: OwnedPtr<DLFileDeviceImageSPIBase>,
    pub mutex: DLPlainLightMutex,
}

impl DLFileDeviceVmt for DLFileDeviceBase {
    extern "C" fn destructor(&mut self) {
        (self.vftable.destructor)(self);
    }

    extern "C" fn load_file(
        &mut self,
        name_dlstring: &DLString,
        name_u16: *const u16,
        operator_container: &mut DLFileOperatorContainer,
        allocator: &mut DLAllocatorBase,
        param_6: bool,
    ) -> *const u8 {
        (self.vftable.load_file)(self, name_dlstring, name_u16, operator_container, allocator, param_6)
    }

    extern "C" fn file_enumerator(&self) -> *const u8 {
        (self.vftable.file_enumerator)(self)
    }

    extern "C" fn drive_type(&self) -> DLFileDeviceDriveType {
        (self.vftable.drive_type)(self)
    }

    extern "C" fn unk5(&self) -> bool {
        (self.vftable.unk5)(self)
    }
}

#[repr(C)]
pub struct AdapterFileOperator<R, T>
where
    R: Read + Seek + 'static,
    T: DLFileDeviceVmt,
{
    pub vftable: VPtr<dyn DLFileOperatorVmt, Self>,
    pub allocator: Option<NonNull<DLAllocatorBase>>,
    pub result: DLFileOperatorResult,
    unk18: usize,
    pub io_state: u32,
    pub file_device: Option<NonNull<T>>,
    pub name: DLString,
    buffer: R,
}

impl<R, T> AdapterFileOperator<R, T>
where
    R: Read + Seek + 'static,
    T: DLFileDeviceVmt,
{
    pub fn new(buffer: R) -> Self {
        Self {
            vftable: Default::default(),
            allocator: Default::default(),
            result: DLFileOperatorResult::Success,
            unk18: Default::default(),
            io_state: Default::default(),
            file_device: Default::default(),
            name: Default::default(),
            buffer,
        }
    }
}

impl<R, T> DLFileOperatorVmt for AdapterFileOperator<R, T>
where
    R: Read + Seek + 'static,
    T: DLFileDeviceVmt,
{
    extern "C" fn destructor(&mut self) {
        tracing::info!("AdapterFileOperator::destructor()");
    }

    #[doc = " Copies the data from the source DLFileOperator into itself."]
    extern "C" fn copy_from(&mut self, source: &DLFileOperatorBase) -> bool {
        tracing::info!("AdapterFileOperator::copy_from()");
        unimplemented!()
    }

    extern "C" fn set_path(&mut self, path: &DLString, param_3: bool) -> bool {
        tracing::info!(
            "AdapterFileOperator::set_path({}, {})",
            path.to_string(),
            param_3
        );
        true
    }

    #[doc = " Duplicate of set_path, believed to be for other DLString variants."]
    extern "C" fn set_path_other_1(&mut self, path: &DLString, param_3: bool) -> bool {
        tracing::info!(
            "AdapterFileOperator::set_path_other_1({}, {})",
            path.to_string(),
            param_3
        );
        unimplemented!()
    }

    #[doc = " Duplicate of set_path, believed to be for other DLString variants."]
    extern "C" fn set_path_other_2(&mut self, path: &DLString, param_3: bool) -> bool {
        tracing::info!(
            "AdapterFileOperator::set_path_other_2({}, {})",
            path.to_string(),
            param_3
        );
        unimplemented!()
    }

    #[doc = " Duplicate of set_path, believed to be for other DLString variants."]
    extern "C" fn set_path_other_3(&mut self, path: &DLString, param_3: bool) -> bool {
        tracing::info!(
            "AdapterFileOperator::set_path_other_3({}, {})",
            path.to_string(),
            param_3
        );
        unimplemented!()
    }

    extern "C" fn close_file(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::close_file()");
        unimplemented!()
    }

    extern "C" fn get_virtual_disk_operator(&self) -> OwnedPtr<DLFileOperatorBase> {
        tracing::info!("AdapterFileOperator::get_virtual_disk_operator()");
        unimplemented!()
    }

    extern "C" fn bind_device_image(
        &mut self,
        image_spi: &DLFileDeviceImageSPIBase,
    ) -> OwnedPtr<DLFileDeviceImageSPIBase> {
        tracing::info!("AdapterFileOperator::bind_device_image()");
        unimplemented!()
    }

    extern "C" fn populate_dir_info(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::populate_dir_info()");
        unimplemented!()
    }

    extern "C" fn populate_file_info(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::populate_file_info()");
        unimplemented!()
    }

    extern "C" fn last_access_time(&self) -> u64 {
        tracing::info!("AdapterFileOperator::last_access_time()");
        unimplemented!()
    }

    extern "C" fn last_modify_time(&self) -> u64 {
        tracing::info!("AdapterFileOperator::last_modify_time()");
        unimplemented!()
    }

    extern "C" fn file_size(&mut self) -> usize {
        let current = self.buffer.stream_position().unwrap();
        let end = self.buffer.seek(SeekFrom::End(0)).unwrap() as usize;
        self.buffer.seek(SeekFrom::Start(current));
        tracing::info!("AdapterFileOperator::file_size() -> {end}");
        end
    }

    extern "C" fn remaining_size(&mut self) -> usize {
        tracing::info!("AdapterFileOperator::remaining_size()");
        unimplemented!()
    }

    extern "C" fn max_non_streamed_size(&self) -> usize {
        tracing::info!("AdapterFileOperator::max_non_streamed_size()");
        unimplemented!()
    }

    extern "C" fn truncate_file(&mut self) {
        tracing::info!("AdapterFileOperator::truncate_file()");
        unimplemented!()
    }

    extern "C" fn has_file_control_0x4(&self) -> bool {
        tracing::info!("AdapterFileOperator::has_file_control_0x4()");
        unimplemented!()
    }

    extern "C" fn is_directory(&self) -> bool {
        tracing::info!("AdapterFileOperator::is_directory()");
        unimplemented!()
    }

    extern "C" fn is_open(&self) -> bool {
        tracing::info!("AdapterFileOperator::is_open()");
        true
    }

    extern "C" fn open_file(&mut self, open_mode: u32) -> bool {
        tracing::info!("AdapterFileOperator::open_file({})", open_mode);
        true
    }

    extern "C" fn try_close_file(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::try_close_file()");
        true
    }

    extern "C" fn set_control_unk(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::set_control_unk()");
        unimplemented!()
    }

    extern "C" fn seek(&mut self, is_stream: bool, offset: i64, seek_mode: SeekMode) -> bool {
        tracing::info!(
            "AdapterFileOperator::seek({}, {}, {})",
            is_stream,
            offset,
            seek_mode
        );
        unimplemented!()
    }

    extern "C" fn cursor_position(&self) -> usize {
        tracing::info!("AdapterFileOperator::cursor_position()");
        unimplemented!()
    }

    extern "C" fn read_file(&mut self, output: *mut u8, length: usize) -> usize {
        tracing::info!("AdapterFileOperator::read_file({:x?}, {})", output, length);
        let mut buffer = vec![0x0u8; length];
        self.buffer.read_exact(&mut buffer).unwrap();

        unsafe { std::ptr::copy_nonoverlapping(buffer.as_ptr(), output, length) };
        self.result = DLFileOperatorResult::Success;
        length
    }

    extern "C" fn write_file(&mut self, input: *const u8, length: usize) -> usize {
        tracing::info!("AdapterFileOperator::write_file({:x?}, {})", input, length);
        unimplemented!()
    }

    extern "C" fn stream_complete_operation(
        &mut self,
        handle: *const c_void,
        length: usize,
    ) -> bool {
        tracing::info!(
            "AdapterFileOperator::stream_complete_operation({:x?}, {})",
            handle,
            length
        );
        unimplemented!()
    }

    extern "C" fn file_creation_flags(&self) -> u32 {
        tracing::info!("AdapterFileOperator::file_creation_flags()");
        unimplemented!()
    }

    extern "C" fn delete_file(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::delete_file()");
        unimplemented!()
    }

    extern "C" fn unk1(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::unk1()");
        unimplemented!()
    }

    extern "C" fn populate_file_info_2(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::populate_file_info_2()");
        unimplemented!()
    }

    extern "C" fn unk2(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::unk2()");
        unimplemented!()
    }

    extern "C" fn move_file_w(&mut self, path: *const u16) -> bool {
        tracing::info!("AdapterFileOperator::move_file_w({:x?})", path);
        unimplemented!()
    }

    extern "C" fn move_file(&mut self, path: *const u8) -> bool {
        tracing::info!("AdapterFileOperator::move_file({:x?})", path);
        unimplemented!()
    }

    extern "C" fn create_directory(&mut self) -> bool {
        tracing::info!("AdapterFileOperator::create_directory()");
        unimplemented!()
    }
}
