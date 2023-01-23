//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCMouseMoved = *mut Block<(NonNull<GCMouseInput>, c_float, c_float), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCMouseInput")]
    pub struct GCMouseInput;

    #[cfg(feature = "GameController_GCMouseInput")]
    unsafe impl ClassType for GCMouseInput {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCMouseInput")]
    unsafe impl GCMouseInput {
        #[method(mouseMovedHandler)]
        pub unsafe fn mouseMovedHandler(&self) -> GCMouseMoved;

        #[method(setMouseMovedHandler:)]
        pub unsafe fn setMouseMovedHandler(&self, mouse_moved_handler: GCMouseMoved);

        #[cfg(feature = "GameController_GCDeviceCursor")]
        #[method_id(@__retain_semantics Other scroll)]
        pub unsafe fn scroll(&self) -> Id<GCDeviceCursor, Shared>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other leftButton)]
        pub unsafe fn leftButton(&self) -> Id<GCControllerButtonInput, Shared>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other rightButton)]
        pub unsafe fn rightButton(&self) -> Option<Id<GCControllerButtonInput, Shared>>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other middleButton)]
        pub unsafe fn middleButton(&self) -> Option<Id<GCControllerButtonInput, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameController_GCControllerButtonInput"
        ))]
        #[method_id(@__retain_semantics Other auxiliaryButtons)]
        pub unsafe fn auxiliaryButtons(
            &self,
        ) -> Option<Id<NSArray<GCControllerButtonInput>, Shared>>;
    }
);
