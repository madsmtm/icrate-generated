//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "GameController_GCPhysicalInputElement")]
    pub unsafe trait GCAxisElement: GCPhysicalInputElement {
        #[cfg(feature = "GameController_GCAxisInput")]
        #[method_id(@__retain_semantics Other absoluteInput)]
        unsafe fn absoluteInput(&self) -> Option<Id<ProtocolObject<dyn GCAxisInput>>>;

        #[cfg(feature = "GameController_GCRelativeInput")]
        #[method_id(@__retain_semantics Other relativeInput)]
        unsafe fn relativeInput(&self) -> Id<ProtocolObject<dyn GCRelativeInput>>;
    }

    #[cfg(feature = "GameController_GCPhysicalInputElement")]
    unsafe impl ProtocolType for dyn GCAxisElement {}
);
