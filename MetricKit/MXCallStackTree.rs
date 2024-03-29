//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXCallStackTree;

    unsafe impl ClassType for MXCallStackTree {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MXCallStackTree {}

unsafe impl NSObjectProtocol for MXCallStackTree {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MXCallStackTree {}

extern_methods!(
    unsafe impl MXCallStackTree {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXCallStackTree {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
