//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CIPlugInRegistration {
        #[method(load:)]
        unsafe fn load(&self, host: *mut c_void) -> bool;
    }

    unsafe impl ProtocolType for dyn CIPlugInRegistration {}
);
