use std::fmt::{Debug, Display};

#[repr(u32)]
#[derive(Debug)]
/// Determines the starting position for passed-in offsets.
pub enum DLFileSeekDirection {
    /// Seek from start of stream.
    Head = 0x0,
    /// Seek from current position.
    Current = 0x1,
    /// Seek from end of stream.
    Tail = 0x2,
}

impl Display for DLFileSeekDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DLFileSeekDirection::Head => write!(f, "Head"),
            DLFileSeekDirection::Current => write!(f, "Current"),
            DLFileSeekDirection::Tail => write!(f, "Tail"),
        }
    }
}

#[repr(i32)]
pub enum DLIOResult {
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

#[repr(C)]
/// Source of name: dantelion2 leak
pub struct DLDateTime {
    /// Set to FILETIME on creation.
    pub time64: u64,
    /// Packed datetime value.
    pub date: u64,
}

impl DLDateTime {
    pub fn new(time64: u64, is_utc: bool) -> Self {
        Self::from_time64(time64, is_utc)
    }

    pub fn from_time64(time64: u64, is_utc: bool) -> Self {
        let mut packed_value: u64 = 0;

        // UTC flag (1 bit)
        packed_value |= if is_utc { 1 } else { 0 };

        // seconds (6 bits)
        packed_value <<= 6;
        packed_value |= (time64 & 0x3F);

        // minutes (6 bits)
        packed_value <<= 6;
        packed_value |= (time64 & 0x3F);

        // hours (5 bits)
        packed_value <<= 5;
        packed_value |= (time64 & 0x1F);

        // day (5 bits)
        packed_value <<= 5;
        packed_value |= (time64 & 0x1F);

        // day of week (3 bits)
        packed_value <<= 3;
        packed_value |= (time64 & 0x7);

        // month (4 bits)
        packed_value <<= 4;
        packed_value |= (time64 & 0xF);

        // milliseconds (10 bits)
        packed_value <<= 10;
        packed_value |= (time64 & 0x3FF);

        // year (12 bits)
        packed_value <<= 12;
        packed_value |= (time64 & 0xFFF);

        Self {
            time64,
            date: packed_value,
        }
    }

    pub fn year(&self) -> u16 {
        (self.date & 0xFFF) as u16
    }
    pub fn milliseconds(&self) -> u16 {
        ((self.date >> 12) & 0x3FF) as u16
    }
    pub fn month(&self) -> u8 {
        ((self.date >> 22) & 0xF) as u8
    }
    pub fn day_of_week(&self) -> u8 {
        ((self.date >> 26) & 0x7) as u8
    }
    pub fn day(&self) -> u8 {
        ((self.date >> 29) & 0x1F) as u8
    }
    pub fn hour(&self) -> u8 {
        ((self.date >> 34) & 0x1F) as u8
    }
    pub fn minute(&self) -> u8 {
        ((self.date >> 39) & 0x3F) as u8
    }
    pub fn second(&self) -> u8 {
        ((self.date >> 45) & 0x3F) as u8
    }
    pub fn is_utc(&self) -> bool {
        (self.date >> 51) & 0x1 != 0
    }
}

#[repr(C)]
pub struct OpenFileMode(pub u32);

impl OpenFileMode {
    pub fn read_only(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn write(&self) -> bool {
        self.0 & 0x2 != 0
    }
    pub fn append(&self) -> bool {
        self.0 & 0x4 != 0
    }
    pub fn detailed_errors(&self) -> bool {
        self.0 & 0x8 != 0
    }
    pub fn async_io(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn unk5(&self) -> bool {
        self.0 & 0x20 != 0
    }
    pub fn no_sharing(&self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn unk7(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

impl Debug for OpenFileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenFileMode {{ read_only: {}, write: {}, append: {}, detailed_errors: {}, async_io: {}, unk5: {}, no_sharing: {}, unk7: {} }}",
            self.read_only(),
            self.write(),
            self.append(),
            self.detailed_errors(),
            self.async_io(),
            self.unk5(),
            self.no_sharing(),
            self.unk7()
        )
    }
}
