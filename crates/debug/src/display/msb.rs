use std::{mem::transmute, ptr::NonNull};

use game::{
    cs::{CSFileImp, MsbFileCap, MsbRepository},
    dlio::{AdapterFileOperator, DLFileDeviceBase, DLFileDeviceManager, DLFileDeviceVmt},
    dlkr::{DLAllocatorBase, DLPlainLightMutex},
    dltx::DLString,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};
use std::io::Cursor;

use super::DebugDisplay;
use game::dlkr::DLAllocatorVmt;

const MSB_DATA: &[u8] = include_bytes!("test.msb.dcx");
const MSB_DATA_SKYBOX: &[u8] = include_bytes!("test_skybox.msb.dcx");

#[repr(C)]
#[derive(Default)]
pub struct StubFileDevice {
    pub vftable: usize,
    unk8: bool,
    pub mutex: DLPlainLightMutex,
}

impl DLFileDeviceVmt for StubFileDevice {
    extern "C" fn destructor(&mut self) {
        tracing::info!("Called destructor");
    }

    extern "C" fn load_file(
        &mut self,
        name_dlstring: &DLString,
        _name_u16: *const u16,
        _param_4: usize,
        allocator: &mut DLAllocatorBase,
        _param_6: bool,
    ) -> *const u8 {
        if name_dlstring.to_string().starts_with("test_map:/") {
            tracing::info!(
                "Found cringe map load {}",
                name_dlstring.to_string().as_str()
            );

            let cursor = if name_dlstring.to_string().contains("99") {
                Cursor::new(MSB_DATA_SKYBOX)
            } else {
                Cursor::new(MSB_DATA)
            };

            let mut operator = AdapterFileOperator::new(cursor);
            operator.io_state = 0x1;
            operator.file_device = Some(NonNull::new(self).expect("Test"));

            let allocation = allocator
                .allocate_aligned(size_of::<AdapterFileOperator<Cursor<&[u8]>, Self>>(), 0x8)
                as *mut u8
                as *mut AdapterFileOperator<Cursor<&[u8]>, Self>;
            tracing::info!("Allocated memory: {allocation:x?}");

            unsafe { *allocation = operator };

            return allocation as *const u8;
        }

        std::ptr::null()
    }

    extern "C" fn file_enumerator(&self) -> *const u8 {
        tracing::info!("Called file enumerator");
        std::ptr::null()
    }
}

impl DebugDisplay for MsbRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            if let Some(_t) =
                ui.begin_table_header("msb-repository-rescaps", [TableColumnSetup::new("Name")])
            {
                for msb in self.res_rep.res_cap_holder.entries() {
                    ui.table_next_column();
                    ui.text(msb.file_cap.res_cap.name.to_string());
                }
            }

            if ui.button("Test MSB Load") {
                let file_device_manager =
                    unsafe { &mut *(0x1448464c0usize as *mut DLFileDeviceManager) };

                let device = Box::leak(Box::new(StubFileDevice::default())) as *mut StubFileDevice
                    as *mut DLFileDeviceBase;
                let device = NonNull::new(device).unwrap();
                file_device_manager.mutex.lock();
                file_device_manager.devices.push(device);
                file_device_manager.mutex.unlock();

                load_msb("test_map:/test.msb.dcx");
                load_msb("test_map:/test_skybox.msb.dcx");
            }
        }
    }
}

fn load_msb(path: &str) {
    // Get some memory
    // let allocator: &mut DLAllocatorBase = unsafe { transmute(0x143b405b8usize) };
    // let allocation = allocator.allocate_aligned(0xC8, 8);
    // tracing::info!("Allocation: {allocation:x?}");

    // Construct MsbFileCap
    //
    // FUN_1401f35e0(GLOBAL_CSFile,virtualPath,param_1->field1_0xb0);
    let cs_file_imp: &mut &mut CSFileImp = unsafe { transmute(0x143d5b0f8usize) };
    let create_msb_filecap: extern "C" fn(
        &mut CSFileImp,
        *const u16,
        usize,
    ) -> NonNull<MsbFileCap> = unsafe { transmute(0x1401f35e0usize) };

    let mut string_bytes = path.encode_utf16().collect::<Vec<_>>();
    string_bytes.push(0x0);

    let msb = unsafe { create_msb_filecap(*cs_file_imp, string_bytes.as_ptr(), 0).as_mut() };
    tracing::info!(
        "File cap {} - {:x?}",
        msb.file_cap.res_cap.name.to_string(),
        msb as *mut _ as usize
    );
}
