//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSAccessibilityColor;

    unsafe impl ProtocolType for NSAccessibilityColor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessibilityName)]
        pub unsafe fn accessibilityName(&self) -> Id<NSString, Shared>;
    }
);

extern_methods!(
    /// NSAccessibilityColorConformance
    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl NSColor {}
);