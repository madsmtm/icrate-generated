//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_ALLOW_KEYBOARD_INPUT: c_uint = 1;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMElement;

    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMElement {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "WebKit_DOMEventTarget",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMElement {}

#[cfg(all(
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMElement {}

#[cfg(all(
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMElement {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other tagName)]
        pub unsafe fn tagName(&self) -> Id<NSString>;

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[deprecated]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<DOMCSSStyleDeclaration>>;

        #[deprecated]
        #[method(offsetLeft)]
        pub unsafe fn offsetLeft(&self) -> c_int;

        #[deprecated]
        #[method(offsetTop)]
        pub unsafe fn offsetTop(&self) -> c_int;

        #[deprecated]
        #[method(offsetWidth)]
        pub unsafe fn offsetWidth(&self) -> c_int;

        #[deprecated]
        #[method(offsetHeight)]
        pub unsafe fn offsetHeight(&self) -> c_int;

        #[method(clientLeft)]
        pub unsafe fn clientLeft(&self) -> c_int;

        #[method(clientTop)]
        pub unsafe fn clientTop(&self) -> c_int;

        #[deprecated]
        #[method(clientWidth)]
        pub unsafe fn clientWidth(&self) -> c_int;

        #[deprecated]
        #[method(clientHeight)]
        pub unsafe fn clientHeight(&self) -> c_int;

        #[deprecated]
        #[method(scrollLeft)]
        pub unsafe fn scrollLeft(&self) -> c_int;

        #[deprecated]
        #[method(setScrollLeft:)]
        pub unsafe fn setScrollLeft(&self, scroll_left: c_int);

        #[deprecated]
        #[method(scrollTop)]
        pub unsafe fn scrollTop(&self) -> c_int;

        #[deprecated]
        #[method(setScrollTop:)]
        pub unsafe fn setScrollTop(&self, scroll_top: c_int);

        #[deprecated]
        #[method(scrollWidth)]
        pub unsafe fn scrollWidth(&self) -> c_int;

        #[deprecated]
        #[method(scrollHeight)]
        pub unsafe fn scrollHeight(&self) -> c_int;

        #[deprecated]
        #[method_id(@__retain_semantics Other offsetParent)]
        pub unsafe fn offsetParent(&self) -> Option<Id<DOMElement>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other innerHTML)]
        pub unsafe fn innerHTML(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setInnerHTML:)]
        pub unsafe fn setInnerHTML(&self, inner_html: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other outerHTML)]
        pub unsafe fn outerHTML(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setOuterHTML:)]
        pub unsafe fn setOuterHTML(&self, outer_html: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other className)]
        pub unsafe fn className(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setClassName:)]
        pub unsafe fn setClassName(&self, class_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other innerText)]
        pub unsafe fn innerText(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other previousElementSibling)]
        pub unsafe fn previousElementSibling(&self) -> Option<Id<DOMElement>>;

        #[method_id(@__retain_semantics Other nextElementSibling)]
        pub unsafe fn nextElementSibling(&self) -> Option<Id<DOMElement>>;

        #[method_id(@__retain_semantics Other firstElementChild)]
        pub unsafe fn firstElementChild(&self) -> Option<Id<DOMElement>>;

        #[method_id(@__retain_semantics Other lastElementChild)]
        pub unsafe fn lastElementChild(&self) -> Option<Id<DOMElement>>;

        #[method(childElementCount)]
        pub unsafe fn childElementCount(&self) -> c_uint;

        #[deprecated]
        #[method_id(@__retain_semantics Other getAttribute:)]
        pub unsafe fn getAttribute(&self, name: Option<&NSString>) -> Option<Id<NSString>>;

        #[method(setAttribute:value:)]
        pub unsafe fn setAttribute_value(&self, name: Option<&NSString>, value: Option<&NSString>);

        #[deprecated]
        #[method(removeAttribute:)]
        pub unsafe fn removeAttribute(&self, name: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMAttr")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getAttributeNode:)]
        pub unsafe fn getAttributeNode(&self, name: Option<&NSString>) -> Option<Id<DOMAttr>>;

        #[cfg(feature = "WebKit_DOMAttr")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setAttributeNode:)]
        pub unsafe fn setAttributeNode(&self, new_attr: Option<&DOMAttr>) -> Option<Id<DOMAttr>>;

        #[cfg(feature = "WebKit_DOMAttr")]
        #[deprecated]
        #[method_id(@__retain_semantics Other removeAttributeNode:)]
        pub unsafe fn removeAttributeNode(&self, old_attr: Option<&DOMAttr>)
            -> Option<Id<DOMAttr>>;

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getElementsByTagName:)]
        pub unsafe fn getElementsByTagName(
            &self,
            name: Option<&NSString>,
        ) -> Option<Id<DOMNodeList>>;

        #[method_id(@__retain_semantics Other getAttributeNS:localName:)]
        pub unsafe fn getAttributeNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[method(setAttributeNS:qualifiedName:value:)]
        pub unsafe fn setAttributeNS_qualifiedName_value(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            value: Option<&NSString>,
        );

        #[method(removeAttributeNS:localName:)]
        pub unsafe fn removeAttributeNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        );

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[method_id(@__retain_semantics Other getElementsByTagNameNS:localName:)]
        pub unsafe fn getElementsByTagNameNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNodeList>>;

        #[cfg(feature = "WebKit_DOMAttr")]
        #[method_id(@__retain_semantics Other getAttributeNodeNS:localName:)]
        pub unsafe fn getAttributeNodeNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMAttr>>;

        #[cfg(feature = "WebKit_DOMAttr")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setAttributeNodeNS:)]
        pub unsafe fn setAttributeNodeNS(&self, new_attr: Option<&DOMAttr>) -> Option<Id<DOMAttr>>;

        #[deprecated]
        #[method(hasAttribute:)]
        pub unsafe fn hasAttribute(&self, name: Option<&NSString>) -> bool;

        #[method(hasAttributeNS:localName:)]
        pub unsafe fn hasAttributeNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> bool;

        #[method(focus)]
        pub unsafe fn focus(&self);

        #[method(blur)]
        pub unsafe fn blur(&self);

        #[method(scrollIntoView:)]
        pub unsafe fn scrollIntoView(&self, align_with_top: bool);

        #[method(scrollIntoViewIfNeeded:)]
        pub unsafe fn scrollIntoViewIfNeeded(&self, center_if_needed: bool);

        #[method(scrollByLines:)]
        pub unsafe fn scrollByLines(&self, lines: c_int);

        #[method(scrollByPages:)]
        pub unsafe fn scrollByPages(&self, pages: c_int);

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[method_id(@__retain_semantics Other getElementsByClassName:)]
        pub unsafe fn getElementsByClassName(
            &self,
            name: Option<&NSString>,
        ) -> Option<Id<DOMNodeList>>;

        #[method(webkitRequestFullScreen:)]
        pub unsafe fn webkitRequestFullScreen(&self, flags: c_ushort);

        #[method_id(@__retain_semantics Other querySelector:)]
        pub unsafe fn querySelector(&self, selectors: Option<&NSString>) -> Option<Id<DOMElement>>;

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[method_id(@__retain_semantics Other querySelectorAll:)]
        pub unsafe fn querySelectorAll(
            &self,
            selectors: Option<&NSString>,
        ) -> Option<Id<DOMNodeList>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMElementDeprecated
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMElement {
        #[deprecated]
        #[method(setAttribute::)]
        pub unsafe fn setAttribute(&self, name: Option<&NSString>, value: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other getAttributeNS::)]
        pub unsafe fn getAttributeNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(setAttributeNS:::)]
        pub unsafe fn setAttributeNS(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            value: Option<&NSString>,
        );

        #[deprecated]
        #[method(removeAttributeNS::)]
        pub unsafe fn removeAttributeNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        );

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getElementsByTagNameNS::)]
        pub unsafe fn getElementsByTagNameNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNodeList>>;

        #[cfg(feature = "WebKit_DOMAttr")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getAttributeNodeNS::)]
        pub unsafe fn getAttributeNodeNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMAttr>>;

        #[deprecated]
        #[method(hasAttributeNS::)]
        pub unsafe fn hasAttributeNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> bool;
    }
);
