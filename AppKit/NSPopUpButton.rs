//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPopUpButton")]
    pub struct NSPopUpButton;

    #[cfg(feature = "AppKit_NSPopUpButton")]
    unsafe impl ClassType for NSPopUpButton {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSButton;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPopUpButton")]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics Init initWithFrame:pullsDown:)]
        pub unsafe fn initWithFrame_pullsDown(
            this: Option<Allocated<Self>>,
            buttonFrame: NSRect,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(pullsDown)]
        pub unsafe fn pullsDown(&self) -> bool;

        #[method(setPullsDown:)]
        pub unsafe fn setPullsDown(&self, pullsDown: bool);

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenablesItems: bool);

        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferredEdge: NSRectEdge);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addItemWithTitle:)]
        pub unsafe fn addItemWithTitle(&self, title: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(addItemsWithTitles:)]
        pub unsafe fn addItemsWithTitles(&self, itemTitles: &NSArray<NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(insertItemWithTitle:atIndex:)]
        pub unsafe fn insertItemWithTitle_atIndex(&self, title: &NSString, index: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeItemWithTitle:)]
        pub unsafe fn removeItemWithTitle(&self, title: &NSString);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>, Shared>;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, obj: Option<&Object>) -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&Object>,
            actionSelector: Option<Sel>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other lastItem)]
        pub unsafe fn lastItem(&self) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(selectItem:)]
        pub unsafe fn selectItem(&self, item: Option<&NSMenuItem>);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(selectItemWithTitle:)]
        pub unsafe fn selectItemWithTitle(&self, title: &NSString);

        #[method(selectItemWithTag:)]
        pub unsafe fn selectItemWithTag(&self, tag: NSInteger) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: &NSString);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other selectedItem)]
        pub unsafe fn selectedItem(&self) -> Option<Id<NSMenuItem, Shared>>;

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[method(synchronizeTitleAndSelectedItem)]
        pub unsafe fn synchronizeTitleAndSelectedItem(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other itemTitleAtIndex:)]
        pub unsafe fn itemTitleAtIndex(&self, index: NSInteger) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemTitles)]
        pub unsafe fn itemTitles(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleOfSelectedItem)]
        pub unsafe fn titleOfSelectedItem(&self) -> Option<Id<NSString, Shared>>;
    }
);

extern_static!(NSPopUpButtonWillPopUpNotification: &'static NSNotificationName);

extern_methods!(
    /// Methods declared on superclass `NSButton`
    #[cfg(feature = "AppKit_NSPopUpButton")]
    unsafe impl NSPopUpButton {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSPopUpButton")]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
