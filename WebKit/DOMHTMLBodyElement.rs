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
    pub struct DOMHTMLBodyElement;

    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLBodyElement {
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
unsafe impl DOMEventTarget for DOMHTMLBodyElement {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLBodyElement {}

#[cfg(all(
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLBodyElement {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLBodyElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other aLink)]
        pub unsafe fn aLink(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setALink:)]
        pub unsafe fn setALink(&self, a_link: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other background)]
        pub unsafe fn background(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setBackground:)]
        pub unsafe fn setBackground(&self, background: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other link)]
        pub unsafe fn link(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLink:)]
        pub unsafe fn setLink(&self, link: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other vLink)]
        pub unsafe fn vLink(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setVLink:)]
        pub unsafe fn setVLink(&self, v_link: Option<&NSString>);
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
    unsafe impl DOMHTMLBodyElement {
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
    unsafe impl DOMHTMLBodyElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
