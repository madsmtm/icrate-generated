//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    pub struct NSGroupTouchBarItem;

    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl ClassType for NSGroupTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl NSGroupTouchBarItem {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other groupItemWithIdentifier:items:)]
        pub unsafe fn groupItemWithIdentifier_items(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other groupItemWithIdentifier:items:allowedCompressionOptions:)]
        pub unsafe fn groupItemWithIdentifier_items_allowedCompressionOptions(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
            allowedCompressionOptions: &NSUserInterfaceCompressionOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other alertStyleGroupItemWithIdentifier:)]
        pub unsafe fn alertStyleGroupItemWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other groupTouchBar)]
        pub unsafe fn groupTouchBar(&self) -> Id<NSTouchBar, Shared>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method(setGroupTouchBar:)]
        pub unsafe fn setGroupTouchBar(&self, groupTouchBar: &NSTouchBar);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);

        #[method(groupUserInterfaceLayoutDirection)]
        pub unsafe fn groupUserInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setGroupUserInterfaceLayoutDirection:)]
        pub unsafe fn setGroupUserInterfaceLayoutDirection(
            &self,
            groupUserInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );

        #[method(prefersEqualWidths)]
        pub unsafe fn prefersEqualWidths(&self) -> bool;

        #[method(setPrefersEqualWidths:)]
        pub unsafe fn setPrefersEqualWidths(&self, prefersEqualWidths: bool);

        #[method(preferredItemWidth)]
        pub unsafe fn preferredItemWidth(&self) -> CGFloat;

        #[method(setPreferredItemWidth:)]
        pub unsafe fn setPreferredItemWidth(&self, preferredItemWidth: CGFloat);

        #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
        #[method_id(@__retain_semantics Other effectiveCompressionOptions)]
        pub unsafe fn effectiveCompressionOptions(
            &self,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other prioritizedCompressionOptions)]
        pub unsafe fn prioritizedCompressionOptions(
            &self,
        ) -> Id<NSArray<NSUserInterfaceCompressionOptions>, Shared>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(setPrioritizedCompressionOptions:)]
        pub unsafe fn setPrioritizedCompressionOptions(
            &self,
            prioritizedCompressionOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl NSGroupTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
