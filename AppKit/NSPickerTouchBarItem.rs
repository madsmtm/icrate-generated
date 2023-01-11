//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPickerTouchBarItemSelectionMode {
        NSPickerTouchBarItemSelectionModeSelectOne = 0,
        NSPickerTouchBarItemSelectionModeSelectAny = 1,
        NSPickerTouchBarItemSelectionModeMomentary = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPickerTouchBarItemControlRepresentation {
        NSPickerTouchBarItemControlRepresentationAutomatic = 0,
        NSPickerTouchBarItemControlRepresentationExpanded = 1,
        NSPickerTouchBarItemControlRepresentationCollapsed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPickerTouchBarItem")]
    pub struct NSPickerTouchBarItem;

    #[cfg(feature = "AppKit_NSPickerTouchBarItem")]
    unsafe impl ClassType for NSPickerTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPickerTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pickerTouchBarItemWithIdentifier:labels:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_labels_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            labels: &NSArray<NSString>,
            selectionMode: NSPickerTouchBarItemSelectionMode,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other pickerTouchBarItemWithIdentifier:images:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_images_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            images: &NSArray<NSImage>,
            selectionMode: NSPickerTouchBarItemSelectionMode,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSPickerTouchBarItemControlRepresentation;

        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            controlRepresentation: NSPickerTouchBarItemControlRepresentation,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsedRepresentationLabel: &NSString,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsedRepresentationImage: Option<&NSImage>,
        );

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other selectionColor)]
        pub unsafe fn selectionColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setSelectionColor:)]
        pub unsafe fn setSelectionColor(&self, selectionColor: Option<&NSColor>);

        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSPickerTouchBarItemSelectionMode;

        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selectionMode: NSPickerTouchBarItemSelectionMode);

        #[method(numberOfOptions)]
        pub unsafe fn numberOfOptions(&self) -> NSInteger;

        #[method(setNumberOfOptions:)]
        pub unsafe fn setNumberOfOptions(&self, numberOfOptions: NSInteger);

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:atIndex:)]
        pub unsafe fn setImage_atIndex(&self, image: Option<&NSImage>, index: NSInteger);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageAtIndex:)]
        pub unsafe fn imageAtIndex(&self, index: NSInteger) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:atIndex:)]
        pub unsafe fn setLabel_atIndex(&self, label: &NSString, index: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelAtIndex:)]
        pub unsafe fn labelAtIndex(&self, index: NSInteger) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(setEnabled:atIndex:)]
        pub unsafe fn setEnabled_atIndex(&self, enabled: bool, index: NSInteger);

        #[method(isEnabledAtIndex:)]
        pub unsafe fn isEnabledAtIndex(&self, index: NSInteger) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSPickerTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
