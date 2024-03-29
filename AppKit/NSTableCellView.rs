//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct NSTableCellView;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for NSTableCellView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSTableCellView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for NSTableCellView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTableCellView {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableCellView {
        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<AnyObject>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&AnyObject>);

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSTextField"))]
        #[method_id(@__retain_semantics Other textField)]
        pub unsafe fn textField(&self) -> Option<Id<NSTextField>>;

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSTextField"))]
        #[method(setTextField:)]
        pub unsafe fn setTextField(&self, text_field: Option<&NSTextField>);

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSImageView"))]
        #[method_id(@__retain_semantics Other imageView)]
        pub unsafe fn imageView(&self) -> Option<Id<NSImageView>>;

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSImageView"))]
        #[method(setImageView:)]
        pub unsafe fn setImageView(&self, image_view: Option<&NSImageView>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, background_style: NSBackgroundStyle);

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(rowSizeStyle)]
        pub unsafe fn rowSizeStyle(&self) -> NSTableViewRowSizeStyle;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setRowSizeStyle:)]
        pub unsafe fn setRowSizeStyle(&self, row_size_style: NSTableViewRowSizeStyle);

        #[cfg(all(feature = "AppKit_NSDraggingItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other draggingImageComponents)]
        pub unsafe fn draggingImageComponents(&self) -> Id<NSArray<NSDraggingImageComponent>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableCellView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableCellView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableCellView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
