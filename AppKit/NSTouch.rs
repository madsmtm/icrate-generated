//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchPhase(pub NSUInteger);
impl NSTouchPhase {
    #[doc(alias = "NSTouchPhaseBegan")]
    pub const Began: Self = Self(1 << 0);
    #[doc(alias = "NSTouchPhaseMoved")]
    pub const Moved: Self = Self(1 << 1);
    #[doc(alias = "NSTouchPhaseStationary")]
    pub const Stationary: Self = Self(1 << 2);
    #[doc(alias = "NSTouchPhaseEnded")]
    pub const Ended: Self = Self(1 << 3);
    #[doc(alias = "NSTouchPhaseCancelled")]
    pub const Cancelled: Self = Self(1 << 4);
    #[doc(alias = "NSTouchPhaseTouching")]
    pub const Touching: Self =
        Self(NSTouchPhase::Began.0 | NSTouchPhase::Moved.0 | NSTouchPhase::Stationary.0);
    #[doc(alias = "NSTouchPhaseAny")]
    pub const Any: Self = Self(NSUIntegerMax as _);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTouchPhase {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTouchPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchType(pub NSInteger);
impl NSTouchType {
    #[doc(alias = "NSTouchTypeDirect")]
    pub const Direct: Self = Self(0);
    #[doc(alias = "NSTouchTypeIndirect")]
    pub const Indirect: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTouchType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTouchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchTypeMask(pub NSUInteger);
impl NSTouchTypeMask {
    #[doc(alias = "NSTouchTypeMaskDirect")]
    pub const Direct: Self = Self(1 << NSTouchType::Direct.0);
    #[doc(alias = "NSTouchTypeMaskIndirect")]
    pub const Indirect: Self = Self(1 << NSTouchType::Indirect.0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTouchTypeMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTouchTypeMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn NSTouchTypeMaskFromType(r#type: NSTouchType,) -> NSTouchTypeMask;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouch;

    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

unsafe impl Send for NSTouch {}

unsafe impl Sync for NSTouch {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSTouch {}

unsafe impl NSObjectProtocol for NSTouch {}

extern_methods!(
    unsafe impl NSTouch {
        #[cfg(feature = "Foundation_NSObject")]
        #[method_id(@__retain_semantics Other identity)]
        pub unsafe fn identity(&self) -> Id<TodoProtocols>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSTouchBar
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSTouchType;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
