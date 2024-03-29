//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKServerChangeToken;

    unsafe impl ClassType for CKServerChangeToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKServerChangeToken {}

unsafe impl Sync for CKServerChangeToken {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CKServerChangeToken {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CKServerChangeToken {}

unsafe impl NSObjectProtocol for CKServerChangeToken {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CKServerChangeToken {}

extern_methods!(
    unsafe impl CKServerChangeToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
