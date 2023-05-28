//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCActivationLayer")]
    pub struct MLCActivationLayer;

    #[cfg(feature = "MLCompute_MLCActivationLayer")]
    unsafe impl ClassType for MLCActivationLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCActivationLayer")]
unsafe impl NSObjectProtocol for MLCActivationLayer {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCActivationLayer")]
    unsafe impl MLCActivationLayer {
        #[cfg(feature = "MLCompute_MLCActivationDescriptor")]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCActivationDescriptor>;

        #[cfg(feature = "MLCompute_MLCActivationDescriptor")]
        #[method_id(@__retain_semantics Other layerWithDescriptor:)]
        pub unsafe fn layerWithDescriptor(descriptor: &MLCActivationDescriptor) -> Id<Self>;

        #[method_id(@__retain_semantics Other reluLayer)]
        pub unsafe fn reluLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other relu6Layer)]
        pub unsafe fn relu6Layer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other leakyReLULayer)]
        pub unsafe fn leakyReLULayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other leakyReLULayerWithNegativeSlope:)]
        pub unsafe fn leakyReLULayerWithNegativeSlope(negative_slope: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other linearLayerWithScale:bias:)]
        pub unsafe fn linearLayerWithScale_bias(scale: c_float, bias: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other sigmoidLayer)]
        pub unsafe fn sigmoidLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other hardSigmoidLayer)]
        pub unsafe fn hardSigmoidLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other tanhLayer)]
        pub unsafe fn tanhLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other absoluteLayer)]
        pub unsafe fn absoluteLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other softPlusLayer)]
        pub unsafe fn softPlusLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other softPlusLayerWithBeta:)]
        pub unsafe fn softPlusLayerWithBeta(beta: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other softSignLayer)]
        pub unsafe fn softSignLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other eluLayer)]
        pub unsafe fn eluLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other eluLayerWithA:)]
        pub unsafe fn eluLayerWithA(a: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other relunLayerWithA:b:)]
        pub unsafe fn relunLayerWithA_b(a: c_float, b: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other logSigmoidLayer)]
        pub unsafe fn logSigmoidLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other seluLayer)]
        pub unsafe fn seluLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other celuLayer)]
        pub unsafe fn celuLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other celuLayerWithA:)]
        pub unsafe fn celuLayerWithA(a: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other hardShrinkLayer)]
        pub unsafe fn hardShrinkLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other hardShrinkLayerWithA:)]
        pub unsafe fn hardShrinkLayerWithA(a: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other softShrinkLayer)]
        pub unsafe fn softShrinkLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other softShrinkLayerWithA:)]
        pub unsafe fn softShrinkLayerWithA(a: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other tanhShrinkLayer)]
        pub unsafe fn tanhShrinkLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other thresholdLayerWithThreshold:replacement:)]
        pub unsafe fn thresholdLayerWithThreshold_replacement(
            threshold: c_float,
            replacement: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other geluLayer)]
        pub unsafe fn geluLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other hardSwishLayer)]
        pub unsafe fn hardSwishLayer() -> Id<MLCActivationLayer>;

        #[method_id(@__retain_semantics Other clampLayerWithMinValue:maxValue:)]
        pub unsafe fn clampLayerWithMinValue_maxValue(
            min_value: c_float,
            max_value: c_float,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCompute_MLCActivationLayer")]
    unsafe impl MLCActivationLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
