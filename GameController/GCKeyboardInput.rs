//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(
    feature = "GameController_GCControllerButtonInput",
    feature = "GameController_GCControllerElement",
    feature = "GameController_GCKeyCodes",
    feature = "GameController_GCPhysicalInputProfile",
    feature = "block2"
))]
pub type GCKeyboardValueChangedHandler =
    *mut Block<dyn Fn(NonNull<GCKeyboardInput>, NonNull<GCControllerButtonInput>, GCKeyCode, Bool)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    pub struct GCKeyboardInput;

    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    unsafe impl ClassType for GCKeyboardInput {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCPhysicalInputProfile")]
unsafe impl NSObjectProtocol for GCKeyboardInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    unsafe impl GCKeyboardInput {
        #[cfg(all(
            feature = "GameController_GCControllerButtonInput",
            feature = "GameController_GCControllerElement",
            feature = "GameController_GCKeyCodes",
            feature = "block2"
        ))]
        #[method(keyChangedHandler)]
        pub unsafe fn keyChangedHandler(&self) -> GCKeyboardValueChangedHandler;

        #[cfg(all(
            feature = "GameController_GCControllerButtonInput",
            feature = "GameController_GCControllerElement",
            feature = "GameController_GCKeyCodes",
            feature = "block2"
        ))]
        #[method(setKeyChangedHandler:)]
        pub unsafe fn setKeyChangedHandler(
            &self,
            key_changed_handler: GCKeyboardValueChangedHandler,
        );

        #[method(isAnyKeyPressed)]
        pub unsafe fn isAnyKeyPressed(&self) -> bool;

        #[cfg(all(
            feature = "GameController_GCControllerButtonInput",
            feature = "GameController_GCControllerElement",
            feature = "GameController_GCKeyCodes"
        ))]
        #[method_id(@__retain_semantics Other buttonForKeyCode:)]
        pub unsafe fn buttonForKeyCode(
            &self,
            code: GCKeyCode,
        ) -> Option<Id<GCControllerButtonInput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    unsafe impl GCKeyboardInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
