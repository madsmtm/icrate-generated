//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMenuItem")]
    pub struct NSMenuItem;

    #[cfg(feature = "AppKit_NSMenuItem")]
    unsafe impl ClassType for NSMenuItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSMenuItem")]
    unsafe impl NSMenuItem {
        #[method(usesUserKeyEquivalents)]
        pub unsafe fn usesUserKeyEquivalents() -> bool;

        #[method(setUsesUserKeyEquivalents:)]
        pub unsafe fn setUsesUserKeyEquivalents(uses_user_key_equivalents: bool);

        #[method_id(@__retain_semantics Other separatorItem)]
        pub unsafe fn separatorItem() -> Id<NSMenuItem, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithTitle:action:keyEquivalent:)]
        pub unsafe fn initWithTitle_action_keyEquivalent(
            this: Option<Allocated<Self>>,
            string: &NSString,
            selector: Option<Sel>,
            char_code: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(hasSubmenu)]
        pub unsafe fn hasSubmenu(&self) -> bool;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other submenu)]
        pub unsafe fn submenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setSubmenu:)]
        pub unsafe fn setSubmenu(&self, submenu: Option<&NSMenu>);

        #[method_id(@__retain_semantics Other parentItem)]
        pub unsafe fn parentItem(&self) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[method(isSeparatorItem)]
        pub unsafe fn isSeparatorItem(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, key_equivalent: &NSString);

        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;

        #[method(setKeyEquivalentModifierMask:)]
        pub unsafe fn setKeyEquivalentModifierMask(
            &self,
            key_equivalent_modifier_mask: NSEventModifierFlags,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userKeyEquivalent)]
        pub unsafe fn userKeyEquivalent(&self) -> Id<NSString, Shared>;

        #[method(allowsKeyEquivalentWhenHidden)]
        pub unsafe fn allowsKeyEquivalentWhenHidden(&self) -> bool;

        #[method(setAllowsKeyEquivalentWhenHidden:)]
        pub unsafe fn setAllowsKeyEquivalentWhenHidden(
            &self,
            allows_key_equivalent_when_hidden: bool,
        );

        #[method(allowsAutomaticKeyEquivalentLocalization)]
        pub unsafe fn allowsAutomaticKeyEquivalentLocalization(&self) -> bool;

        #[method(setAllowsAutomaticKeyEquivalentLocalization:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentLocalization(
            &self,
            allows_automatic_key_equivalent_localization: bool,
        );

        #[method(allowsAutomaticKeyEquivalentMirroring)]
        pub unsafe fn allowsAutomaticKeyEquivalentMirroring(&self) -> bool;

        #[method(setAllowsAutomaticKeyEquivalentMirroring:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentMirroring(
            &self,
            allows_automatic_key_equivalent_mirroring: bool,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other onStateImage)]
        pub unsafe fn onStateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setOnStateImage:)]
        pub unsafe fn setOnStateImage(&self, on_state_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other offStateImage)]
        pub unsafe fn offStateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setOffStateImage:)]
        pub unsafe fn setOffStateImage(&self, off_state_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other mixedStateImage)]
        pub unsafe fn mixedStateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setMixedStateImage:)]
        pub unsafe fn setMixedStateImage(&self, mixed_state_image: Option<&NSImage>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isAlternate)]
        pub unsafe fn isAlternate(&self) -> bool;

        #[method(setAlternate:)]
        pub unsafe fn setAlternate(&self, alternate: bool);

        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        #[method(setIndentationLevel:)]
        pub unsafe fn setIndentationLevel(&self, indentation_level: NSInteger);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&Object>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(isHiddenOrHasHiddenAncestor)]
        pub unsafe fn isHiddenOrHasHiddenAncestor(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);
    }
);

extern_methods!(
    /// NSViewEnclosingMenuItem
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other enclosingMenuItem)]
        pub unsafe fn enclosingMenuItem(&self) -> Option<Id<NSMenuItem, Shared>>;
    }
);

extern_static!(NSMenuItemImportFromDeviceIdentifier: &'static NSUserInterfaceItemIdentifier);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSMenuItem")]
    unsafe impl NSMenuItem {
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);

        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other mnemonic)]
        pub unsafe fn mnemonic(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: &NSString);
    }
);
