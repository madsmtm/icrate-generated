//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTokenStyle {
        NSTokenStyleDefault = 0,
        NSTokenStyleNone = 1,
        NSTokenStyleRounded = 2,
        NSTokenStyleSquared = 3,
        NSTokenStylePlainSquared = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTokenFieldCell;

    unsafe impl ClassType for NSTokenFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, tokenStyle: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completionDelay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay() -> NSTimeInterval;

        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<NSCharacterSet, Shared>;

        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizingCharacterSet: Option<&NSCharacterSet>,
        );

        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTokenFieldCellDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTokenFieldCellDelegate>);
    }
);

extern_protocol!(
    pub struct NSTokenFieldCellDelegate;

    unsafe impl ProtocolType for NSTokenFieldCellDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        pub unsafe fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            substring: &NSString,
            tokenIndex: NSInteger,
            selectedIndex: NonNull<NSInteger>,
        ) -> Id<NSArray, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:shouldAddObjects:atIndex:)]
        pub unsafe fn tokenFieldCell_shouldAddObjects_atIndex(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Id<NSArray, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:displayStringForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_displayStringForRepresentedObject(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:editingStringForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_editingStringForRepresentedObject(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:representedObjectForEditingString:)]
        pub unsafe fn tokenFieldCell_representedObjectForEditingString(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            editingString: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(tokenFieldCell:writeRepresentedObjects:toPasteboard:)]
        pub unsafe fn tokenFieldCell_writeRepresentedObjects_toPasteboard(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:readFromPasteboard:)]
        pub unsafe fn tokenFieldCell_readFromPasteboard(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            pboard: &NSPasteboard,
        ) -> Option<Id<NSArray, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:menuForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_menuForRepresentedObject(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<NSMenu, Shared>>;

        #[optional]
        #[method(tokenFieldCell:hasMenuForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_hasMenuForRepresentedObject(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            representedObject: &Object,
        ) -> bool;

        #[optional]
        #[method(tokenFieldCell:styleForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_styleForRepresentedObject(
            &self,
            tokenFieldCell: &NSTokenFieldCell,
            representedObject: &Object,
        ) -> NSTokenStyle;
    }
);

extern_static!(NSDefaultTokenStyle: NSTokenStyle = NSTokenStyleDefault);

extern_static!(NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyleNone);

extern_static!(NSRoundedTokenStyle: NSTokenStyle = NSTokenStyleRounded);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    unsafe impl NSTokenFieldCell {
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
