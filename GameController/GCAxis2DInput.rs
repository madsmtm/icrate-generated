//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait GCAxis2DInput: NSObjectProtocol {
        #[cfg(all(
            feature = "GameController_GCPhysicalInputElement",
            feature = "GameController_GCTypes",
            feature = "block2"
        ))]
        #[method(valueDidChangeHandler)]
        unsafe fn valueDidChangeHandler(
            &self,
        ) -> *mut Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCAxis2DInput>>,
                GCPoint2,
            ),
        >;

        #[cfg(all(
            feature = "GameController_GCPhysicalInputElement",
            feature = "GameController_GCTypes",
            feature = "block2"
        ))]
        #[method(setValueDidChangeHandler:)]
        unsafe fn setValueDidChangeHandler(
            &self,
            value_did_change_handler: Option<
                &Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCAxis2DInput>>,
                        GCPoint2,
                    ),
                >,
            >,
        );

        #[cfg(feature = "GameController_GCTypes")]
        #[method(value)]
        unsafe fn value(&self) -> GCPoint2;

        #[method(isAnalog)]
        unsafe fn isAnalog(&self) -> bool;

        #[method(canWrap)]
        unsafe fn canWrap(&self) -> bool;

        #[method(lastValueTimestamp)]
        unsafe fn lastValueTimestamp(&self) -> NSTimeInterval;

        #[method(lastValueLatency)]
        unsafe fn lastValueLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GameController_GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Id<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCAxis2DInput {}
);
