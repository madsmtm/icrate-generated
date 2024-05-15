//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLImageSizeConstraint;

    unsafe impl ClassType for MLImageSizeConstraint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MLImageSizeConstraint {}

unsafe impl NSObjectProtocol for MLImageSizeConstraint {}

unsafe impl NSSecureCoding for MLImageSizeConstraint {}

extern_methods!(
    unsafe impl MLImageSizeConstraint {
        #[cfg(feature = "MLImageSizeConstraintType")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MLImageSizeConstraintType;

        #[method(pixelsWideRange)]
        pub unsafe fn pixelsWideRange(&self) -> NSRange;

        #[method(pixelsHighRange)]
        pub unsafe fn pixelsHighRange(&self) -> NSRange;

        #[cfg(feature = "MLImageSize")]
        #[method_id(@__retain_semantics Other enumeratedImageSizes)]
        pub unsafe fn enumeratedImageSizes(&self) -> Id<NSArray<MLImageSize>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLImageSizeConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
