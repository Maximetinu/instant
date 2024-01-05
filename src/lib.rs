#![cfg_attr(not(feature = "std"), no_std)]

extern crate crankstart_sys;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod instant;
        mod crankstart;
        pub use crate::instant::Instant;
        pub use crate::crankstart::now;
    } else if #[cfg(any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        target_arch = "asmjs"
    ))] {
        #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
        #[macro_use]
        extern crate stdweb;

        mod instant;
        pub use crate::instant::Instant;
        mod wasm;
        pub use crate::wasm::now;
        pub use wasm::SystemTime;
    } else {
        mod native;
        pub use native::Instant;
        pub use native::now;
        pub use native::SystemTime;
    }
}

pub use core::time::Duration;
