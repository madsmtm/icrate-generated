//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_HORIZONTAL: c_uint = 0;
#[deprecated]
pub const DOM_VERTICAL: c_uint = 1;
#[deprecated]
pub const DOM_BOTH: c_uint = 2;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "WebKit_DOMEvent",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMOverflowEvent;

    #[cfg(all(
        feature = "WebKit_DOMEvent",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMOverflowEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "WebKit_DOMEvent",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMOverflowEvent {}

#[cfg(all(
    feature = "WebKit_DOMEvent",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMOverflowEvent {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMEvent",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[deprecated]
        #[method(orient)]
        pub unsafe fn orient(&self) -> c_ushort;

        #[deprecated]
        #[method(horizontalOverflow)]
        pub unsafe fn horizontalOverflow(&self) -> bool;

        #[deprecated]
        #[method(verticalOverflow)]
        pub unsafe fn verticalOverflow(&self) -> bool;

        #[deprecated]
        #[method(initOverflowEvent:horizontalOverflow:verticalOverflow:)]
        pub unsafe fn initOverflowEvent_horizontalOverflow_verticalOverflow(
            &self,
            orient: c_ushort,
            horizontal_overflow: bool,
            vertical_overflow: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "WebKit_DOMEvent",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "WebKit_DOMEvent",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
