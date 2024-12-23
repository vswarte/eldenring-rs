use std::{
    f32::consts::PI,
    sync::Arc,
    time::{Duration, Instant},
};

use game::{
    cs::{CSWorldGeomMan, PlayerIns, WorldChrMan},
    matrix::FSVector4,
    position::HavokPosition,
};
use nalgebra::Vector3;
use util::{
    geometry::{CSWorldGeomManExt, GeometrySpawnParameters},
    singleton::get_instance,
};

use crate::{
    config::{ConfigurationProvider, MapPoint}, rva::{RVA_APPLY_SPEFFECT, RVA_SFX_SPAWN}, ProgramLocationProvider
};

pub struct PainRing {
    location: Arc<ProgramLocationProvider>,
    config: Arc<ConfigurationProvider>,

    center: Option<MapPoint>,

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

                self.spawn_center_marker(&point);
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
    fn spawn_center_marker(&self, center: &MapPoint) {
        tracing::info!("Spawning center marker at {center:?}");
        // let world_geom_man = unsafe { get_instance::<CSWorldGeomMan>() }
        //     .unwrap()
        //     .unwrap();
        //
        // // Put down the center asset
        // world_geom_man.spawn_geometry(
        //     "AEG099_620",
        //     &GeometrySpawnParameters {
        //         map_id: center.map,
        //         position: center.position,
        //         rot_x: 0.0,
        //         rot_y: 0.0,
        //         rot_z: 0.0,
        //         scale_x: 4.0,
        //         scale_y: 4.0,
        //         scale_z: 4.0,
        //     },
        // ).unwrap();

        // Put down the ring
        let fxr_id = 523573;
        // Angle the fucking thing upwards
        let angle = (
            FSVector4(0.7882865667, -0.007318737917, 0.6165360808, 0.0),
            FSVector4(0.06933222711, 0.9946286082, -0.07685082406, 0.0),
            FSVector4(-0.6126625538, 0.1033189669, 0.784560442, 0.0),
        );

        let spawn_sfx: fn(&u32, &SfxSpawnLocation) -> bool =
            unsafe { std::mem::transmute(self.location.get(RVA_SFX_SPAWN).unwrap()) };

        // let cast_ray: extern "C" fn(&*mut CSPhysWorld, u32, &HavokPosition, &FSVector4, &mut HavokPosition, &ChrIns) =
        //     unsafe { std::mem::transmute(self.location.get(RVA_PHYS_WORLD_CAST_RAY).unwrap()) };

        let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
        if let Some(main_player) = &world_chr_man.main_player {
            // Since the sfx spawn + later raycast is in havok space we need to figure out the
            // centers position inside the AABB. This is luckily possible because both havok and FS
            // use meters and 1 meter represents the same length in both systems.
            let player_havok_pos = &main_player.chr_ins.module_container.physics.position;
            let player_block_pos = &main_player.block_position;

            // Grab the delta between players block pos and centers block pos.
            let delta = HavokPosition::from_xyz(
                center.position.0 - player_block_pos.0 .0,
                center.position.1 - player_block_pos.0 .1,
                center.position.2 - player_block_pos.0 .2,
            );

            // Determine the center's position in current AABB using the players physics pos as a
            // reference.
            let center_havok_pos = *player_havok_pos + delta;
            tracing::info!("Center {center_havok_pos}");

            let count = 64;
            for i in 0..count {
                let tau = PI * 2.0;
                let current = (tau / count as f32) * i as f32;
                // Offset for a point from the center
                let offset = HavokPosition::from_xyz(
                    f32::sin(current) * self.radius,
                    0.0,
                    f32::cos(current) * self.radius,
                );

                // let mut raycast_out = HavokPosition::from_xyz(0.0, 0.0, 0.0);
                // let phys_world = unsafe { get_instance::<CSHavokMan>() }.unwrap().unwrap().phys_world.as_ptr();

                // cast_ray(
                //     &phys_world,
                //     0x2000058,
                //     &cast_origin,
                //     &FSVector4(0.0, 0.0, -1000.0, 0.0), // Aim the fuck down
                //     &mut raycast_out,
                //     &main_player.chr_ins,
                // );

                // let spawn_position = if raycast_out.xyz().0 != 0.0 || raycast_out.xyz().2 != 0.0 {
                //     raycast_out
                // } else {
                //     cast_origin
                // };

                let spawn_location = SfxSpawnLocation {
                    angle,
                    position: center_havok_pos + offset,
                };

                spawn_sfx(&fxr_id, &spawn_location);
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
