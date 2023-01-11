//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSecureTextField")]
    pub struct NSSecureTextField;

    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl ClassType for NSSecureTextField {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    pub struct NSSecureTextFieldCell;

    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl ClassType for NSSecureTextFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl NSSecureTextFieldCell {
        #[method(echosBullets)]
        pub unsafe fn echosBullets(&self) -> bool;

        #[method(setEchosBullets:)]
        pub unsafe fn setEchosBullets(&self, echosBullets: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextField`
    ///
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(stringValue: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(stringValue: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributedStringValue: &NSAttributedString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(stringValue: &NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl NSSecureTextFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
