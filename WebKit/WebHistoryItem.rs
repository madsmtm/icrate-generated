//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebHistoryItemChangedNotification: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebHistoryItem")]
    #[deprecated]
    pub struct WebHistoryItem;

    #[cfg(feature = "WebKit_WebHistoryItem")]
    unsafe impl ClassType for WebHistoryItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WebHistoryItem")]
    unsafe impl WebHistoryItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithURLString:title:lastVisitedTimeInterval:)]
        pub unsafe fn initWithURLString_title_lastVisitedTimeInterval(
            this: Option<Allocated<Self>>,
            url_string: Option<&NSString>,
            title: Option<&NSString>,
            time: NSTimeInterval,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other originalURLString)]
        pub unsafe fn originalURLString(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other URLString)]
        pub unsafe fn URLString(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(lastVisitedTimeInterval)]
        pub unsafe fn lastVisitedTimeInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage, Shared>>;
    }
);