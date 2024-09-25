use std::fs::File;
use std::io::Write;

use broadsword::dll;
use tracing::Level;
use tracing_panic::panic_hook;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use util::program::Program;
use util::singleton::build_singleton_table;

#[dll::entrypoint]
pub fn entry(_hmodule: usize) -> bool {
    std::panic::set_hook(Box::new(panic_hook));

    let appender = tracing_appender::rolling::never("./", "export-singletons.log");
    tracing_subscriber::fmt().with_writer(appender).init();
    tracing::debug!("Setup logging");

    let mut fh = File::create("singleton.csv")
        .expect("Could not create export file");
    let program = unsafe { Program::current() };

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

    true
}
