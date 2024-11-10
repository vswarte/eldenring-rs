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

use crate::matrix::FSVector4;

/// Represents a chunk local position. Found all over the code usually in together with a
/// map ID.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkPosition(FSVector4);

impl ChunkPosition {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self(FSVector4(x, y, z, 0.0))
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.xyz();
        write!(f, "ChunkPos({}, {}, {})", x, y, z)
    }
}

impl From<FSVector4> for ChunkPosition {
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
        write!(f, "ChunkPos({}, {}, {})", x, y, z)
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
