use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct WavStream {
    _inner: *mut ffi::WavStream,
}

impl WavStream {
    pub fn length(&mut self) -> f64 {
        assert!(!self._inner.is_null());
        unsafe { ffi::WavStream_getLength(self._inner) }
    }
}
