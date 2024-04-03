//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPostingStyle(pub NSUInteger);
impl NSPostingStyle {
    pub const NSPostWhenIdle: Self = Self(1);
    pub const NSPostASAP: Self = Self(2);
    pub const NSPostNow: Self = Self(3);
}

unsafe impl Encode for NSPostingStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPostingStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNotificationCoalescing(pub NSUInteger);
impl NSNotificationCoalescing {
    pub const NSNotificationNoCoalescing: Self = Self(0);
    #[doc(alias = "NSNotificationCoalescingOnName")]
    pub const OnName: Self = Self(1);
    #[doc(alias = "NSNotificationCoalescingOnSender")]
    pub const OnSender: Self = Self(2);
}

unsafe impl Encode for NSNotificationCoalescing {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNotificationCoalescing {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotificationQueue;

    unsafe impl ClassType for NSNotificationQueue {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSNotificationQueue {}

extern_methods!(
    unsafe impl NSNotificationQueue {
        #[method_id(@__retain_semantics Other defaultQueue)]
        pub unsafe fn defaultQueue() -> Id<NSNotificationQueue>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method_id(@__retain_semantics Init initWithNotificationCenter:)]
        pub unsafe fn initWithNotificationCenter(
            this: Allocated<Self>,
            notification_center: &NSNotificationCenter,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(enqueueNotification:postingStyle:)]
        pub unsafe fn enqueueNotification_postingStyle(
            &self,
            notification: &NSNotification,
            posting_style: NSPostingStyle,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSNotification",
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSString"
        ))]
        #[method(enqueueNotification:postingStyle:coalesceMask:forModes:)]
        pub unsafe fn enqueueNotification_postingStyle_coalesceMask_forModes(
            &self,
            notification: &NSNotification,
            posting_style: NSPostingStyle,
            coalesce_mask: NSNotificationCoalescing,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(dequeueNotificationsMatching:coalesceMask:)]
        pub unsafe fn dequeueNotificationsMatching_coalesceMask(
            &self,
            notification: &NSNotification,
            coalesce_mask: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNotificationQueue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
