//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSToolbarItem")]
    pub struct NSTrackingSeparatorToolbarItem;

    #[cfg(feature = "AppKit_NSToolbarItem")]
    unsafe impl ClassType for NSTrackingSeparatorToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSToolbarItem")]
unsafe impl NSCopying for NSTrackingSeparatorToolbarItem {}

#[cfg(feature = "AppKit_NSToolbarItem")]
unsafe impl NSObjectProtocol for NSTrackingSeparatorToolbarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSSplitView",
            feature = "AppKit_NSToolbar",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other trackingSeparatorToolbarItemWithIdentifier:splitView:dividerIndex:)]
        pub unsafe fn trackingSeparatorToolbarItemWithIdentifier_splitView_dividerIndex(
            identifier: &NSToolbarItemIdentifier,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSSplitView",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSSplitView",
            feature = "AppKit_NSView"
        ))]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, split_view: &NSSplitView);

        #[method(dividerIndex)]
        pub unsafe fn dividerIndex(&self) -> NSInteger;

        #[method(setDividerIndex:)]
        pub unsafe fn setDividerIndex(&self, divider_index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[cfg(feature = "AppKit_NSToolbar")]
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Allocated<Self>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
