//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSButtonType {
        NSButtonTypeMomentaryLight = 0,
        NSButtonTypePushOnPushOff = 1,
        NSButtonTypeToggle = 2,
        NSButtonTypeSwitch = 3,
        NSButtonTypeRadio = 4,
        NSButtonTypeMomentaryChange = 5,
        NSButtonTypeOnOff = 6,
        NSButtonTypeMomentaryPushIn = 7,
        NSButtonTypeAccelerator = 8,
        NSButtonTypeMultiLevelAccelerator = 9,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBezelStyle {
        NSBezelStyleRounded = 1,
        NSBezelStyleRegularSquare = 2,
        NSBezelStyleDisclosure = 5,
        NSBezelStyleShadowlessSquare = 6,
        NSBezelStyleCircular = 7,
        NSBezelStyleTexturedSquare = 8,
        NSBezelStyleHelpButton = 9,
        NSBezelStyleSmallSquare = 10,
        NSBezelStyleTexturedRounded = 11,
        NSBezelStyleRoundRect = 12,
        NSBezelStyleRecessed = 13,
        NSBezelStyleRoundedDisclosure = 14,
        NSBezelStyleInline = 15,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSButtonCell")]
    pub struct NSButtonCell;

    #[cfg(feature = "AppKit_NSButtonCell")]
    unsafe impl ClassType for NSButtonCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSButtonCell")]
    unsafe impl NSButtonCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSBezelStyle);

        #[method(setButtonType:)]
        pub unsafe fn setButtonType(&self, r#type: NSButtonType);

        #[method(highlightsBy)]
        pub unsafe fn highlightsBy(&self) -> NSCellStyleMask;

        #[method(setHighlightsBy:)]
        pub unsafe fn setHighlightsBy(&self, highlights_by: NSCellStyleMask);

        #[method(showsStateBy)]
        pub unsafe fn showsStateBy(&self) -> NSCellStyleMask;

        #[method(setShowsStateBy:)]
        pub unsafe fn setShowsStateBy(&self, shows_state_by: NSCellStyleMask);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedAlternateTitle)]
        pub unsafe fn attributedAlternateTitle(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedAlternateTitle:)]
        pub unsafe fn setAttributedAlternateTitle(
            &self,
            attributed_alternate_title: &NSAttributedString,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);

        #[method(imagePosition)]
        pub unsafe fn imagePosition(&self) -> NSCellImagePosition;

        #[method(setImagePosition:)]
        pub unsafe fn setImagePosition(&self, image_position: NSCellImagePosition);

        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

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

        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(imageDimsWhenDisabled)]
        pub unsafe fn imageDimsWhenDisabled(&self) -> bool;

        #[method(setImageDimsWhenDisabled:)]
        pub unsafe fn setImageDimsWhenDisabled(&self, image_dims_when_disabled: bool);

        #[method(showsBorderOnlyWhileMouseInside)]
        pub unsafe fn showsBorderOnlyWhileMouseInside(&self) -> bool;

        #[method(setShowsBorderOnlyWhileMouseInside:)]
        pub unsafe fn setShowsBorderOnlyWhileMouseInside(
            &self,
            shows_border_only_while_mouse_inside: bool,
        );

        #[cfg(feature = "AppKit_NSSound")]
        #[method_id(@__retain_semantics Other sound)]
        pub unsafe fn sound(&self) -> Option<Id<NSSound, Shared>>;

        #[cfg(feature = "AppKit_NSSound")]
        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&NSSound>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(setPeriodicDelay:interval:)]
        pub unsafe fn setPeriodicDelay_interval(&self, delay: c_float, interval: c_float);

        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawBezelWithFrame:inView:)]
        pub unsafe fn drawBezelWithFrame_inView(&self, frame: NSRect, control_view: &NSView);

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSView"))]
        #[method(drawImage:withFrame:inView:)]
        pub unsafe fn drawImage_withFrame_inView(
            &self,
            image: &NSImage,
            frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSAttributedString"))]
        #[method(drawTitle:withFrame:inView:)]
        pub unsafe fn drawTitle_withFrame_inView(
            &self,
            title: &NSAttributedString,
            frame: NSRect,
            control_view: &NSView,
        ) -> NSRect;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSGradientType {
        NSGradientNone = 0,
        NSGradientConcaveWeak = 1,
        NSGradientConcaveStrong = 2,
        NSGradientConvexWeak = 3,
        NSGradientConvexStrong = 4,
    }
);

extern_static!(NSMomentaryLightButton: NSButtonType = NSButtonTypeMomentaryLight);

extern_static!(NSPushOnPushOffButton: NSButtonType = NSButtonTypePushOnPushOff);

extern_static!(NSToggleButton: NSButtonType = NSButtonTypeToggle);

extern_static!(NSSwitchButton: NSButtonType = NSButtonTypeSwitch);

extern_static!(NSRadioButton: NSButtonType = NSButtonTypeRadio);

extern_static!(NSMomentaryChangeButton: NSButtonType = NSButtonTypeMomentaryChange);

extern_static!(NSOnOffButton: NSButtonType = NSButtonTypeOnOff);

extern_static!(NSMomentaryPushInButton: NSButtonType = NSButtonTypeMomentaryPushIn);

extern_static!(NSAcceleratorButton: NSButtonType = NSButtonTypeAccelerator);

extern_static!(NSMultiLevelAcceleratorButton: NSButtonType = NSButtonTypeMultiLevelAccelerator);

extern_static!(NSMomentaryPushButton: NSButtonType = NSButtonTypeMomentaryLight);

extern_static!(NSMomentaryLight: NSButtonType = NSButtonTypeMomentaryPushIn);

extern_static!(NSRoundedBezelStyle: NSBezelStyle = NSBezelStyleRounded);

extern_static!(NSRegularSquareBezelStyle: NSBezelStyle = NSBezelStyleRegularSquare);

extern_static!(NSDisclosureBezelStyle: NSBezelStyle = NSBezelStyleDisclosure);

extern_static!(NSShadowlessSquareBezelStyle: NSBezelStyle = NSBezelStyleShadowlessSquare);

extern_static!(NSCircularBezelStyle: NSBezelStyle = NSBezelStyleCircular);

extern_static!(NSTexturedSquareBezelStyle: NSBezelStyle = NSBezelStyleTexturedSquare);

extern_static!(NSHelpButtonBezelStyle: NSBezelStyle = NSBezelStyleHelpButton);

extern_static!(NSSmallSquareBezelStyle: NSBezelStyle = NSBezelStyleSmallSquare);

extern_static!(NSTexturedRoundedBezelStyle: NSBezelStyle = NSBezelStyleTexturedRounded);

extern_static!(NSRoundRectBezelStyle: NSBezelStyle = NSBezelStyleRoundRect);

extern_static!(NSRecessedBezelStyle: NSBezelStyle = NSBezelStyleRecessed);

extern_static!(NSRoundedDisclosureBezelStyle: NSBezelStyle = NSBezelStyleRoundedDisclosure);

extern_static!(NSInlineBezelStyle: NSBezelStyle = NSBezelStyleInline);

extern_static!(NSSmallIconButtonBezelStyle: NSBezelStyle = 2);

extern_static!(NSThickSquareBezelStyle: NSBezelStyle = 3);

extern_static!(NSThickerSquareBezelStyle: NSBezelStyle = 4);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSButtonCell")]
    unsafe impl NSButtonCell {
        #[method(gradientType)]
        pub unsafe fn gradientType(&self) -> NSGradientType;

        #[method(setGradientType:)]
        pub unsafe fn setGradientType(&self, gradient_type: NSGradientType);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlternateTitleWithMnemonic:)]
        pub unsafe fn setAlternateTitleWithMnemonic(
            &self,
            string_with_ampersand: Option<&NSString>,
        );

        #[method(setAlternateMnemonicLocation:)]
        pub unsafe fn setAlternateMnemonicLocation(&self, location: NSUInteger);

        #[method(alternateMnemonicLocation)]
        pub unsafe fn alternateMnemonicLocation(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alternateMnemonic)]
        pub unsafe fn alternateMnemonic(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other keyEquivalentFont)]
        pub unsafe fn keyEquivalentFont(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setKeyEquivalentFont:)]
        pub unsafe fn setKeyEquivalentFont(&self, key_equivalent_font: Option<&NSFont>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKeyEquivalentFont:size:)]
        pub unsafe fn setKeyEquivalentFont_size(&self, font_name: &NSString, font_size: CGFloat);
    }
);
