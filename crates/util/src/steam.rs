use std::sync::OnceLock;

use steamworks::{Client, ClientManager};
use steamworks_sys::ISteamNetworkingMessages;
use vtable_rs::VPtr;

pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client<ClientManager>> = OnceLock::new();

    CLIENT.get_or_init(|| Client::init_app(1245620).unwrap().0)
}

#[allow(dead_code)]
unsafe fn steam_networking_messages() -> Option<*mut ISteamNetworkingMessages> {
    let result = steamworks_sys::SteamAPI_SteamNetworkingMessages_SteamAPI_v002();
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}

#[vtable_rs::vtable]
pub trait SteamCallbackVmt {
    fn run(&mut self, data: *const std::ffi::c_void);

    fn run_other(&mut self, data: *const std::ffi::c_void, p3: u64, p4: bool);

    fn get_callback_size_bytes(&mut self) -> u32;
}

#[repr(C)]
pub struct SteamCallback<D>
where
    D: Sized + 'static,
{
    vftable: VPtr<dyn SteamCallbackVmt, Self>,
    closure: Box<dyn FnMut(&D)>,
}

impl<D> SteamCallbackVmt for SteamCallback<D>
where
    D: Sized + 'static,
{
    extern "C" fn run(&mut self, data: *const std::ffi::c_void) {
        unsafe {
            (self.closure)(std::mem::transmute(data));
        }
    }

    extern "C" fn run_other(&mut self, data: *const std::ffi::c_void, _p3: u64, _p4: bool) {
        unsafe {
            (self.closure)(std::mem::transmute(data));
        }
    }

    extern "C" fn get_callback_size_bytes(&mut self) -> u32 {
        std::mem::size_of::<D>() as u32
    }
}

impl<F, D> From<F> for SteamCallback<D>
where
    F: FnMut(&D) + 'static + Send,
    D: Sized + 'static,
{
    fn from(value: F) -> Self {
        Self {
            vftable: Default::default(),
            closure: Box::new(value),
        }
    }
}

pub fn register_callback<D, F>(callback: i32, f: F)
where
    D: Sized + 'static,
    F: FnMut(&D) + 'static + Send,
{
    let callback_fn: &mut SteamCallback<D> = {
        let tmp: SteamCallback<D> = f.into();
        Box::leak(Box::new(tmp))
    };

    unsafe {
        steamworks_sys::SteamAPI_RegisterCallback(
            callback_fn as *mut SteamCallback<D> as _,
            callback,
        );
    }
}

pub fn networking_messages() -> Option<*mut ISteamNetworkingMessages> {
    let result = unsafe { steamworks_sys::SteamAPI_SteamNetworkingMessages_SteamAPI_v002() };
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}
