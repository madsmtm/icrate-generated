//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCConvolutionLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCConvolutionLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCConvolutionLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCConvolutionLayer {
        #[cfg(feature = "MLCConvolutionDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCConvolutionDescriptor>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other weights)]
        pub unsafe fn weights(&self) -> Id<MLCTensor>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biases)]
        pub unsafe fn biases(&self) -> Option<Id<MLCTensor>>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other weightsParameter)]
        pub unsafe fn weightsParameter(&self) -> Id<MLCTensorParameter>;

        #[cfg(feature = "MLCTensorParameter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other biasesParameter)]
        pub unsafe fn biasesParameter(&self) -> Option<Id<MLCTensorParameter>>;

        #[cfg(all(feature = "MLCConvolutionDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithWeights:biases:descriptor:)]
        pub unsafe fn layerWithWeights_biases_descriptor(
            weights: &MLCTensor,
            biases: Option<&MLCTensor>,
            descriptor: &MLCConvolutionDescriptor,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCConvolutionLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
