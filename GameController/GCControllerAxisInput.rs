//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

#[cfg(feature = "GameController_GCControllerElement")]
pub type GCControllerAxisValueChangedHandler =
    *mut Block<dyn Fn(NonNull<GCControllerAxisInput>, c_float)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerElement")]
    pub struct GCControllerAxisInput;

    #[cfg(feature = "GameController_GCControllerElement")]
    unsafe impl ClassType for GCControllerAxisInput {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerAxisInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerElement")]
    unsafe impl GCControllerAxisInput {
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerAxisValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerAxisValueChangedHandler,
        );

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCControllerElement")]
    unsafe impl GCControllerAxisInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
