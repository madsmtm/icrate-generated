//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSShadow;

    unsafe impl ClassType for NSShadow {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSShadow {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSShadow {}

unsafe impl NSObjectProtocol for NSShadow {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSShadow {}

extern_methods!(
    unsafe impl NSShadow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(shadowOffset)]
        pub unsafe fn shadowOffset(&self) -> NSSize;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setShadowOffset:)]
        pub unsafe fn setShadowOffset(&self, shadow_offset: NSSize);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(shadowBlurRadius)]
        pub unsafe fn shadowBlurRadius(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setShadowBlurRadius:)]
        pub unsafe fn setShadowBlurRadius(&self, shadow_blur_radius: CGFloat);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadow_color: Option<&NSColor>);

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSShadow {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
