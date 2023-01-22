//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKFindConfiguration")]
    pub struct WKFindConfiguration;

    #[cfg(feature = "WebKit_WKFindConfiguration")]
    unsafe impl ClassType for WKFindConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WKFindConfiguration")]
    unsafe impl WKFindConfiguration {
        #[method(backwards)]
        pub unsafe fn backwards(&self) -> bool;

        #[method(setBackwards:)]
        pub unsafe fn setBackwards(&self, backwards: bool);

        #[method(caseSensitive)]
        pub unsafe fn caseSensitive(&self) -> bool;

        #[method(setCaseSensitive:)]
        pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);
    }
);