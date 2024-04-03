//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKNavigation;

    unsafe impl ClassType for WKNavigation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKNavigation {}

extern_methods!(
    unsafe impl WKNavigation {
        #[cfg(feature = "WebKit_WKWebpagePreferences")]
        #[method(effectiveContentMode)]
        pub unsafe fn effectiveContentMode(&self) -> WKContentMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKNavigation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
