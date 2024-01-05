use core::{ptr, time::Duration};

use crankstart_sys::{playdate_sys, PlaydateAPI};

use crate::Instant;

static mut SYSTEM: System = System(ptr::null_mut());

#[derive(Clone, Debug)]
pub struct System(*const crankstart_sys::playdate_sys);

impl System {
    pub(crate) fn register(system: *const crankstart_sys::playdate_sys) {
        unsafe {
            SYSTEM = Self(system);
        }
    }
}

impl Instant {
    #[inline]
    pub fn now() -> Self {
        Instant::from_duration(Duration::from_millis(now_u64()))
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    pub fn register(playdate: *const crankstart_sys::PlaydateAPI) {
        let playdate_api = unsafe { *playdate };
        System::register(playdate_api.system);
    }
}

/// The current time, expressed in milliseconds since the Unix Epoch.
pub fn now() -> f64 {
    now_u64() as f64
}

/// The current time, expressed in milliseconds since the Unix Epoch.
pub fn now_u64() -> u64 {
    unsafe {
        let sys: *const playdate_sys = SYSTEM.0;
        if sys.is_null() {
            return 1;
        }
        if let Some(get_now) = (*sys).getCurrentTimeMilliseconds {
            return get_now() as u64;
        }
    }
    2
}
