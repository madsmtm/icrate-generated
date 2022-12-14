//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDistributedLock;

    unsafe impl ClassType for NSDistributedLock {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSDistributedLock {
        #[method_id(@__retain_semantics Other lockWithPath:)]
        pub unsafe fn lockWithPath(path: &NSString) -> Option<Id<NSDistributedLock, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(breakLock)]
        pub unsafe fn breakLock(&self);

        #[method_id(@__retain_semantics Other lockDate)]
        pub unsafe fn lockDate(&self) -> Id<NSDate, Shared>;
    }
);
