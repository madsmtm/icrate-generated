//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILMessageFilterCapabilitiesQueryRequest;

    unsafe impl ClassType for ILMessageFilterCapabilitiesQueryRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for ILMessageFilterCapabilitiesQueryRequest {}

unsafe impl NSObjectProtocol for ILMessageFilterCapabilitiesQueryRequest {}

unsafe impl NSSecureCoding for ILMessageFilterCapabilitiesQueryRequest {}

extern_methods!(
    unsafe impl ILMessageFilterCapabilitiesQueryRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILMessageFilterCapabilitiesQueryRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
