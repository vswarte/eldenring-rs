use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait DLReferenceCountObjectVmt {
    /// Ran when the ref count hits 0?
    fn clean_up(&self);

    fn destructor(&mut self);
}

/// Tracks the amount of references for the deriving class.
///
/// Source of name: RTTI
#[repr(C)]
pub struct DLReferenceCountObjectBase {
    pub vftable: VPtr<dyn DLReferenceCountObjectVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
}

#[repr(C)]
/// Source of name: dantelion2 leak
/// https://archive.org/details/dantelion2
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

    pub fn years(&self) -> u16 {
        (self.date & 0xFFF) as u16
    }
    pub fn milliseconds(&self) -> u16 {
        ((self.date >> 12) & 0x3FF) as u16
    }
    pub fn months(&self) -> u8 {
        ((self.date >> 22) & 0xF) as u8
    }
    pub fn day_of_week(&self) -> u8 {
        ((self.date >> 26) & 0x7) as u8
    }
    pub fn days(&self) -> u8 {
        ((self.date >> 29) & 0x1F) as u8
    }
    pub fn hours(&self) -> u8 {
        ((self.date >> 34) & 0x1F) as u8
    }
    pub fn minutes(&self) -> u8 {
        ((self.date >> 39) & 0x3F) as u8
    }
    pub fn seconds(&self) -> u8 {
        ((self.date >> 45) & 0x3F) as u8
    }
    pub fn is_utc(&self) -> bool {
        (self.date >> 51) & 0x1 != 0
    }
}
