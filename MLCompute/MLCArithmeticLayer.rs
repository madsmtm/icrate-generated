//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCArithmeticLayer")]
    pub struct MLCArithmeticLayer;

    #[cfg(feature = "MLCompute_MLCArithmeticLayer")]
    unsafe impl ClassType for MLCArithmeticLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCArithmeticLayer")]
unsafe impl NSObjectProtocol for MLCArithmeticLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCArithmeticLayer")]
    unsafe impl MLCArithmeticLayer {
        #[method(operation)]
        pub unsafe fn operation(&self) -> MLCArithmeticOperation;

        #[method_id(@__retain_semantics Other layerWithOperation:)]
        pub unsafe fn layerWithOperation(operation: MLCArithmeticOperation) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCArithmeticLayer")]
    unsafe impl MLCArithmeticLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
