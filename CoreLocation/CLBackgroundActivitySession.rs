//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLBackgroundActivitySession;

    unsafe impl ClassType for CLBackgroundActivitySession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CLBackgroundActivitySession {}

extern_methods!(
    unsafe impl CLBackgroundActivitySession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method_id(@__retain_semantics Other backgroundActivitySession)]
        pub unsafe fn backgroundActivitySession() -> Id<Self>;
    }
);
