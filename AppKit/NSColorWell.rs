//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorWellStyle(pub NSInteger);
impl NSColorWellStyle {
    #[doc(alias = "NSColorWellStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSColorWellStyleMinimal")]
    pub const Minimal: Self = Self(1);
    #[doc(alias = "NSColorWellStyleExpanded")]
    pub const Expanded: Self = Self(2);
}

unsafe impl Encode for NSColorWellStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorWellStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    pub struct NSColorWell;

    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for NSColorWell {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSCoding for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for NSColorWell {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSColorWell {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Other colorWellWithStyle:)]
        pub unsafe fn colorWellWithStyle(
            style: NSColorWellStyle,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(activate:)]
        pub unsafe fn activate(&self, exclusive: bool);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(drawWellInside:)]
        pub unsafe fn drawWellInside(&self, inside_rect: NSRect);

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(takeColorFrom:)]
        pub unsafe fn takeColorFrom(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method(colorWellStyle)]
        pub unsafe fn colorWellStyle(&self) -> NSColorWellStyle;

        #[method(setColorWellStyle:)]
        pub unsafe fn setColorWellStyle(&self, color_well_style: NSColorWellStyle);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other pulldownTarget)]
        pub unsafe fn pulldownTarget(&self) -> Option<Id<AnyObject>>;

        #[method(setPulldownTarget:)]
        pub unsafe fn setPulldownTarget(&self, pulldown_target: Option<&AnyObject>);

        #[method(pulldownAction)]
        pub unsafe fn pulldownAction(&self) -> Option<Sel>;

        #[method(setPulldownAction:)]
        pub unsafe fn setPulldownAction(&self, pulldown_action: Option<Sel>);

        #[method(supportsAlpha)]
        pub unsafe fn supportsAlpha(&self) -> bool;

        #[method(setSupportsAlpha:)]
        pub unsafe fn setSupportsAlpha(&self, supports_alpha: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
