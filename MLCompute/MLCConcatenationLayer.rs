//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCConcatenationLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCConcatenationLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCConcatenationLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCConcatenationLayer {
        #[deprecated]
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDimension:)]
        pub unsafe fn layerWithDimension(dimension: NSUInteger) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCConcatenationLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
