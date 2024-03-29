//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCKeyboardDidConnectNotification: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCKeyboardDidDisconnectNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCKeyboard;

    unsafe impl ClassType for GCKeyboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCDevice")]
unsafe impl GCDevice for GCKeyboard {}

unsafe impl NSObjectProtocol for GCKeyboard {}

extern_methods!(
    unsafe impl GCKeyboard {
        #[cfg(all(
            feature = "GameController_GCKeyboardInput",
            feature = "GameController_GCPhysicalInputProfile"
        ))]
        #[method_id(@__retain_semantics Other keyboardInput)]
        pub unsafe fn keyboardInput(&self) -> Option<Id<GCKeyboardInput>>;

        #[method_id(@__retain_semantics Other coalescedKeyboard)]
        pub unsafe fn coalescedKeyboard() -> Option<Id<GCKeyboard>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCKeyboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
