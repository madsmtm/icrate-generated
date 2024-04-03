//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotification;

    unsafe impl ClassType for UNNotification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotification {}

unsafe impl NSCopying for UNNotification {}

unsafe impl NSObjectProtocol for UNNotification {}

unsafe impl NSSecureCoding for UNNotification {}

extern_methods!(
    unsafe impl UNNotification {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[cfg(feature = "UserNotifications_UNNotificationRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<UNNotificationRequest>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotification {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
