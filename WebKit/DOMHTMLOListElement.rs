//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLOListElement;

    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLOListElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMEventTarget",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLOListElement {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLOListElement {}

#[cfg(all(
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLOListElement {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLOListElement {
        #[deprecated]
        #[method(compact)]
        pub unsafe fn compact(&self) -> bool;

        #[deprecated]
        #[method(setCompact:)]
        pub unsafe fn setCompact(&self, compact: bool);

        #[deprecated]
        #[method(start)]
        pub unsafe fn start(&self) -> c_int;

        #[deprecated]
        #[method(setStart:)]
        pub unsafe fn setStart(&self, start: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLOListElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLOListElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
