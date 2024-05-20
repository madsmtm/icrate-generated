//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactPicker;

    unsafe impl ClassType for CNContactPicker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CNContactPicker {}

extern_methods!(
    unsafe impl CNContactPicker {
        #[method_id(@__retain_semantics Other displayedKeys)]
        pub unsafe fn displayedKeys(&self) -> Id<NSArray<NSString>>;

        #[method(setDisplayedKeys:)]
        pub unsafe fn setDisplayedKeys(&self, displayed_keys: &NSArray<NSString>);

        #[cfg(feature = "CNContactPickerDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CNContactPickerDelegate>>>;

        #[cfg(feature = "CNContactPickerDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CNContactPickerDelegate>>,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[method(close)]
        pub unsafe fn close(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactPicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
