//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPressureConfiguration")]
    pub struct NSPressureConfiguration;

    #[cfg(feature = "AppKit_NSPressureConfiguration")]
    unsafe impl ClassType for NSPressureConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPressureConfiguration")]
    unsafe impl NSPressureConfiguration {
        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[method_id(@__retain_semantics Init initWithPressureBehavior:)]
        pub unsafe fn initWithPressureBehavior(
            this: Option<Allocated<Self>>,
            pressure_behavior: NSPressureBehavior,
        ) -> Id<Self, Shared>;

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// NSPressureConfiguration
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSPressureConfiguration")]
        #[method_id(@__retain_semantics Other pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Option<Id<NSPressureConfiguration, Shared>>;

        #[cfg(feature = "AppKit_NSPressureConfiguration")]
        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressure_configuration: Option<&NSPressureConfiguration>,
        );
    }
);
