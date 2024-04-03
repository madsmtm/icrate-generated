//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHObject;

    unsafe impl ClassType for PHObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for PHObject {}

unsafe impl Sync for PHObject {}

unsafe impl NSCopying for PHObject {}

unsafe impl NSObjectProtocol for PHObject {}

extern_methods!(
    unsafe impl PHObject {
        #[method_id(@__retain_semantics Other localIdentifier)]
        pub unsafe fn localIdentifier(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHObjectPlaceholder;

    unsafe impl ClassType for PHObjectPlaceholder {
        #[inherits(NSObject)]
        type Super = PHObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for PHObjectPlaceholder {}

unsafe impl Sync for PHObjectPlaceholder {}

unsafe impl NSCopying for PHObjectPlaceholder {}

unsafe impl NSObjectProtocol for PHObjectPlaceholder {}

extern_methods!(
    unsafe impl PHObjectPlaceholder {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHObjectPlaceholder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
