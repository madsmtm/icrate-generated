//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutGuide;

    unsafe impl ClassType for NSLayoutGuide {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSLayoutGuide {}

unsafe impl NSObjectProtocol for NSLayoutGuide {}

#[cfg(feature = "AppKit_NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSLayoutGuide {}

extern_methods!(
    unsafe impl NSLayoutGuide {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other owningView)]
        pub unsafe fn owningView(&self, mtm: MainThreadMarker) -> Option<Id<NSView>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owning_view: Option<&NSView>);

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceItemIdentification",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceItemIdentification",
            feature = "Foundation_NSString"
        ))]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutGuide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSLayoutGuideSupport
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(addLayoutGuide:)]
        pub unsafe fn addLayoutGuide(&self, guide: &NSLayoutGuide);

        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(removeLayoutGuide:)]
        pub unsafe fn removeLayoutGuide(&self, guide: &NSLayoutGuide);

        #[cfg(all(feature = "AppKit_NSLayoutGuide", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other layoutGuides)]
        pub unsafe fn layoutGuides(&self) -> Id<NSArray<NSLayoutGuide>>;
    }
);
