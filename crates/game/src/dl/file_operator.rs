// Vftable buildup:
//  - 0 = DLFileOperator::~DLFileOperator
//  - 9 = DLFileOperator::CreateFile
//  - 10 = DLFileOperator::Open
#[repr(C)]
pub struct DLFileOperatorVMT {
    // Returns instance of DLRuntimeClass describing current class
    pub get_runtime_class: fn(*const CSFile) -> *const ffi::c_void,

    // Destructor
    pub destructor: fn(*const CSFile, u64) -> *const ffi::c_void,

    // Retrieves a FileCap from the first CSFileRepository
    pub get_repository_1_resource: fn(*const CSFile, *const FD4BasicHashString) -> *const ffi::c_void,

    // Seems to be inserting something?
    pub unk3: fn(*const CSFile, *const FD4BasicHashString, *const FD4ResCap<()>, u32),

    // Seems to be inserting something as well?
    // pub unk4: fn(*const CSFile, *const FD4BasicHashString, *const FD4ResCap<()>, u32),
}

// Constructor: 0x141f49730 
// Put on param_1->0x10 at 141f05a4f 
