use std::{mem::size_of, ops::DerefMut, os::raw, pin::Pin, sync::OnceLock};

use steamworks::{Client, ClientManager};
use steamworks_sys::{uint8, ISteamNetworkingMessages};

pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client<ClientManager>> = OnceLock::new();

    CLIENT.get_or_init(|| Client::init_app(1245620).unwrap().0)
}

unsafe fn steam_networking_messages() -> Option<*mut ISteamNetworkingMessages> {
    let result = steamworks_sys::SteamAPI_SteamNetworkingMessages_SteamAPI_v002();
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}

#[repr(C)]
struct SteamCallbackVMT<C: SteamCallbackImpl> {
    pub run: fn(usize, *const C::TData),
    pub run_other: fn(usize, *const C::TData, bool, u64),
    pub get_callback_size_bytes: fn(usize) -> u32,
}

#[repr(C)]
#[allow(non_snake_case)]
struct SteamCallback<C: SteamCallbackImpl> {
    pub vfptr: *const SteamCallbackVMT<C>,
    pub m_nCallbackFlags: uint8,
    pub m_iCallback: raw::c_int,
}

pub trait SteamCallbackImpl {
    type TData: Sized;
    const CALLBACK: i32;

    fn run(data: *const Self::TData);
}

pub struct SteamCallbackHandle<T: SteamCallbackImpl> {
    vmt: Pin<Box<SteamCallbackVMT<T>>>,
    callback: Pin<Box<SteamCallback<T>>>,
}

impl<T: SteamCallbackImpl> Drop for SteamCallbackHandle<T> {
    fn drop(&mut self) {
        todo!()
    }
}

pub fn register_callback<T: SteamCallbackImpl>() -> SteamCallbackHandle<T> {
    let vmt = Box::pin(SteamCallbackVMT {
        run: |_, data| T::run(data),
        run_other: |_, data, _, _| T::run(data),
        get_callback_size_bytes: |_| size_of::<T::TData>().try_into().unwrap(),
    });

    let mut callback = Box::pin(SteamCallback {
        vfptr: vmt.as_ref().get_ref() as *const _,
        m_nCallbackFlags: 0,
        m_iCallback: 0,
    });

    unsafe {
        steamworks_sys::SteamAPI_RegisterCallback(
            callback.as_mut().deref_mut() as *mut SteamCallback<_> as *mut _,
            T::CALLBACK,
        );
    }

    SteamCallbackHandle {
        vmt,
        callback,
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
