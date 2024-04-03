//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    pub struct NSMenuItemCell;

    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    unsafe impl ClassType for NSMenuItemCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSButtonCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSAccessibility for NSMenuItemCell {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSMenuItemCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSCoding for NSMenuItemCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSCopying for NSMenuItemCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell"
))]
unsafe impl NSObjectProtocol for NSMenuItemCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSButtonCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSMenuItemCell {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other menuItem)]
        pub unsafe fn menuItem(&self) -> Option<Id<NSMenuItem>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(setMenuItem:)]
        pub unsafe fn setMenuItem(&self, menu_item: Option<&NSMenuItem>);

        #[method(needsSizing)]
        pub unsafe fn needsSizing(&self) -> bool;

        #[method(setNeedsSizing:)]
        pub unsafe fn setNeedsSizing(&self, needs_sizing: bool);

        #[method(calcSize)]
        pub unsafe fn calcSize(&self);

        #[method(needsDisplay)]
        pub unsafe fn needsDisplay(&self) -> bool;

        #[method(setNeedsDisplay:)]
        pub unsafe fn setNeedsDisplay(&self, needs_display: bool);

        #[method(stateImageWidth)]
        pub unsafe fn stateImageWidth(&self) -> CGFloat;

        #[method(imageWidth)]
        pub unsafe fn imageWidth(&self) -> CGFloat;

        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;

        #[method(keyEquivalentWidth)]
        pub unsafe fn keyEquivalentWidth(&self) -> CGFloat;

        #[method(stateImageRectForBounds:)]
        pub unsafe fn stateImageRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[method(keyEquivalentRectForBounds:)]
        pub unsafe fn keyEquivalentRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawSeparatorItemWithFrame:inView:)]
        pub unsafe fn drawSeparatorItemWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawStateImageWithFrame:inView:)]
        pub unsafe fn drawStateImageWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawImageWithFrame:inView:)]
        pub unsafe fn drawImageWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawTitleWithFrame:inView:)]
        pub unsafe fn drawTitleWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawKeyEquivalentWithFrame:inView:)]
        pub unsafe fn drawKeyEquivalentWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(drawBorderAndBackgroundWithFrame:inView:)]
        pub unsafe fn drawBorderAndBackgroundWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButtonCell`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    unsafe impl NSMenuItemCell {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSButtonCell",
        feature = "AppKit_NSCell"
    ))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
