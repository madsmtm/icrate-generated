//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILNetworkResponse;

    unsafe impl ClassType for ILNetworkResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for ILNetworkResponse {}

unsafe impl NSObjectProtocol for ILNetworkResponse {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for ILNetworkResponse {}

extern_methods!(
    unsafe impl ILNetworkResponse {
        #[cfg(feature = "Foundation_NSURLResponse")]
        #[method_id(@__retain_semantics Other urlResponse)]
        pub unsafe fn urlResponse(&self) -> Id<NSHTTPURLResponse>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILNetworkResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
