//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSTouchBarCustomizationIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBar;

    unsafe impl ClassType for NSTouchBar {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTouchBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other customizationIdentifier)]
        pub unsafe fn customizationIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarCustomizationIdentifier, Shared>>;

        #[method(setCustomizationIdentifier:)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customizationIdentifier: Option<&NSTouchBarCustomizationIdentifier>,
        );

        #[method_id(@__retain_semantics Other customizationAllowedItemIdentifiers)]
        pub unsafe fn customizationAllowedItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[method(setCustomizationAllowedItemIdentifiers:)]
        pub unsafe fn setCustomizationAllowedItemIdentifiers(
            &self,
            customizationAllowedItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other customizationRequiredItemIdentifiers)]
        pub unsafe fn customizationRequiredItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[method(setCustomizationRequiredItemIdentifiers:)]
        pub unsafe fn setCustomizationRequiredItemIdentifiers(
            &self,
            customizationRequiredItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other defaultItemIdentifiers)]
        pub unsafe fn defaultItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[method(setDefaultItemIdentifiers:)]
        pub unsafe fn setDefaultItemIdentifiers(
            &self,
            defaultItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[method_id(@__retain_semantics Other principalItemIdentifier)]
        pub unsafe fn principalItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;

        #[method(setPrincipalItemIdentifier:)]
        pub unsafe fn setPrincipalItemIdentifier(
            &self,
            principalItemIdentifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other escapeKeyReplacementItemIdentifier)]
        pub unsafe fn escapeKeyReplacementItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;

        #[method(setEscapeKeyReplacementItemIdentifier:)]
        pub unsafe fn setEscapeKeyReplacementItemIdentifier(
            &self,
            escapeKeyReplacementItemIdentifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other templateItems)]
        pub unsafe fn templateItems(&self) -> Id<NSSet<NSTouchBarItem>, Shared>;

        #[method(setTemplateItems:)]
        pub unsafe fn setTemplateItems(&self, templateItems: &NSSet<NSTouchBarItem>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTouchBarDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTouchBarDelegate>);

        #[method_id(@__retain_semantics Other itemForIdentifier:)]
        pub unsafe fn itemForIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Id<NSTouchBarItem, Shared>>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled() -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            automaticCustomizeTouchBarMenuItemEnabled: bool,
        );
    }
);

extern_protocol!(
    pub struct NSTouchBarDelegate;

    unsafe impl ProtocolType for NSTouchBarDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other touchBar:makeItemForIdentifier:)]
        pub unsafe fn touchBar_makeItemForIdentifier(
            &self,
            touchBar: &NSTouchBar,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Id<NSTouchBarItem, Shared>>;
    }
);

extern_protocol!(
    pub struct NSTouchBarProvider;

    unsafe impl ProtocolType for NSTouchBarProvider {
        #[method_id(@__retain_semantics Other touchBar)]
        pub unsafe fn touchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
    }
);

extern_methods!(
    /// NSTouchBarProvider
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Other touchBar)]
        pub unsafe fn touchBar(&self) -> Option<Id<NSTouchBar, Shared>>;

        #[method(setTouchBar:)]
        pub unsafe fn setTouchBar(&self, touchBar: Option<&NSTouchBar>);

        #[method_id(@__retain_semantics Other makeTouchBar)]
        pub unsafe fn makeTouchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
    }
);

extern_methods!(
    /// NSTouchBarCustomization
    unsafe impl NSApplication {
        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            &self,
            automaticCustomizeTouchBarMenuItemEnabled: bool,
        );

        #[method(toggleTouchBarCustomizationPalette:)]
        pub unsafe fn toggleTouchBarCustomizationPalette(&self, sender: Option<&Object>);
    }
);
