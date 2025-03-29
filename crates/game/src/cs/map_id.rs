use std::fmt::Display;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MapId(pub i32);

impl MapId {
    /// MapId -1 indicating that some entity is global or not segregated by map.
    pub const fn none() -> Self {
        Self::from_parts(-1, -1, -1, -1)
    }

    /// Constructs a MapId from seperate parts.
    pub const fn from_parts(area: i8, block: i8, region: i8, index: i8) -> Self {
        Self(
            (index as i32) | (region as i32) << 8 | (block as i32) << 16 | (area as i32) << 24,
        )
    }

    pub const fn area(&self) -> i32 {
        self.0 >> 24 & 0xFF
    }

    pub const fn block(&self) -> i32 {
        self.0 >> 16 & 0xFF
    }

    pub const fn region(&self) -> i32 {
        self.0 >> 8 & 0xFF
    }

    pub const fn index(&self) -> i32 {
        self.0 & 0xFF
    }

    pub const fn is_overworld(&self) -> bool {
        self.area() >= 50 && self.area() < 89
    }
}

impl From<MapId> for i32 {
    fn from(val: MapId) -> Self {
        val.0
    }
}

impl From<i32> for MapId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m{:0>2}_{:0>2}_{:0>2}_{:0>2}",
            self.area(),
            self.block(),
            self.region(),
            self.index()
        )
    }
}
