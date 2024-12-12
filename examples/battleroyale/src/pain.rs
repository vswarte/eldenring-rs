use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use game::cs::{CSWorldGeomMan, PlayerIns, WorldChrMan};
use nalgebra::Vector3;
use util::{
    geometry::{CSWorldGeomManExt, GeometrySpawnParameters},
    singleton::get_instance,
};

use crate::{
    mapdata::{self, MapPoint},
    ProgramLocationProvider, LOCATION_APPLY_SPEFFECT,
};

pub struct PainRing {
    center: Mutex<Option<MapPoint>>,
    last_applied_hurt: Mutex<Instant>,
    location: Arc<ProgramLocationProvider>,
}

impl PainRing {
    pub fn new(location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            center: Default::default(),
            last_applied_hurt: Mutex::new(Instant::now()),
            location,
        }
    }

    pub fn update(&self) {
        let mut center = self.center.lock().unwrap();

        {
            // Set center position first time around
            if center.as_ref().is_none() {
                let point = mapdata::get(0)
                    .unwrap()
                    .ring_centers
                    .first()
                    .expect("No suitable ring center found for match.")
                    .clone();

                self.spawn_center_marker(&point);
                *center = Some(point);
            }
        }

        {
            // Check if we're outside the circle and apply damage to player.
            if let Some(main_player) = &mut unsafe { get_instance::<WorldChrMan>() }
                .unwrap()
                .map(|w| w.main_player.as_ref())
                .flatten() {
                let player_pos: Vector3<f32> = main_player.block_position.into();
                let center_pos: Vector3<f32> = center.as_ref().unwrap().position.into();

                // Hurt player if they're outside of the circles radius.
                if player_pos.metric_distance(&center_pos) > 1000.0 {
                    // Kind of naive but :shrug:
                    let mut last_applied_hurt = self.last_applied_hurt.lock().unwrap();
                    if Instant::now().duration_since(*last_applied_hurt) > Duration::from_millis(75)
                    {
                        *last_applied_hurt = Instant::now();
                        self.hurt_player();
                    }
                }
            }
        }
    }

    /// Runs cleanup at end of match.
    pub fn reset(&self) {
        *self.center.lock().unwrap() = None;
    }

    fn spawn_center_marker(&self, point: &MapPoint) {
        tracing::info!("Spawning center marker at {point:?}");
        let world_geom_man = unsafe { get_instance::<CSWorldGeomMan>() }
            .unwrap()
            .unwrap();

        world_geom_man.spawn_geometry(
            "AEG099_620",
            &GeometrySpawnParameters {
                map_id: point.map,
                position: point.position,
                rot_x: 0.0,
                rot_y: 90.0,
                rot_z: 90.0,
                scale_x: 4.0,
                scale_y: 4.0,
                scale_z: 4.0,
            },
        ).unwrap();
    }

    fn hurt_player(&self) {
        let main_player = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .unwrap()
            .main_player
            .as_ref()
            .unwrap();

        let location_apply_speffect = self.location.get(LOCATION_APPLY_SPEFFECT).unwrap();
        let apply_speffect: extern "C" fn(&PlayerIns, u32, bool) =
            unsafe { std::mem::transmute(location_apply_speffect) };
        apply_speffect(main_player.as_ref(), 4004, false);
    }
}
