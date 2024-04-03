//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait SKOverlayDelegate: NSObjectProtocol {
        #[optional]
        #[method(storeOverlay:didFailToLoadWithError:)]
        unsafe fn storeOverlay_didFailToLoadWithError(&self, overlay: &SKOverlay, error: &NSError);

        #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
        #[optional]
        #[method(storeOverlay:willStartPresentation:)]
        unsafe fn storeOverlay_willStartPresentation(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
        #[optional]
        #[method(storeOverlay:didFinishPresentation:)]
        unsafe fn storeOverlay_didFinishPresentation(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
        #[optional]
        #[method(storeOverlay:willStartDismissal:)]
        unsafe fn storeOverlay_willStartDismissal(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
        #[optional]
        #[method(storeOverlay:didFinishDismissal:)]
        unsafe fn storeOverlay_didFinishDismissal(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );
    }

    unsafe impl ProtocolType for dyn SKOverlayDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKOverlay;

    unsafe impl ClassType for SKOverlay {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKOverlay {}

extern_methods!(
    unsafe impl SKOverlay {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &SKOverlayConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn SKOverlayDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn SKOverlayDelegate>>);

        #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<SKOverlayConfiguration>;
    }
);
