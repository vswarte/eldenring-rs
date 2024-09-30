use std::thread::sleep;
/// Implements a minimal armor visual-appearance changer.
use std::{error::Error, thread::spawn};
use std::time::Duration;
use tracing_panic::panic_hook;

use game::cs::{
    CSCamera, CSTaskGroupIndex, CSTaskImp, WorldChrMan, CHR_ASM_SLOT_ACCESSORY_1,
    CHR_ASM_SLOT_ACCESSORY_COVENANT, CHR_ASM_SLOT_WEAPON_LEFT_1,
};
use util::{singleton::get_instance, task::TaskRuntime};

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "mini-transmog.log");
        tracing_subscriber::fmt().with_writer(appender).init();

        spawn::<_, _>(|| {
            // Give the CRT init a bit of leeway
            sleep(Duration::from_secs(5));

            init().expect("Could not initialize mod");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    let task = get_instance::<CSTaskImp>().unwrap().unwrap();
    std::mem::forget(task.run_task(
        |_, _| {
            if let Some(mut main_player) = get_instance::<WorldChrMan>()
                .unwrap()
                .map(|w| unsafe { w.main_player.as_mut() })
                .flatten() {

                // Override weapon left 1 with Zweihander
                // main_player.chr_asm.equipment_param_ids[CHR_ASM_SLOT_WEAPON_LEFT_1] = 4040000; 
            }
        },
        CSTaskGroupIndex::WorldChrMan_Update_BackreadRequestPost,
    ));

    Ok(())
}
