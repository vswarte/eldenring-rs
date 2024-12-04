/// The game has a few different coordinate spaces and it's constantly translating between them.
///
/// The most notable ones probably are:
/// - chunk position (which is how assets, regions, etc are placed)
/// - "global" world position (which seems used primarily used by map cleanup and LOD code).
/// - havok position (seems to be AABB broadphase space, often used where a lot of collision
///       checking happens like the effective player position, the camera position,
///       anything that needs raycasting, etc).
/// - map position (positions on the in-game map, used for pins and the like).
///
/// Converting from one space to another space usually requires some additional data about the new
/// space the coordinates are moving into or from. For example going from chunk local to world "global"
/// coords requires knowing the world coordinates of the chunk center and going from havok position
/// to chunk position requires either the chunk position of the havok aabb center or reference
/// coordinate where both chunk and havok position are known.
use std::{fmt::Display, ops::Sub};

use crate::matrix::{FSPoint, FSVector4};

/// Represents a position relative to some block center.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockPoint(pub FSPoint);

impl BlockPoint {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self(FSPoint(x, y, z))
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

impl Display for BlockPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.xyz();
        write!(f, "BlockPoint({x}, {y}, {z})")
    }
}

// TODO: reevaluate if we need this thing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkPosition4(pub FSVector4);

impl ChunkPosition4 {
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(FSVector4(x, y, z, 0.0))
    }

    pub fn xyzw(&self) -> (f32, f32, f32, f32) {
        (self.0 .0, self.0 .1, self.0 .2, self.0 .3)
    }
}

impl Display for ChunkPosition4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z, w) = self.xyzw();
        write!(f, "ChunkPosition({}, {}, {}, {})", x, y, z, w)
    }
}

impl From<FSVector4> for ChunkPosition4 {
    fn from(value: FSVector4) -> Self {
        Self(value)
    }
}

/// Represents a havok AABB position. Often found where physics are involved.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HavokPosition(FSVector4);

impl HavokPosition {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self(FSVector4(x, y, z, 0.0))
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

impl Display for HavokPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.xyz();
        write!(f, "HavokPosition({}, {}, {})", x, y, z)
    }
}

impl From<FSVector4> for HavokPosition {
    fn from(value: FSVector4) -> Self {
        Self(value)
    }
}

impl Sub<HavokPosition> for HavokPosition {
    type Output = HavokPosition;

    fn sub(self, rhs: HavokPosition) -> Self::Output {
        HavokPosition(self.0 - rhs.0)
    }
}

/// Represents an orientation in radians.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion(FSVector4);

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.0 .0, self.0 .1, self.0 .2, self.0 .3
        )
    }
}

impl From<FSVector4> for Quaternion {
    fn from(value: FSVector4) -> Self {
        Self(value)
    }
}
