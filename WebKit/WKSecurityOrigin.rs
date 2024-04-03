//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKSecurityOrigin;

    unsafe impl ClassType for WKSecurityOrigin {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKSecurityOrigin {}

extern_methods!(
    unsafe impl WKSecurityOrigin {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKSecurityOrigin {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
