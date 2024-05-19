//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(all(feature = "GCControllerElement", feature = "block2"))]
pub type GCControllerButtonValueChangedHandler =
    *mut block2::Block<dyn Fn(NonNull<GCControllerButtonInput>, c_float, Bool)>;

#[cfg(all(feature = "GCControllerElement", feature = "block2"))]
pub type GCControllerButtonTouchedChangedHandler =
    *mut block2::Block<dyn Fn(NonNull<GCControllerButtonInput>, c_float, Bool, Bool)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCControllerElement")]
    pub struct GCControllerButtonInput;

    #[cfg(feature = "GCControllerElement")]
    unsafe impl ClassType for GCControllerButtonInput {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerButtonInput {}

extern_methods!(
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerButtonInput {
        #[cfg(feature = "block2")]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerButtonValueChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerButtonValueChangedHandler,
        );

        #[cfg(feature = "block2")]
        #[method(pressedChangedHandler)]
        pub unsafe fn pressedChangedHandler(&self) -> GCControllerButtonValueChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setPressedChangedHandler:)]
        pub unsafe fn setPressedChangedHandler(
            &self,
            pressed_changed_handler: GCControllerButtonValueChangedHandler,
        );

        #[cfg(feature = "block2")]
        #[method(touchedChangedHandler)]
        pub unsafe fn touchedChangedHandler(&self) -> GCControllerButtonTouchedChangedHandler;

        #[cfg(feature = "block2")]
        #[method(setTouchedChangedHandler:)]
        pub unsafe fn setTouchedChangedHandler(
            &self,
            touched_changed_handler: GCControllerButtonTouchedChangedHandler,
        );

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(isPressed)]
        pub unsafe fn isPressed(&self) -> bool;

        #[method(isTouched)]
        pub unsafe fn isTouched(&self) -> bool;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerButtonInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
