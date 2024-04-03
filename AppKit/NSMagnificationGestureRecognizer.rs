//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    pub struct NSMagnificationGestureRecognizer;

    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl ClassType for NSMagnificationGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSGestureRecognizer")]
unsafe impl NSCoding for NSMagnificationGestureRecognizer {}

#[cfg(feature = "AppKit_NSGestureRecognizer")]
unsafe impl NSObjectProtocol for NSMagnificationGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSMagnificationGestureRecognizer {
        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSMagnificationGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSMagnificationGestureRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
