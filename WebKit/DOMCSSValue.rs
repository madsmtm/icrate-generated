//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[deprecated]
pub const DOM_CSS_INHERIT: c_uint = 0;
#[deprecated]
pub const DOM_CSS_PRIMITIVE_VALUE: c_uint = 1;
#[deprecated]
pub const DOM_CSS_VALUE_LIST: c_uint = 2;
#[deprecated]
pub const DOM_CSS_CUSTOM: c_uint = 3;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    #[deprecated]
    pub struct DOMCSSValue;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMCSSValue {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMCSSValue {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMCSSValue {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cssText)]
        pub unsafe fn cssText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCssText:)]
        pub unsafe fn setCssText(&self, css_text: Option<&NSString>);

        #[deprecated]
        #[method(cssValueType)]
        pub unsafe fn cssValueType(&self) -> c_ushort;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
