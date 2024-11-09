use std::fs::File;
use std::io::Write;

use tracing_panic::panic_hook;
use util::program::Program;
use util::singleton::build_singleton_table;

#[no_mangle]
pub unsafe extern "C" fn DllMain(_base: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "export-singletons.log");
        tracing_subscriber::fmt().with_writer(appender).init();
        tracing::debug!("Setup logging");

        let mut fh = File::create("singleton.csv")
            .expect("Could not create export file");
        let program = unsafe { Program::current() };

        // Give the game a bit of time to populate up the DLRF structures
        // before dumping the statics.
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(5));

            let table = build_singleton_table(&program);
            tracing::debug!("Table result: {table:#x?}");

            for entry in table.expect("Could not find singleton table").iter() {
                writeln!(fh, "\"{}\", {:x}", entry.0, entry.1)
                    .expect("Could not write to export file");
            }
            tracing::debug!("Wrote table to CSV");
        });
    }

    true
}
