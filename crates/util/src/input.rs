use std::sync::Mutex;
use std::time;
use std::sync;
use std::collections;
use std::collections::hash_map::Entry;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(250);

type DebounceMap = collections::HashMap<i32, time::Instant>;
static DEBOUNCE_MAP: sync::LazyLock<Mutex<DebounceMap>> = sync::LazyLock::new(Default::default);

pub fn is_key_pressed(key: i32) -> bool {
    if unsafe { KeyboardAndMouse::GetKeyState(key) } < 0 {
        let now = std::time::Instant::now();

        match DEBOUNCE_MAP.lock().unwrap().entry(key) {
            Entry::Occupied(mut o) => {
                if o.get().elapsed() > DEBOUNCE_TIMEOUT {
                    o.insert(now);
                    return true
                } else {
                    return false
                }
            },
            collections::hash_map::Entry::Vacant(v) => {
                v.insert(now);
                return true
            },
        }
    }

    false
}
