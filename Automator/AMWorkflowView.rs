//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct AMWorkflowView;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for AMWorkflowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for AMWorkflowView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for AMWorkflowView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for AMWorkflowView {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl AMWorkflowView {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(all(
            feature = "AppKit_NSController",
            feature = "Automator_AMWorkflowController"
        ))]
        #[method_id(@__retain_semantics Other workflowController)]
        pub unsafe fn workflowController(&self) -> Option<Id<AMWorkflowController>>;

        #[cfg(all(
            feature = "AppKit_NSController",
            feature = "Automator_AMWorkflowController"
        ))]
        #[method(setWorkflowController:)]
        pub unsafe fn setWorkflowController(
            &self,
            workflow_controller: Option<&AMWorkflowController>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl AMWorkflowView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl AMWorkflowView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
