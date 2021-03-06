use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BassBoostFilterAttr {
    Wet = 0,
    Boost = 1,
}

#[derive(FilterExt)]
pub struct BassboostFilter {
    _inner: *mut soloud_sys::soloud::BassboostFilter,
}

impl BassboostFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = soloud_sys::soloud::BassboostFilter_setParams(self._inner, delay);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
