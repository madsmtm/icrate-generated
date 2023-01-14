//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPathControlItem")]
    pub struct NSPathControlItem;

    #[cfg(feature = "AppKit_NSPathControlItem")]
    unsafe impl ClassType for NSPathControlItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPathControlItem")]
    unsafe impl NSPathControlItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
    }
);
