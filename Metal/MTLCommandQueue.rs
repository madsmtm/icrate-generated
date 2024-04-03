//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MTLCommandQueue: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Metal_MTLCommandBuffer")]
        #[method_id(@__retain_semantics Other commandBuffer)]
        fn commandBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[cfg(feature = "Metal_MTLCommandBuffer")]
        #[method_id(@__retain_semantics Other commandBufferWithDescriptor:)]
        unsafe fn commandBufferWithDescriptor(
            &self,
            descriptor: &MTLCommandBufferDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[cfg(feature = "Metal_MTLCommandBuffer")]
        #[method_id(@__retain_semantics Other commandBufferWithUnretainedReferences)]
        unsafe fn commandBufferWithUnretainedReferences(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[deprecated = "Use MTLCaptureScope instead"]
        #[method(insertDebugCaptureBoundary)]
        unsafe fn insertDebugCaptureBoundary(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandQueue {}
);
