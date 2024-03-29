//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    #[cfg(feature = "GameController_GCPhysicalInputElement")]
    pub unsafe trait GCButtonElement: GCPhysicalInputElement {
        #[cfg(all(
            feature = "GameController_GCLinearInput",
            feature = "GameController_GCPressedStateInput"
        ))]
        #[method_id(@__retain_semantics Other pressedInput)]
        unsafe fn pressedInput(&self) -> Id<TodoProtocols>;

        #[cfg(feature = "GameController_GCTouchedStateInput")]
        #[method_id(@__retain_semantics Other touchedInput)]
        unsafe fn touchedInput(&self) -> Option<Id<ProtocolObject<dyn GCTouchedStateInput>>>;
    }

    #[cfg(feature = "GameController_GCPhysicalInputElement")]
    unsafe impl ProtocolType for dyn GCButtonElement {}
);
