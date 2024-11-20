use std::{
    cell::RefCell,
    error::Error,
    f32::consts::PI,
    mem::forget,
    ptr::NonNull,
    sync::{Arc, LazyLock, Mutex},
};

use game::{
    cs::{
        CSCamera, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, ChrIns, EnemyIns,
        WorldChrMan, WorldChrManDbg,
    }, fd4::FD4TaskData, position::ChunkPosition
};
use tracing_panic::panic_hook;
use util::{
    camera::CSCamExt,
    geometry::{CSWorldGeomManExt, GeometrySpawnParameters, SpawnGeometryError},
    input::is_key_pressed,
    singleton::get_instance,
    task::CSTaskImpExt,
    world_chr_man::WorldChrManExt,
};

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "enemy-control.log");
        tracing_subscriber::fmt().with_writer(appender).init();

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            std::thread::sleep(std::time::Duration::from_secs(5));
            init().expect("Could not initialize mod");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    let tracker = RefCell::new(EnemyControlTracker::default());
    let task = unsafe { get_instance::<CSTaskImp>() }.unwrap().unwrap();
    let task = task.run_recurring(
        move |_: &FD4TaskData| {
            is_key_pressed(0x68).then(|| {
                // Grab WorldChrMan
                let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
                    return;
                };

                // Get locked on enemy
                let Some(player) = world_chr_man.main_player.as_ref() else {
                    return;
                };

                // Bail if we're not locked onto an enemy
                if player.locked_on_enemy.is_empty() {
                    return;
                }

                // Retrieve targeted enemy from worldchrman
                let target_handle = player.locked_on_enemy.clone();
                let Some(targeted) = world_chr_man.chr_ins_by_handle(&target_handle) else {
                    return;
                };

                tracker.borrow_mut().attach(targeted);
            });

            is_key_pressed(0x62).then(|| {
                tracker.borrow_mut().detach();
            });
        },
        CSTaskGroupIndex::CameraStep,
    );
    forget(task);

    Ok(())
}

#[derive(Default)]
pub struct EnemyControlTracker {
    controlling: Option<ControlledEnemy>,
}

pub struct ControlledEnemy {
    original_manipulator: usize,
    chr_ins: SendPointer<EnemyIns>,
}

impl EnemyControlTracker {
    pub fn attach(&mut self, chr_ins: &mut ChrIns) {
        // Yolo it for now
        let chr_ins = unsafe { (chr_ins as *mut ChrIns as *mut EnemyIns).as_mut() }.unwrap();

        let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() else {
            return;
        };

        self.controlling = Some(ControlledEnemy {
            original_manipulator: chr_ins.com_manipulator,
            chr_ins: SendPointer(NonNull::new(chr_ins as *mut _).unwrap()),
        });

        chr_ins.com_manipulator = world_chr_man_dbg.debug_manipulator;
        world_chr_man_dbg.cam_override_chr_ins = NonNull::new(chr_ins as *mut EnemyIns as *mut _);
    }

    pub fn detach(&mut self) {
        let Some(controlling) = self.controlling.as_mut() else {
            return;
        };

        let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() else {
            return;
        };

        unsafe { controlling.chr_ins.0.as_mut() }.com_manipulator =
            controlling.original_manipulator;
        world_chr_man_dbg.cam_override_chr_ins = None;
        self.controlling = None;
    }
}

// FML lmao
struct SendPointer<T>(NonNull<T>);
unsafe impl<T> Send for SendPointer<T> {}
