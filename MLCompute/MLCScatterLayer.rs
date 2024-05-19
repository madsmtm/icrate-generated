//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCScatterLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCScatterLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCScatterLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCScatterLayer {
        #[deprecated]
        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MLCReductionType;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDimension:reductionType:)]
        pub unsafe fn layerWithDimension_reductionType(
            dimension: NSUInteger,
            reduction_type: MLCReductionType,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCScatterLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
