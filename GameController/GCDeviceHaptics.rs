//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type GCHapticsLocality = NSString;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityDefault: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityAll: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityHandles: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityLeftHandle: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityRightHandle: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityTriggers: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static GCHapticsLocalityLeftTrigger: &'static GCHapticsLocality;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
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
        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
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
