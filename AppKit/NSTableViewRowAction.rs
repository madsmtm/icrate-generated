//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTableViewRowActionStyle {
        NSTableViewRowActionStyleRegular = 0,
        NSTableViewRowActionStyleDestructive = 1,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSTableViewRowAction;

    unsafe impl ClassType for NSTableViewRowAction {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTableViewRowAction {
        #[method_id(@__retain_semantics Other rowActionWithStyle:title:handler:)]
        pub unsafe fn rowActionWithStyle_title_handler(
            style: NSTableViewRowActionStyle,
            title: &NSString,
            handler: &Block<(NonNull<NSTableViewRowAction>, NSInteger), ()>,
        ) -> Id<Self, Shared>;

        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewRowActionStyle;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
    }
);
