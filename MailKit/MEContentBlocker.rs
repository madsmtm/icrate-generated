//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MEContentBlocker: NSObjectProtocol {
        #[method_id(@__retain_semantics Other contentRulesJSON)]
        unsafe fn contentRulesJSON(&self) -> Id<NSData>;
    }

    unsafe impl ProtocolType for dyn MEContentBlocker {}
);
