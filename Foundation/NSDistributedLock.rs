//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDistributedLock;

    unsafe impl ClassType for NSDistributedLock {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSDistributedLock {}

unsafe impl Sync for NSDistributedLock {}

unsafe impl NSObjectProtocol for NSDistributedLock {}

extern_methods!(
    unsafe impl NSDistributedLock {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lockWithPath:)]
        pub unsafe fn lockWithPath(path: &NSString) -> Option<Id<NSDistributedLock>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(this: Allocated<Self>, path: &NSString) -> Option<Id<Self>>;

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(breakLock)]
        pub unsafe fn breakLock(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lockDate)]
        pub unsafe fn lockDate(&self) -> Id<NSDate>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDistributedLock {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
