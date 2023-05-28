//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCSoftmaxLayer")]
    pub struct MLCSoftmaxLayer;

    #[cfg(feature = "MLCompute_MLCSoftmaxLayer")]
    unsafe impl ClassType for MLCSoftmaxLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCSoftmaxLayer")]
unsafe impl NSObjectProtocol for MLCSoftmaxLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCSoftmaxLayer")]
    unsafe impl MLCSoftmaxLayer {
        #[method(operation)]
        pub unsafe fn operation(&self) -> MLCSoftmaxOperation;

        #[method(dimension)]
        pub unsafe fn dimension(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other layerWithOperation:)]
        pub unsafe fn layerWithOperation(operation: MLCSoftmaxOperation) -> Id<Self>;

        #[method_id(@__retain_semantics Other layerWithOperation:dimension:)]
        pub unsafe fn layerWithOperation_dimension(
            operation: MLCSoftmaxOperation,
            dimension: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCSoftmaxLayer")]
    unsafe impl MLCSoftmaxLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
