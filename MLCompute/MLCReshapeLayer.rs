//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCReshapeLayer")]
    pub struct MLCReshapeLayer;

    #[cfg(feature = "MLCompute_MLCReshapeLayer")]
    unsafe impl ClassType for MLCReshapeLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCReshapeLayer")]
unsafe impl NSObjectProtocol for MLCReshapeLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCReshapeLayer")]
    unsafe impl MLCReshapeLayer {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other layerWithShape:)]
        pub unsafe fn layerWithShape(shape: &NSArray<NSNumber>) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCReshapeLayer")]
    unsafe impl MLCReshapeLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
