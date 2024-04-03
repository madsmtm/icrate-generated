//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// DOMNodeExtensions
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMNode {
        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;

        #[method_id(@__retain_semantics Other lineBoxRects)]
        pub unsafe fn lineBoxRects(&self) -> Option<Id<NSArray>>;
    }
);

extern_methods!(
    /// DOMElementAppKitExtensions
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMElement {
        #[cfg(feature = "objc2-app-kit")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;
    }
);

extern_methods!(
    /// DOMHTMLDocumentExtensions
    #[cfg(all(
        feature = "WebKit_DOMDocument",
        feature = "WebKit_DOMHTMLDocument",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLDocument {
        #[cfg(feature = "WebKit_DOMDocumentFragment")]
        #[method_id(@__retain_semantics Other createDocumentFragmentWithMarkupString:baseURL:)]
        pub unsafe fn createDocumentFragmentWithMarkupString_baseURL(
            &self,
            markup_string: Option<&NSString>,
            base_url: Option<&NSURL>,
        ) -> Option<Id<DOMDocumentFragment>>;

        #[cfg(feature = "WebKit_DOMDocumentFragment")]
        #[method_id(@__retain_semantics Other createDocumentFragmentWithText:)]
        pub unsafe fn createDocumentFragmentWithText(
            &self,
            text: Option<&NSString>,
        ) -> Option<Id<DOMDocumentFragment>>;
    }
);
