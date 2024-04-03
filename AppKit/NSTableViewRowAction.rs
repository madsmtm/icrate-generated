//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTableViewRowActionStyle(pub NSInteger);
impl NSTableViewRowActionStyle {
    #[doc(alias = "NSTableViewRowActionStyleRegular")]
    pub const Regular: Self = Self(0);
    #[doc(alias = "NSTableViewRowActionStyleDestructive")]
    pub const Destructive: Self = Self(1);
}

unsafe impl Encode for NSTableViewRowActionStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTableViewRowActionStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableViewRowAction;

    unsafe impl ClassType for NSTableViewRowAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for NSTableViewRowAction {}

unsafe impl NSObjectProtocol for NSTableViewRowAction {}

extern_methods!(
    unsafe impl NSTableViewRowAction {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other rowActionWithStyle:title:handler:)]
        pub unsafe fn rowActionWithStyle_title_handler(
            style: NSTableViewRowActionStyle,
            title: &NSString,
            handler: &Block<dyn Fn(NonNull<NSTableViewRowAction>, NSInteger)>,
        ) -> Id<Self>;

        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewRowActionStyle;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTableViewRowAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
