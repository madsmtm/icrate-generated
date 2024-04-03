//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    pub struct NSSwitch;

    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for NSSwitch {
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
unsafe impl NSAccessibility for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityButton for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilitySwitch for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSCoding for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for NSSwitch {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSwitch {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSSwitch {
        #[cfg(feature = "AppKit_NSCell")]
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSSwitch {
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
    unsafe impl NSSwitch {
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
    unsafe impl NSSwitch {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
