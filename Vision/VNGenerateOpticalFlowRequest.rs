//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNGenerateOpticalFlowRequestComputationAccuracy(pub NSUInteger);
impl VNGenerateOpticalFlowRequestComputationAccuracy {
    #[doc(alias = "VNGenerateOpticalFlowRequestComputationAccuracyLow")]
    pub const Low: Self = Self(0);
    #[doc(alias = "VNGenerateOpticalFlowRequestComputationAccuracyMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "VNGenerateOpticalFlowRequestComputationAccuracyHigh")]
    pub const High: Self = Self(2);
    #[doc(alias = "VNGenerateOpticalFlowRequestComputationAccuracyVeryHigh")]
    pub const VeryHigh: Self = Self(3);
}

unsafe impl Encode for VNGenerateOpticalFlowRequestComputationAccuracy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNGenerateOpticalFlowRequestComputationAccuracy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    pub struct VNGenerateOpticalFlowRequest;

    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl ClassType for VNGenerateOpticalFlowRequest {
        #[inherits(VNImageBasedRequest, VNRequest, NSObject)]
        type Super = VNTargetedImageRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSCopying for VNGenerateOpticalFlowRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSObjectProtocol for VNGenerateOpticalFlowRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNGenerateOpticalFlowRequest {
        #[method(computationAccuracy)]
        pub unsafe fn computationAccuracy(&self)
            -> VNGenerateOpticalFlowRequestComputationAccuracy;

        #[method(setComputationAccuracy:)]
        pub unsafe fn setComputationAccuracy(
            &self,
            computation_accuracy: VNGenerateOpticalFlowRequestComputationAccuracy,
        );

        #[method(outputPixelFormat)]
        pub unsafe fn outputPixelFormat(&self) -> OSType;

        #[method(setOutputPixelFormat:)]
        pub unsafe fn setOutputPixelFormat(&self, output_pixel_format: OSType);

        #[method(keepNetworkOutput)]
        pub unsafe fn keepNetworkOutput(&self) -> bool;

        #[method(setKeepNetworkOutput:)]
        pub unsafe fn setKeepNetworkOutput(&self, keep_network_output: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNPixelBufferObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNTargetedImageRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNGenerateOpticalFlowRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-image"))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:)]
        pub unsafe fn initWithTargetedCIImage_options(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-image"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:completionHandler:)]
        pub unsafe fn initWithTargetedCIImage_options_completionHandler(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:)]
        pub unsafe fn initWithTargetedImageURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageURL_options_completionHandler(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:)]
        pub unsafe fn initWithTargetedImageData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageData_options_completionHandler(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNGenerateOpticalFlowRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNGenerateOpticalFlowRequestRevision1: NSUInteger = 1;

pub static VNGenerateOpticalFlowRequestRevision2: NSUInteger = 2;