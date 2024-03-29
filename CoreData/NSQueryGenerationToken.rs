//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSQueryGenerationToken;

    unsafe impl ClassType for NSQueryGenerationToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSQueryGenerationToken {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSQueryGenerationToken {}

unsafe impl NSObjectProtocol for NSQueryGenerationToken {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSQueryGenerationToken {}

extern_methods!(
    unsafe impl NSQueryGenerationToken {
        #[method_id(@__retain_semantics Other currentQueryGenerationToken)]
        pub unsafe fn currentQueryGenerationToken() -> Id<NSQueryGenerationToken>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSQueryGenerationToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
