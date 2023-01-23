//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCDeviceBatteryState {
        GCDeviceBatteryStateUnknown = -1,
        GCDeviceBatteryStateDischarging = 0,
        GCDeviceBatteryStateCharging = 1,
        GCDeviceBatteryStateFull = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDeviceBattery")]
    pub struct GCDeviceBattery;

    #[cfg(feature = "GameController_GCDeviceBattery")]
    unsafe impl ClassType for GCDeviceBattery {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCDeviceBattery")]
    unsafe impl GCDeviceBattery {
        #[method(batteryLevel)]
        pub unsafe fn batteryLevel(&self) -> c_float;

        #[method(batteryState)]
        pub unsafe fn batteryState(&self) -> GCDeviceBatteryState;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
