//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSToolbarItemVisibilityPriority = NSInteger;

pub static NSToolbarItemVisibilityPriorityStandard: NSToolbarItemVisibilityPriority = 0;

pub static NSToolbarItemVisibilityPriorityLow: NSToolbarItemVisibilityPriority = -1000;

pub static NSToolbarItemVisibilityPriorityHigh: NSToolbarItemVisibilityPriority = 1000;

pub static NSToolbarItemVisibilityPriorityUser: NSToolbarItemVisibilityPriority = 2000;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSToolbarItem;

    unsafe impl ClassType for NSToolbarItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCopying for NSToolbarItem {}

unsafe impl NSObjectProtocol for NSToolbarItem {}

extern_methods!(
    unsafe impl NSToolbarItem {
        #[cfg(feature = "AppKit_NSToolbar")]
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Allocated<Self>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSToolbar")]
        #[method_id(@__retain_semantics Other itemIdentifier)]
        pub unsafe fn itemIdentifier(&self) -> Id<NSToolbarItemIdentifier>;

        #[cfg(feature = "AppKit_NSToolbar")]
        #[method_id(@__retain_semantics Other toolbar)]
        pub unsafe fn toolbar(&self) -> Option<Id<NSToolbar>>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method_id(@__retain_semantics Other paletteLabel)]
        pub unsafe fn paletteLabel(&self) -> Id<NSString>;

        #[method(setPaletteLabel:)]
        pub unsafe fn setPaletteLabel(&self, palette_label: &NSString);

        #[method_id(@__retain_semantics Other possibleLabels)]
        pub unsafe fn possibleLabels(&self) -> Id<NSSet<NSString>>;

        #[method(setPossibleLabels:)]
        pub unsafe fn setPossibleLabels(&self, possible_labels: &NSSet<NSString>);

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString>>;

        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other menuFormRepresentation)]
        pub unsafe fn menuFormRepresentation(&self) -> Option<Id<NSMenuItem>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(setMenuFormRepresentation:)]
        pub unsafe fn setMenuFormRepresentation(
            &self,
            menu_form_representation: Option<&NSMenuItem>,
        );

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(isNavigational)]
        pub unsafe fn isNavigational(&self) -> bool;

        #[method(setNavigational:)]
        pub unsafe fn setNavigational(&self, navigational: bool);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[deprecated = "This property is no longer recommended. Instead, let the system automatically measure the size of the view using constraints."]
        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;

        #[deprecated = "This property is no longer recommended. Instead, let the system automatically measure the size of the view using constraints."]
        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, min_size: NSSize);

        #[deprecated = "This property is no longer recommended. Instead, let the system automatically measure the size of the view using constraints."]
        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;

        #[deprecated = "This property is no longer recommended. Instead, let the system automatically measure the size of the view using constraints."]
        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, max_size: NSSize);

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSToolbarItemVisibilityPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(
            &self,
            visibility_priority: NSToolbarItemVisibilityPriority,
        );

        #[method(validate)]
        pub unsafe fn validate(&self);

        #[method(autovalidates)]
        pub unsafe fn autovalidates(&self) -> bool;

        #[method(setAutovalidates:)]
        pub unsafe fn setAutovalidates(&self, autovalidates: bool);

        #[method(allowsDuplicatesInToolbar)]
        pub unsafe fn allowsDuplicatesInToolbar(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSToolbarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    unsafe impl NSToolbarItem {}
);

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSMenuItemValidation for NSToolbarItem {}

#[cfg(feature = "AppKit_NSUserInterfaceValidation")]
unsafe impl NSValidatedUserInterfaceItem for NSToolbarItem {}

extern_protocol!(
    pub unsafe trait NSToolbarItemValidation: NSObjectProtocol + IsMainThreadOnly {
        #[method(validateToolbarItem:)]
        unsafe fn validateToolbarItem(&self, item: &NSToolbarItem) -> bool;
    }

    unsafe impl ProtocolType for dyn NSToolbarItemValidation {}
);

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarSpaceItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarFlexibleSpaceItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarShowColorsItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarShowFontsItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarPrintItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarToggleSidebarItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarToggleInspectorItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarCloudSharingItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarSidebarTrackingSeparatorItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarInspectorTrackingSeparatorItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarSeparatorItemIdentifier: &'static NSToolbarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSToolbar")]
    pub static NSToolbarCustomizeToolbarItemIdentifier: &'static NSToolbarItemIdentifier;
}
