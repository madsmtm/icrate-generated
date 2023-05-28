//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MLCompute::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
    pub struct MLCConvolutionDescriptor;

    #[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
    unsafe impl ClassType for MLCConvolutionDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
unsafe impl NSCopying for MLCConvolutionDescriptor {}

#[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
unsafe impl NSObjectProtocol for MLCConvolutionDescriptor {}

extern_methods!(
    #[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
    unsafe impl MLCConvolutionDescriptor {
        #[method(convolutionType)]
        pub unsafe fn convolutionType(&self) -> MLCConvolutionType;

        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        #[method(inputFeatureChannelCount)]
        pub unsafe fn inputFeatureChannelCount(&self) -> NSUInteger;

        #[method(outputFeatureChannelCount)]
        pub unsafe fn outputFeatureChannelCount(&self) -> NSUInteger;

        #[method(strideInX)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        #[method(strideInY)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        #[method(dilationRateInX)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        #[method(dilationRateInY)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        #[method(groupCount)]
        pub unsafe fn groupCount(&self) -> NSUInteger;

        #[method(paddingPolicy)]
        pub unsafe fn paddingPolicy(&self) -> MLCPaddingPolicy;

        #[method(paddingSizeInX)]
        pub unsafe fn paddingSizeInX(&self) -> NSUInteger;

        #[method(paddingSizeInY)]
        pub unsafe fn paddingSizeInY(&self) -> NSUInteger;

        #[method(isConvolutionTranspose)]
        pub unsafe fn isConvolutionTranspose(&self) -> bool;

        #[method(usesDepthwiseConvolution)]
        pub unsafe fn usesDepthwiseConvolution(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other descriptorWithType:kernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithType_kernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            convolution_type: MLCConvolutionType,
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other descriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:outputFeatureChannelCount:)]
        pub unsafe fn descriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other descriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other descriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:outputFeatureChannelCount:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other convolutionTransposeDescriptorWithKernelSizes:inputFeatureChannelCount:outputFeatureChannelCount:groupCount:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            output_feature_channel_count: NSUInteger,
            group_count: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelWidth:kernelHeight:inputFeatureChannelCount:channelMultiplier:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_channelMultiplier(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelSizes:inputFeatureChannelCount:channelMultiplier:strides:paddingPolicy:paddingSizes:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other depthwiseConvolutionDescriptorWithKernelSizes:inputFeatureChannelCount:channelMultiplier:strides:dilationRates:paddingPolicy:paddingSizes:)]
        pub unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            input_feature_channel_count: NSUInteger,
            channel_multiplier: NSUInteger,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MLCompute_MLCConvolutionDescriptor")]
    unsafe impl MLCConvolutionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
