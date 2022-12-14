//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPathComponentCell;

    unsafe impl ClassType for NSPathComponentCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    unsafe impl NSPathComponentCell {
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    unsafe impl NSPathComponentCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
