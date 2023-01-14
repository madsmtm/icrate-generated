//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSToolbarIdentifier = NSString;

typed_extensible_enum!(
    pub type NSToolbarItemIdentifier = NSString;
);

typed_enum!(
    pub type NSToolbarUserInfoKey = NSString;
);

extern_static!(NSToolbarItemKey: &'static NSToolbarUserInfoKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSToolbarDisplayMode {
        NSToolbarDisplayModeDefault = 0,
        NSToolbarDisplayModeIconAndLabel = 1,
        NSToolbarDisplayModeIconOnly = 2,
        NSToolbarDisplayModeLabelOnly = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSToolbarSizeMode {
        NSToolbarSizeModeDefault = 0,
        NSToolbarSizeModeRegular = 1,
        NSToolbarSizeModeSmall = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSToolbar")]
    pub struct NSToolbar;

    #[cfg(feature = "AppKit_NSToolbar")]
    unsafe impl ClassType for NSToolbar {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSToolbar")]
    unsafe impl NSToolbar {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSToolbarIdentifier,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(insertItemWithItemIdentifier:atIndex:)]
        pub unsafe fn insertItemWithItemIdentifier_atIndex(
            &self,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        );

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSToolbarDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSToolbarDelegate>);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method(runCustomizationPalette:)]
        pub unsafe fn runCustomizationPalette(&self, sender: Option<&Object>);

        #[method(customizationPaletteIsRunning)]
        pub unsafe fn customizationPaletteIsRunning(&self) -> bool;

        #[method(displayMode)]
        pub unsafe fn displayMode(&self) -> NSToolbarDisplayMode;

        #[method(setDisplayMode:)]
        pub unsafe fn setDisplayMode(&self, display_mode: NSToolbarDisplayMode);

        #[method_id(@__retain_semantics Other selectedItemIdentifier)]
        pub unsafe fn selectedItemIdentifier(&self) -> Option<Id<NSToolbarItemIdentifier, Shared>>;

        #[method(setSelectedItemIdentifier:)]
        pub unsafe fn setSelectedItemIdentifier(
            &self,
            selected_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        #[method(sizeMode)]
        pub unsafe fn sizeMode(&self) -> NSToolbarSizeMode;

        #[method(setSizeMode:)]
        pub unsafe fn setSizeMode(&self, size_mode: NSToolbarSizeMode);

        #[method(showsBaselineSeparator)]
        pub unsafe fn showsBaselineSeparator(&self) -> bool;

        #[method(setShowsBaselineSeparator:)]
        pub unsafe fn setShowsBaselineSeparator(&self, shows_baseline_separator: bool);

        #[method(allowsUserCustomization)]
        pub unsafe fn allowsUserCustomization(&self) -> bool;

        #[method(setAllowsUserCustomization:)]
        pub unsafe fn setAllowsUserCustomization(&self, allows_user_customization: bool);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSToolbarIdentifier, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbarItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<NSToolbarItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbarItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other visibleItems)]
        pub unsafe fn visibleItems(&self) -> Option<Id<NSArray<NSToolbarItem>, Shared>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other centeredItemIdentifiers)]
        pub unsafe fn centeredItemIdentifiers(&self) -> Id<NSSet<NSToolbarItemIdentifier>, Shared>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method(setCenteredItemIdentifiers:)]
        pub unsafe fn setCenteredItemIdentifiers(
            &self,
            centered_item_identifiers: &NSSet<NSToolbarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other centeredItemIdentifier)]
        pub unsafe fn centeredItemIdentifier(&self) -> Option<Id<NSToolbarItemIdentifier, Shared>>;

        #[method(setCenteredItemIdentifier:)]
        pub unsafe fn setCenteredItemIdentifier(
            &self,
            centered_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        #[method(autosavesConfiguration)]
        pub unsafe fn autosavesConfiguration(&self) -> bool;

        #[method(setAutosavesConfiguration:)]
        pub unsafe fn setAutosavesConfiguration(&self, autosaves_configuration: bool);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setConfigurationFromDictionary:)]
        pub unsafe fn setConfigurationFromDictionary(
            &self,
            config_dict: &NSDictionary<NSString, Object>,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other configurationDictionary)]
        pub unsafe fn configurationDictionary(&self) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[method(validateVisibleItems)]
        pub unsafe fn validateVisibleItems(&self);

        #[method(allowsExtensionItems)]
        pub unsafe fn allowsExtensionItems(&self) -> bool;

        #[method(setAllowsExtensionItems:)]
        pub unsafe fn setAllowsExtensionItems(&self, allows_extension_items: bool);
    }
);

extern_protocol!(
    pub struct NSToolbarDelegate;

    unsafe impl ProtocolType for NSToolbarDelegate {
        #[cfg(all(feature = "AppKit_NSToolbar", feature = "AppKit_NSToolbarItem"))]
        #[optional]
        #[method_id(@__retain_semantics Other toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
        pub unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Id<NSToolbarItem, Shared>>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[optional]
        #[method_id(@__retain_semantics Other toolbarDefaultItemIdentifiers:)]
        pub unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[optional]
        #[method_id(@__retain_semantics Other toolbarAllowedItemIdentifiers:)]
        pub unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[optional]
        #[method_id(@__retain_semantics Other toolbarSelectableItemIdentifiers:)]
        pub unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSSet"))]
        #[optional]
        #[method_id(@__retain_semantics Other toolbarImmovableItemIdentifiers:)]
        pub unsafe fn toolbarImmovableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSSet<NSToolbarItemIdentifier>, Shared>;

        #[cfg(feature = "AppKit_NSToolbar")]
        #[optional]
        #[method(toolbar:itemIdentifier:canBeInsertedAtIndex:)]
        pub unsafe fn toolbar_itemIdentifier_canBeInsertedAtIndex(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        ) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(toolbarWillAddItem:)]
        pub unsafe fn toolbarWillAddItem(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(toolbarDidRemoveItem:)]
        pub unsafe fn toolbarDidRemoveItem(&self, notification: &NSNotification);
    }
);

extern_static!(NSToolbarWillAddItemNotification: &'static NSNotificationName);

extern_static!(NSToolbarDidRemoveItemNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSToolbar")]
    unsafe impl NSToolbar {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other fullScreenAccessoryView)]
        pub unsafe fn fullScreenAccessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setFullScreenAccessoryView:)]
        pub unsafe fn setFullScreenAccessoryView(
            &self,
            full_screen_accessory_view: Option<&NSView>,
        );

        #[method(fullScreenAccessoryViewMinHeight)]
        pub unsafe fn fullScreenAccessoryViewMinHeight(&self) -> CGFloat;

        #[method(setFullScreenAccessoryViewMinHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMinHeight(
            &self,
            full_screen_accessory_view_min_height: CGFloat,
        );

        #[method(fullScreenAccessoryViewMaxHeight)]
        pub unsafe fn fullScreenAccessoryViewMaxHeight(&self) -> CGFloat;

        #[method(setFullScreenAccessoryViewMaxHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMaxHeight(
            &self,
            full_screen_accessory_view_max_height: CGFloat,
        );
    }
);
