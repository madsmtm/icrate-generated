//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCLossLayer;

    #[cfg(feature = "MLCLayer")]
    unsafe impl ClassType for MLCLossLayer {
        #[inherits(NSObject)]
        type Super = MLCLayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCLossLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLossLayer {
        #[cfg(feature = "MLCLossDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Id<MLCLossDescriptor>;

        #[cfg(feature = "MLCTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other weights)]
        pub unsafe fn weights(&self) -> Option<Id<MLCTensor>>;

        #[cfg(feature = "MLCLossDescriptor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:)]
        pub unsafe fn layerWithDescriptor(loss_descriptor: &MLCLossDescriptor) -> Id<Self>;

        #[cfg(all(feature = "MLCLossDescriptor", feature = "MLCTensor"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other layerWithDescriptor:weights:)]
        pub unsafe fn layerWithDescriptor_weights(
            loss_descriptor: &MLCLossDescriptor,
            weights: &MLCTensor,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weight:)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weights:)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other logLossWithReductionType:epsilon:weight:)]
        pub unsafe fn logLossWithReductionType_epsilon_weight(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other logLossWithReductionType:epsilon:weights:)]
        pub unsafe fn logLossWithReductionType_epsilon_weights(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other huberLossWithReductionType:delta:weight:)]
        pub unsafe fn huberLossWithReductionType_delta_weight(
            reduction_type: MLCReductionType,
            delta: c_float,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other huberLossWithReductionType:delta:weights:)]
        pub unsafe fn huberLossWithReductionType_delta_weights(
            reduction_type: MLCReductionType,
            delta: c_float,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanAbsoluteErrorLossWithReductionType:weight:)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanAbsoluteErrorLossWithReductionType:weights:)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanSquaredErrorLossWithReductionType:weight:)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other meanSquaredErrorLossWithReductionType:weights:)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other hingeLossWithReductionType:weight:)]
        pub unsafe fn hingeLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other hingeLossWithReductionType:weights:)]
        pub unsafe fn hingeLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cosineDistanceLossWithReductionType:weight:)]
        pub unsafe fn cosineDistanceLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Id<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other cosineDistanceLossWithReductionType:weights:)]
        pub unsafe fn cosineDistanceLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCLossLayer {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
