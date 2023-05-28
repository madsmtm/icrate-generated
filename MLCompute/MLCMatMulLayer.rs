//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCMatMulLayer")]
    pub struct MLCMatMulLayer;

    #[cfg(feature = "MLCompute_MLCMatMulLayer")]
    unsafe impl ClassType for MLCMatMulLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCMatMulLayer")]
unsafe impl NSObjectProtocol for MLCMatMulLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCMatMulLayer")]
    unsafe impl MLCMatMulLayer {
        #[cfg(feature = "MLCompute_MLCMatMulDescriptor")]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCMatMulDescriptor>;

        #[cfg(feature = "MLCompute_MLCMatMulDescriptor")]
        #[method_id(@__retain_semantics Other layerWithDescriptor:)]
        pub unsafe fn layerWithDescriptor(descriptor: &MLCMatMulDescriptor) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCMatMulLayer")]
    unsafe impl MLCMatMulLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
