//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSDistributedNotificationCenterType = NSString;
);

extern_static!(NSLocalNotificationCenterType: &'static NSDistributedNotificationCenterType);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNotificationSuspensionBehavior {
        NSNotificationSuspensionBehaviorDrop = 1,
        NSNotificationSuspensionBehaviorCoalesce = 2,
        NSNotificationSuspensionBehaviorHold = 3,
        NSNotificationSuspensionBehaviorDeliverImmediately = 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDistributedNotificationOptions {
        NSDistributedNotificationDeliverImmediately = 1 << 0,
        NSDistributedNotificationPostToAllSessions = 1 << 1,
    }
);

extern_static!(
    NSNotificationDeliverImmediately: NSDistributedNotificationOptions =
        NSDistributedNotificationDeliverImmediately
);

extern_static!(
    NSNotificationPostToAllSessions: NSDistributedNotificationOptions =
        NSDistributedNotificationPostToAllSessions
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDistributedNotificationCenter")]
    pub struct NSDistributedNotificationCenter;

    #[cfg(feature = "Foundation_NSDistributedNotificationCenter")]
    unsafe impl ClassType for NSDistributedNotificationCenter {
        #[inherits(NSObject)]
        type Super = NSNotificationCenter;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDistributedNotificationCenter")]
    unsafe impl NSDistributedNotificationCenter {
        #[method_id(@__retain_semantics Other notificationCenterForType:)]
        pub unsafe fn notificationCenterForType(
            notification_center_type: &NSDistributedNotificationCenterType,
        ) -> Id<NSDistributedNotificationCenter, Shared>;

        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<NSDistributedNotificationCenter, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:selector:name:object:suspensionBehavior:)]
        pub unsafe fn addObserver_selector_name_object_suspensionBehavior(
            &self,
            observer: &Object,
            selector: Sel,
            name: Option<&NSNotificationName>,
            object: Option<&NSString>,
            suspension_behavior: NSNotificationSuspensionBehavior,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(postNotificationName:object:userInfo:deliverImmediately:)]
        pub unsafe fn postNotificationName_object_userInfo_deliverImmediately(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            user_info: Option<&NSDictionary>,
            deliver_immediately: bool,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(postNotificationName:object:userInfo:options:)]
        pub unsafe fn postNotificationName_object_userInfo_options(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            user_info: Option<&NSDictionary>,
            options: NSDistributedNotificationOptions,
        );

        #[method(suspended)]
        pub unsafe fn suspended(&self) -> bool;

        #[method(setSuspended:)]
        pub unsafe fn setSuspended(&self, suspended: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            a_selector: Sel,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&NSString>,
            a_user_info: Option<&NSDictionary>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&NSString>,
        );
    }
);
