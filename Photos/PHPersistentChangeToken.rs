//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPersistentChangeToken;

    unsafe impl ClassType for PHPersistentChangeToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for PHPersistentChangeToken {}

unsafe impl Sync for PHPersistentChangeToken {}

unsafe impl NSCoding for PHPersistentChangeToken {}

unsafe impl NSCopying for PHPersistentChangeToken {}

unsafe impl NSObjectProtocol for PHPersistentChangeToken {}

unsafe impl NSSecureCoding for PHPersistentChangeToken {}

extern_methods!(
    unsafe impl PHPersistentChangeToken {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
