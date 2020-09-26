#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(windows))]
pub use nix::sys::socket::{sockaddr, sockaddr_in, sockaddr_storage, sockaddr_in6};
#[cfg(windows)]
pub use winapi::shared::{
    ws2def::SOCKADDR as sockaddr,
    ws2def::SOCKADDR_IN as sockaddr_in,
    ws2def::SOCKADDR_STORAGE as sockaddr_storage,
    ws2def::SOCKADDR_STORAGE_XP as sockaddr_storage_xp,
    ws2def::SOCKADDR_DL as sockaddr_dl,
    ws2ipdef::SOCKADDR_IN6_LH as sockaddr_in6,
};