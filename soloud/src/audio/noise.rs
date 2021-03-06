use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NoiseType {
    White = 0,
    Pink,
    Brownish,
    Blueish,
}

#[derive(AudioExt)]
pub struct Noise {
    _inner: *mut ffi::Noise,
}

impl Noise {
    /// Set noise type 
    pub fn set_type(&mut self, typ: NoiseType) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Noise_setType(self._inner, typ as i32) }
    }
}
