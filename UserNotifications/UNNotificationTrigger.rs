//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationTrigger;

    unsafe impl ClassType for UNNotificationTrigger {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationTrigger {}

unsafe impl NSCopying for UNNotificationTrigger {}

unsafe impl NSObjectProtocol for UNNotificationTrigger {}

unsafe impl NSSecureCoding for UNNotificationTrigger {}

extern_methods!(
    unsafe impl UNNotificationTrigger {
        #[method(repeats)]
        pub unsafe fn repeats(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNPushNotificationTrigger;

    unsafe impl ClassType for UNPushNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNPushNotificationTrigger {}

unsafe impl NSCopying for UNPushNotificationTrigger {}

unsafe impl NSObjectProtocol for UNPushNotificationTrigger {}

unsafe impl NSSecureCoding for UNPushNotificationTrigger {}

extern_methods!(
    unsafe impl UNPushNotificationTrigger {}
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNTimeIntervalNotificationTrigger;

    unsafe impl ClassType for UNTimeIntervalNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNTimeIntervalNotificationTrigger {}

unsafe impl NSCopying for UNTimeIntervalNotificationTrigger {}

unsafe impl NSObjectProtocol for UNTimeIntervalNotificationTrigger {}

unsafe impl NSSecureCoding for UNTimeIntervalNotificationTrigger {}

extern_methods!(
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other triggerWithTimeInterval:repeats:)]
        pub unsafe fn triggerWithTimeInterval_repeats(
            time_interval: NSTimeInterval,
            repeats: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNCalendarNotificationTrigger;

    unsafe impl ClassType for UNCalendarNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNCalendarNotificationTrigger {}

unsafe impl NSCopying for UNCalendarNotificationTrigger {}

unsafe impl NSObjectProtocol for UNCalendarNotificationTrigger {}

unsafe impl NSSecureCoding for UNCalendarNotificationTrigger {}

extern_methods!(
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics Other dateComponents)]
        pub unsafe fn dateComponents(&self) -> Id<NSDateComponents>;

        #[method_id(@__retain_semantics Other triggerWithDateMatchingComponents:repeats:)]
        pub unsafe fn triggerWithDateMatchingComponents_repeats(
            date_components: &NSDateComponents,
            repeats: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNLocationNotificationTrigger;

    unsafe impl ClassType for UNLocationNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNLocationNotificationTrigger {}

unsafe impl NSCopying for UNLocationNotificationTrigger {}

unsafe impl NSObjectProtocol for UNLocationNotificationTrigger {}

unsafe impl NSSecureCoding for UNLocationNotificationTrigger {}

extern_methods!(
    unsafe impl UNLocationNotificationTrigger {
        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Id<CLRegion>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other triggerWithRegion:repeats:)]
        pub unsafe fn triggerWithRegion_repeats(region: &CLRegion, repeats: bool) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
