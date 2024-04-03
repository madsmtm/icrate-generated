//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityElement;

    unsafe impl ClassType for NSAccessibilityElement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityProtocols")]
unsafe impl NSAccessibility for NSAccessibilityElement {}

unsafe impl NSObjectProtocol for NSAccessibilityElement {}

extern_methods!(
    unsafe impl NSAccessibilityElement {
        #[cfg(feature = "AppKit_NSAccessibilityConstants")]
        #[method_id(@__retain_semantics Other accessibilityElementWithRole:frame:label:parent:)]
        pub unsafe fn accessibilityElementWithRole_frame_label_parent(
            role: &NSAccessibilityRole,
            frame: NSRect,
            label: Option<&NSString>,
            parent: Option<&AnyObject>,
        ) -> Id<AnyObject>;

        #[method(accessibilityAddChildElement:)]
        pub unsafe fn accessibilityAddChildElement(&self, child_element: &NSAccessibilityElement);

        #[method(accessibilityFrameInParentSpace)]
        pub unsafe fn accessibilityFrameInParentSpace(&self) -> NSRect;

        #[method(setAccessibilityFrameInParentSpace:)]
        pub unsafe fn setAccessibilityFrameInParentSpace(
            &self,
            accessibility_frame_in_parent_space: NSRect,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAccessibilityElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
