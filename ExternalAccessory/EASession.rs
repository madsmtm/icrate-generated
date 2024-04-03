//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EASession;

    unsafe impl ClassType for EASession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for EASession {}

extern_methods!(
    unsafe impl EASession {
        #[cfg(feature = "ExternalAccessory_EAAccessory")]
        #[method_id(@__retain_semantics Init initWithAccessory:forProtocol:)]
        pub unsafe fn initWithAccessory_forProtocol(
            this: Allocated<Self>,
            accessory: &EAAccessory,
            protocol_string: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "ExternalAccessory_EAAccessory")]
        #[method_id(@__retain_semantics Other accessory)]
        pub unsafe fn accessory(&self) -> Option<Id<EAAccessory>>;

        #[method_id(@__retain_semantics Other protocolString)]
        pub unsafe fn protocolString(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other inputStream)]
        pub unsafe fn inputStream(&self) -> Option<Id<NSInputStream>>;

        #[method_id(@__retain_semantics Other outputStream)]
        pub unsafe fn outputStream(&self) -> Option<Id<NSOutputStream>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EASession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
