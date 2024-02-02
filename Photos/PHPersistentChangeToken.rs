//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHPersistentChangeToken")]
    pub struct PHPersistentChangeToken;

    #[cfg(feature = "Photos_PHPersistentChangeToken")]
    unsafe impl ClassType for PHPersistentChangeToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl Send for PHPersistentChangeToken {}

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl Sync for PHPersistentChangeToken {}

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl NSCoding for PHPersistentChangeToken {}

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl NSCopying for PHPersistentChangeToken {}

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl NSObjectProtocol for PHPersistentChangeToken {}

#[cfg(feature = "Photos_PHPersistentChangeToken")]
unsafe impl NSSecureCoding for PHPersistentChangeToken {}

extern_methods!(
    #[cfg(feature = "Photos_PHPersistentChangeToken")]
    unsafe impl PHPersistentChangeToken {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
