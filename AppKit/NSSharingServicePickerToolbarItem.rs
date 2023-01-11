//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    pub struct NSSharingServicePickerToolbarItem;

    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl ClassType for NSSharingServicePickerToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<NSSharingServicePickerToolbarItemDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&NSSharingServicePickerToolbarItemDelegate>,
        );
    }
);

extern_protocol!(
    pub struct NSSharingServicePickerToolbarItemDelegate;

    unsafe impl ProtocolType for NSSharingServicePickerToolbarItemDelegate {
        #[method_id(@__retain_semantics Other itemsForSharingServicePickerToolbarItem:)]
        pub unsafe fn itemsForSharingServicePickerToolbarItem(
            &self,
            pickerToolbarItem: &NSSharingServicePickerToolbarItem,
        ) -> Id<NSArray, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            itemIdentifier: &NSToolbarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
