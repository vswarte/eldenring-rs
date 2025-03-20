use std::{
    f32::consts::PI,
    sync::Arc,
    time::{Duration, Instant},
};

use pelite::pe::Pe;

use game::{
    cs::{CSHavokMan, CSPhysWorld, CSWorldGeomMan, FieldArea, PlayerIns, WorldChrMan},
    matrix::FSVector4,
    position::HavokPosition,
    rva::RVA_GLOBAL_FIELD_AREA,
};
use nalgebra::Vector3;
use util::{
    geometry::{CSWorldGeomManExt, GeometrySpawnParameters},
    program::Program,
    singleton::get_instance,
};

use crate::{
    config::{ConfigurationProvider, RingCenterPoint},
    rva::{RVA_APPLY_SPEFFECT, RVA_SFX_SPAWN, RVA_WORLD_CAST_RAY},
    ProgramLocationProvider,
};

pub struct PainRing {
    location: Arc<ProgramLocationProvider>,
    config: Arc<ConfigurationProvider>,

    center: Option<RingCenterPoint>,

    last_applied_hurt: Instant,
    radius: f32,
}

impl PainRing {
    pub fn new(location: Arc<ProgramLocationProvider>, config: Arc<ConfigurationProvider>) -> Self {
        Self {
            location,
            config,
            center: Default::default(),
            last_applied_hurt: Instant::now(),
            radius: 100.0,
        }
    }

    pub fn update(&mut self) {
        {
            // Set center position first time around
            if self.center.as_ref().is_none() {
                let point = self
                    .config
                    .map(&0)
                    .unwrap()
                    .ring_centers
                    .first()
                    .expect("No ring centers defined for this map")
                    .clone();

                // self.spawn_center_marker(&point);
                self.center = Some(point);
            }
        }

        // TODO: rewrite hurt routine using world coords instead of block coords
        //
        // {
        //     // Check if we're outside the circle and apply damage to player.
        //     if let Some(main_player) = &mut unsafe { get_instance::<WorldChrMan>() }
        //         .unwrap()
        //         .map(|w| w.main_player.as_ref())
        //         .flatten()
        //     {
        //         let player_pos: Vector3<f32> = main_player.block_position.into();
        //         let center_pos: Vector3<f32> = {
        //             let tmp = &self.center.as_ref().unwrap().position;
        //
        //             Vector3::new(tmp.0, tmp.1, tmp.2)
        //         };
        //
        //         // Hurt player if they're outside of the circles radius.
        //         if player_pos.metric_distance(&center_pos).abs() > self.radius {
        //             // Kind of naive since player might suffer terribly low FPS but :shrug:
        //             if Instant::now().duration_since(self.last_applied_hurt)
        //                 > Duration::from_millis(75)
        //             {
        //                 let distance = player_pos.metric_distance(&center_pos);
        //                 tracing::info!("Hurting player. distance = {distance}");
        //                 self.last_applied_hurt = Instant::now();
        //                 self.hurt_player();
        //             }
        //         }
        //     }
        // }
    }

    /// Runs cleanup at end of match.
    pub fn reset(&mut self) {
        self.center = None;
    }

    // TODO: Dissolve unnecessary sfx when they're too tightly packed.
    pub fn spawn_center_marker(&self, center: &RingCenterPoint) {
        let program = unsafe { Program::current() };

        let field_area = unsafe {
            (*(program.rva_to_va(RVA_GLOBAL_FIELD_AREA).unwrap() as *const *const FieldArea))
                .as_ref()
        }
        .unwrap();

        let spawn_sfx: fn(&u32, &SfxSpawnLocation) -> bool =
            unsafe { std::mem::transmute(self.location.get(RVA_SFX_SPAWN).unwrap()) };

        let cast_ray: extern "C" fn(
            *const CSPhysWorld,
            u32,
            *const FSVector4,
            *const FSVector4,
            *const FSVector4,
            *const PlayerIns,
        ) -> bool = unsafe { std::mem::transmute(self.location.get(RVA_WORLD_CAST_RAY).unwrap()) };

        let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
        if let Some(main_player) = &world_chr_man.main_player {
            // Since the raycast and sfx spawn are all in physics space we need to find the
            // WorldBlockInfo for this block to obtain the center of the block in said physics
            // space.
            let block_center_physics_pos = field_area
                .world_info_owner
                .world_res
                .world_info
                .world_block_info_by_map(&(&center.map).into())
                .map(|b| b.physics_center)
                .unwrap();

            let center_havok_pos = HavokPosition::from_xyz(
                block_center_physics_pos.0 .0 + center.position.0,
                block_center_physics_pos.0 .1 + center.position.1,
                block_center_physics_pos.0 .2 + center.position.2,
            );

            let count = 128;
            for i in 0..count {
                // Form a circle around the center.
                let tau = PI * 2.0;
                let current = (tau / count as f32) * i as f32;
                // Offset for a point from the center
                let offset = HavokPosition::from_xyz(
                    f32::sin(current) * self.radius,
                    0.0,
                    f32::cos(current) * self.radius,
                );

                // Find the proper height to place the fog at by raycasting from above.
                let cast_origin = center_havok_pos + offset;
                let phys_world = unsafe { get_instance::<CSHavokMan>() }
                    .unwrap()
                    .unwrap()
                    .phys_world
                    .as_ptr();

                let mut collision = FSVector4(0.0, 0.0, 0.0, 0.0);
                if cast_ray(
                    phys_world,
                    0x2000058,
                    &cast_origin.0,
                    &FSVector4(0.0, -100.0, 0.0, 0.0),
                    &mut collision,
                    main_player.as_ptr(),
                ) {
                    let spawn_location = SfxSpawnLocation {
                        angle: (
                            FSVector4(0.7882865667, -0.007318737917, 0.6165360808, 0.0),
                            FSVector4(0.06933222711, 0.9946286082, -0.07685082406, 0.0),
                            FSVector4(-0.6126625538, 0.1033189669, 0.784560442, 0.0),
                        ),
                        position: HavokPosition(collision),
                    };

                    // 523357 - Fia's Mist
                    // 523573 - Darkness clouds
                    // 523887 - Freezing Mist
                    spawn_sfx(&523357, &spawn_location);
                }
            }
        };
    }

    fn hurt_player(&self) {
        let main_player = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .unwrap()
            .main_player
            .as_ref()
            .unwrap();

        let location_apply_speffect = self.location.get(RVA_APPLY_SPEFFECT).unwrap();
        let apply_speffect: extern "C" fn(&PlayerIns, u32, bool) =
            unsafe { std::mem::transmute(location_apply_speffect) };

        apply_speffect(main_player.as_ref(), 4004, false);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SfxSpawnLocation {
    pub angle: (FSVector4, FSVector4, FSVector4),
    pub position: HavokPosition,
}
