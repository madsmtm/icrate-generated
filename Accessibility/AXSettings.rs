//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

extern "C" {
    pub fn AXPrefersHorizontalTextLayout() -> Bool;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static AXPrefersHorizontalTextLayoutDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub fn AXAnimatedImagesEnabled() -> Bool;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static AXAnimatedImagesEnabledDidChangeNotification: &'static NSNotificationName;
}
