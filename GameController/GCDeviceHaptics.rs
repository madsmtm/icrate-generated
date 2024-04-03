//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type GCHapticsLocality = NSString;

extern "C" {
    pub static GCHapticsLocalityDefault: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityAll: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityHandles: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityLeftHandle: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityRightHandle: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityTriggers: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityLeftTrigger: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticsLocalityRightTrigger: &'static GCHapticsLocality;
}

extern "C" {
    pub static GCHapticDurationInfinite: c_float;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCDeviceHaptics;

    unsafe impl ClassType for GCDeviceHaptics {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GCDeviceHaptics {}

extern_methods!(
    unsafe impl GCDeviceHaptics {
        #[method_id(@__retain_semantics Other supportedLocalities)]
        pub unsafe fn supportedLocalities(&self) -> Id<NSSet<GCHapticsLocality>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCDeviceHaptics {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
