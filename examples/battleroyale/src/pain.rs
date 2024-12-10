use std::{sync::{
    Arc, Mutex
}, time::{Duration, Instant}};

use game::cs::{PlayerIns, WorldChrMan};
use util::singleton::get_instance;

use crate::{
    mapdata::{MapPoint, MAP_CONFIG},
    ProgramLocationProvider, LOCATION_APPLY_SPEFFECT,
};

pub struct PainRing<L>
where
    L: ProgramLocationProvider,
{
    center: Mutex<Option<MapPoint>>,
    last_applied_hurt: Mutex<Instant>,
    location: Arc<L>,
}

impl<L> PainRing<L>
where
    L: ProgramLocationProvider,
{
    pub fn new(location: Arc<L>) -> Self {
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
                let point = MAP_CONFIG[0]
                    .ring_centers
                    .first()
                    .expect("No suitable ring center found for match.")
                    .clone();

                self.spawn_center_marker(&point);

                *center = Some(point);
            }
        }

        {
            // Kind of naive but :shrug:
            let mut last_applied_hurt = self.last_applied_hurt.lock().unwrap();
            if Instant::now().duration_since(*last_applied_hurt) > Duration::from_millis(75) {
                *last_applied_hurt = Instant::now();
                self.hurt_player();
            }
        }
    }

    /// Runs cleanup at end of match.
    pub fn reset(&self) {
        *self.center.lock().unwrap() = None;
    }

    fn spawn_center_marker(&self, point: &MapPoint) {
        tracing::info!("Spawning FFX at {point:?}");
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
