//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// WebDOMNodeOperations
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMNode {
        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;
    }
);

extern_methods!(
    /// WebDOMDocumentOperations
    #[cfg(all(
        feature = "WebKit_DOMDocument",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMDocument {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webFrame)]
        pub unsafe fn webFrame(&self) -> Option<Id<WebFrame>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other URLWithAttributeString:)]
        pub unsafe fn URLWithAttributeString(&self, string: Option<&NSString>)
            -> Option<Id<NSURL>>;
    }
);

extern_methods!(
    /// WebDOMRangeOperations
    #[cfg(all(
        feature = "WebKit_DOMObject",
        feature = "WebKit_DOMRange",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMRange {
        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other markupString)]
        pub unsafe fn markupString(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// WebDOMHTMLFrameElementOperations
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMHTMLFrameElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLFrameElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLIFrameElementOperations
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMHTMLIFrameElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLIFrameElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLObjectElementOperations
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMHTMLObjectElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLObjectElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);
