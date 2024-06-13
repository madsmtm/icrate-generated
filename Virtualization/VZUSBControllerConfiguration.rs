//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZUSBControllerConfiguration;

    unsafe impl ClassType for VZUSBControllerConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VZUSBControllerConfiguration {}

unsafe impl NSObjectProtocol for VZUSBControllerConfiguration {}

extern_methods!(
    unsafe impl VZUSBControllerConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZUSBDeviceConfiguration")]
        #[method_id(@__retain_semantics Other usbDevices)]
        pub unsafe fn usbDevices(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn VZUSBDeviceConfiguration>>>;

        #[cfg(feature = "VZUSBDeviceConfiguration")]
        #[method(setUsbDevices:)]
        pub unsafe fn setUsbDevices(
            &self,
            usb_devices: &NSArray<ProtocolObject<dyn VZUSBDeviceConfiguration>>,
        );
    }
);
