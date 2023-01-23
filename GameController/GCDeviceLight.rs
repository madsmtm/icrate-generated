//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDeviceLight")]
    pub struct GCDeviceLight;

    #[cfg(feature = "GameController_GCDeviceLight")]
    unsafe impl ClassType for GCDeviceLight {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCDeviceLight")]
    unsafe impl GCDeviceLight {
        #[cfg(feature = "GameController_GCColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<GCColor, Shared>;

        #[cfg(feature = "GameController_GCColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &GCColor);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
