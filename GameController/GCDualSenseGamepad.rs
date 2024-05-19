//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    pub struct GCDualSenseGamepad;

    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl ClassType for GCDualSenseGamepad {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCDualSenseGamepad {}

extern_methods!(
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDualSenseGamepad {
        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other touchpadButton)]
        pub unsafe fn touchpadButton(&self) -> Id<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other touchpadPrimary)]
        pub unsafe fn touchpadPrimary(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other touchpadSecondary)]
        pub unsafe fn touchpadSecondary(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(all(
            feature = "GCControllerButtonInput",
            feature = "GCControllerElement",
            feature = "GCDualSenseAdaptiveTrigger"
        ))]
        #[method_id(@__retain_semantics Other leftTrigger)]
        pub unsafe fn leftTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger>;

        #[cfg(all(
            feature = "GCControllerButtonInput",
            feature = "GCControllerElement",
            feature = "GCDualSenseAdaptiveTrigger"
        ))]
        #[method_id(@__retain_semantics Other rightTrigger)]
        pub unsafe fn rightTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDualSenseGamepad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
